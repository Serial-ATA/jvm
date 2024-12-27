mod exceptions;
pub mod frame;
use frame::stack::{FrameStack, StackFrame};
#[allow(non_snake_case)]
pub mod java_lang_Thread;
pub mod pool;

use crate::classpath::classloader::ClassLoader;
use crate::interpreter::Interpreter;
use crate::java_call;
use crate::native::jni::invocation_api::new_env;
use crate::objects::class_instance::ClassInstance;
use crate::objects::method::Method;
use crate::objects::reference::{ClassInstanceRef, Reference};
use crate::stack::local_stack::LocalStack;
use crate::string_interner::StringInterner;
use crate::thread::frame::native::NativeFrame;
use crate::thread::frame::Frame;
use crate::thread::pool::ThreadPool;

use std::cell::{SyncUnsafeCell, UnsafeCell};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

use classfile::accessflags::MethodAccessFlags;
use instructions::{Operand, StackLike};
use jni::env::JniEnv;
use symbols::sym;

#[thread_local]
static CURRENT_JAVA_THREAD: SyncUnsafeCell<Option<&'static JavaThread>> = SyncUnsafeCell::new(None);

pub struct JVMOptions {
	pub dry_run: bool,
	pub system_properties: Option<Vec<String>>,
	pub showversion: bool,
	pub show_version: bool,
}

/// A builder for a `JavaThread`
///
/// This is the only way to construct a `JavaThread`, and is responsible for spawning the associated
/// OS thread, if applicable.
#[derive(Default)]
pub struct JavaThreadBuilder {
	obj: Option<Reference>,
	entry_point: Option<Box<dyn Fn(&JavaThread) + Send + Sync + 'static>>,
	stack_size: usize,
}

impl JavaThreadBuilder {
	/// Create a new `JavaThreadBuilder`
	///
	/// This is equivalent to [`Self::default`].
	pub fn new() -> JavaThreadBuilder {
		Self {
			obj: None,
			entry_point: None,
			stack_size: 0, // TODO: Default -Xss
		}
	}

	// TODO: Unsafe? The object is not verified to be of the correct class.
	/// Set the `java.lang.Thread` associated with this `JavaThread`
	///
	/// It is up to the caller to verify that `obj` is *actually* of the correct type.
	pub fn obj(mut self, obj: Reference) -> Self {
		self.obj = Some(obj);
		self
	}

	/// Set the entrypoint of this `JavaThread`
	///
	/// Setting this will spawn an OS thread to run `entry`. This is really only used with [`JavaThread::default_entry_point`],
	/// which calls `java.lang.Thread#run` on the associated [`obj`].
	///
	/// [`obj`]: Self::obj
	pub fn entry_point(mut self, entry: impl Fn(&JavaThread) + Send + Sync + 'static) -> Self {
		self.entry_point = Some(Box::new(entry));
		self
	}

	/// Set the stack size of the associated OS thread
	///
	/// This will have no effect if there is no [`entry_point`] set.
	///
	/// [`entry_point`]: Self::entry_point
	pub fn stack_size(mut self, size: usize) -> Self {
		self.stack_size = size;
		self
	}

	/// Construct the `JavaThread`
	///
	/// This will also spawn an OS thread if applicable.
	///
	/// The return type of this depends on whether an OS thread has been spawned. If no [`entry_point`]
	/// was set, it is safe to unwrap it as [`MaybeArc::Not`].
	///
	/// [`entry_point`]: Self::entry_point
	pub fn finish(self) -> &'static JavaThread {
		let thread = JavaThread {
			env: unsafe { JniEnv::from_raw(new_env()) },
			obj: UnsafeCell::new(self.obj),
			os_thread: UnsafeCell::new(None),

			pc: AtomicIsize::new(0),
			frame_stack: FrameStack::new(),
			remaining_operand: UnsafeCell::new(None),
		};

		let thread = ThreadPool::push(thread);
		if let Some(entry_point) = self.entry_point {
			let mut os_thread = thread::Builder::new();
			if self.stack_size > 0 {
				os_thread = os_thread.stack_size(self.stack_size);
			}

			let os_thread_ptr = thread.os_thread.get();

			// TODO: Error handling
			let handle = os_thread.spawn(move || entry_point(thread)).unwrap();
			unsafe {
				*os_thread_ptr = Some(handle);
			}
		}

		thread
	}
}

#[repr(C)]
pub struct JavaThread {
	env: JniEnv,
	obj: UnsafeCell<Option<Reference>>,
	os_thread: UnsafeCell<Option<JoinHandle<()>>>,

	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.5.1
	// Each Java Virtual Machine thread has its own pc (program counter) register [...]
	// the pc register contains the address of the Java Virtual Machine instruction currently being executed
	pub pc: AtomicIsize,
	frame_stack: FrameStack,

	remaining_operand: UnsafeCell<Option<Operand<Reference>>>,
}

unsafe impl Sync for JavaThread {}
unsafe impl Send for JavaThread {}

impl PartialEq for JavaThread {
	fn eq(&self, other: &Self) -> bool {
		self.env() == other.env()
	}
}

/// Global accessors
impl JavaThread {
	/// Get the `JavaThread` associated with `env`
	///
	/// # Safety
	///
	/// The caller must ensure that `env` is a pointer obtained from [`Self::env()`], and that it is
	/// valid for the current thread.
	pub unsafe fn for_env(env: *const JniEnv) -> *mut JavaThread {
		unsafe {
			let ptr = env.sub(core::mem::offset_of!(JavaThread, env));
			ptr.cast::<Self>() as _
		}
	}

	/// Get a pointer to the current `JavaThread` for this thread
	pub fn current_ptr() -> *const JavaThread {
		let current = CURRENT_JAVA_THREAD.get();

		// SAFETY: The thread is an `Option`, so it's always initialized with *something*
		let opt = unsafe { &*current };
		match *opt {
			None => std::ptr::null(),
			Some(thread) => thread,
		}
	}

	/// Get the current `JavaThread` for this thread
	///
	/// # Panics
	///
	/// This will panic if there is no `JavaThread` available, which is only possible on an
	/// uninitialized thread.
	pub fn current() -> &'static JavaThread {
		Self::current_opt().expect("current JavaThread should be available")
	}

	/// Get the current `JavaThread` for this thread
	///
	/// This will return `None` if there is no `JavaThread` available.
	pub fn current_opt() -> Option<&'static JavaThread> {
		let current = CURRENT_JAVA_THREAD.get();

		// SAFETY: The thread is an `Option`, so it's always initialized with *something*
		let opt = unsafe { &*current };
		*opt
	}

	/// Sets the current Java [`JavaThread`]
	///
	/// # Panics
	///
	/// This will panic if there is already a current thread set
	pub unsafe fn set_current_thread(thread: &'static JavaThread) {
		let current = CURRENT_JAVA_THREAD.get();

		// SAFETY: The thread is an `Option`, so it's always initialized with *something*
		let opt = unsafe { &*current };
		assert!(
			opt.is_none(),
			"attempting to overwrite an existing JavaThread"
		);

		unsafe {
			*current = Some(thread);
		}
	}
}

/// Value for the `java.lang.Thread$FieldHolder#status` field
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum ThreadStatus {
	New = 0,
	Runnable = 1,
	Sleeping = 2,
	InObjectWait = 3,
	InObjectWaitTimed = 4,
	Parked = 5,
	ParkedTimed = 6,
	BlockedOnMonitorEnter = 7,
	Terminated = 8,
}

// Actions for the related `java.lang.Thread` instance
impl JavaThread {
	/// The default entrypoint for a `java.lang.Thread`
	///
	/// This simply calls `java.lang.Thread#run` with the [`obj`] associated with this `JavaThread`.
	///
	/// [`obj`]: JavaThread::obj
	pub fn default_entry_point(&self) {
		let obj = self.obj().expect("entrypoint should exist");

		let thread_class = ClassLoader::Bootstrap.load(sym!(java_lang_Thread)).unwrap();
		let run_method = thread_class
			.resolve_method_step_two(sym!(run_name), sym!(void_method_signature))
			.unwrap();

		let ret = java_call!(self, run_method, Operand::Reference(obj));
		assert!(ret.is_none());
	}

	/// Allocates a new `java.lang.Thread` for this `JavaThread`
	///
	/// This is called from the JNI `AttachCurrentThread`/`AttachCurrentThreadAsDaemon`.
	pub fn attach_thread_obj(&self, name: Option<&str>, thread_group: Reference, daemon: bool) {
		assert!(self.obj().is_none());

		let thread_class = crate::globals::classes::java_lang_Thread();
		let thread_instance = ClassInstance::new(thread_class);

		if let Some(name) = name {
			let string_object = StringInterner::intern_string(name.to_string());
			let init_method = thread_class
				.vtable()
				.find(
					sym!(object_initializer_name),
					sym!(ThreadGroup_String_void_signature),
					MethodAccessFlags::NONE,
				)
				.unwrap();

			java_call!(
				self,
				init_method,
				Operand::Reference(thread_group),
				Operand::Reference(Reference::class(string_object)),
			);
		} else {
			let init_method = thread_class
				.vtable()
				.find(
					sym!(object_initializer_name),
					sym!(ThreadGroup_Runnable_void_signature),
					MethodAccessFlags::NONE,
				)
				.unwrap();

			java_call!(
				self,
				init_method,
				Operand::Reference(thread_group),
				Operand::Reference(Reference::null())
			);
		}

		if daemon {
			unimplemented!("jni::AttachCurrentThreadAsDaemon");
		}

		self.set_obj(Reference::class(thread_instance));
	}

	pub fn init_obj(&self, thread_group: Reference) {
		let thread_class = crate::globals::classes::java_lang_Thread();
		let thread_instance = ClassInstance::new(thread_class);

		// Set the obj early, since the java.lang.Thread constructor calls Thread#current.
		self.set_obj(Reference::class(ClassInstanceRef::clone(&thread_instance)));
		let obj = self.obj().unwrap();

		java_lang_Thread::set_eetop(obj.clone(), JavaThread::current_ptr() as jni::sys::jlong);

		let init_method = thread_class
			.vtable()
			.find(
				sym!(object_initializer_name),
				sym!(ThreadGroup_String_void_signature),
				MethodAccessFlags::NONE,
			)
			.expect("method should exist");

		let thread_name = StringInterner::intern_str("main");

		java_call!(
			self,
			init_method,
			Operand::Reference(Reference::class(ClassInstanceRef::clone(&thread_instance))),
			Operand::Reference(thread_group),
			Operand::Reference(Reference::class(thread_name)),
		);

		java_lang_Thread::holder::set_thread_status(obj, ThreadStatus::Runnable);
	}

	pub fn set_obj(&self, obj: Reference) {
		let obj_ptr = self.obj.get();

		// SAFETY: The object is an `Option`, so it's always initialized with *something*
		let obj_opt = unsafe { &*obj_ptr };
		assert!(obj_opt.is_none());

		unsafe {
			*obj_ptr = Some(obj);
		}
	}

	pub fn obj(&self) -> Option<Reference> {
		let obj_ptr = self.obj.get();

		// SAFETY: The object is an `Option`, so it's always initialized with *something*
		let obj_opt = unsafe { &*obj_ptr };
		obj_opt.as_ref().map(Reference::clone)
	}
}

impl JavaThread {
	/// Get a pointer to the associated [`JniEnv`]
	pub fn env(&self) -> NonNull<JniEnv> {
		unsafe {
			NonNull::new_unchecked(
				std::ptr::from_ref(self).add(core::mem::offset_of!(JavaThread, env)) as _,
			)
		}
	}

	/// Get the frame stack for this thread
	pub fn frame_stack(&self) -> &FrameStack {
		&self.frame_stack
	}

	fn set_remaining_operand(&self, operand: Option<Operand<Reference>>) {
		let remaining_operand_ptr = self.remaining_operand.get();

		let remaining_operand_opt = unsafe { &*remaining_operand_ptr };
		assert!(remaining_operand_opt.is_none());

		unsafe {
			*remaining_operand_ptr = operand;
		}
	}

	fn take_remaining_operand(&self) -> Option<Operand<Reference>> {
		let remaining_operand_ptr = self.remaining_operand.get();
		let remaining_operand_opt = unsafe { &mut *remaining_operand_ptr };
		remaining_operand_opt.take()
	}

	/// Manually invoke a method and get its return value
	///
	/// This will run the method on the current thread, separate from normal execution. This is used
	/// by [`java_call!`](crate::java_call) to allow us to manually invoke methods in the runtime.
	pub fn invoke_method_scoped(
		&self,
		method: &'static Method,
		locals: LocalStack,
	) -> Option<Operand<Reference>> {
		if method.is_native() {
			unimplemented!("Manual invocation of native methods");
		}

		self.stash_and_reset_pc();

		self.frame_stack.push(StackFrame::Fake);
		self.invoke_method_with_local_stack(method, locals);
		self.run();

		let ret = self.take_remaining_operand();
		// Will pop the dummy frame for us
		self.drop_to_previous_frame(None);

		ret
	}

	pub fn invoke_method_with_local_stack(&self, method: &'static Method, locals: LocalStack) {
		if method.is_native() {
			self.invoke_native(method, locals);
			tracing::debug!(target: "JavaThread", "Native method finished");
			return;
		}

		let max_stack = method.code.max_stack;

		let frame = Frame::new(self, locals, max_stack, method);

		self.stash_and_reset_pc();
		self.frame_stack.push(StackFrame::Real(frame));
	}

	fn invoke_native(&self, method: &'static Method, locals: LocalStack) {
		// Try to lookup and set the method prior to calling
		crate::native::lookup::lookup_native_method(method, self);

		let fn_ptr = super::native::lookup_method(method);

		// See comments on `NativeFrame`
		self.frame_stack
			.push(StackFrame::Native(NativeFrame { method }));

		let ret = fn_ptr(self.env(), locals);
		assert!(
			self.frame_stack.pop_native().is_some(),
			"native frame consumed"
		);

		// Push the return value onto the previous frame's stack
		if let Some(ret) = ret {
			self.frame_stack.current().unwrap().stack_mut().push_op(ret);
		}

		return;
	}

	fn stash_and_reset_pc(&self) {
		if let Some(current_frame) = self.frame_stack.current() {
			current_frame.stash_pc()
		}

		self.pc.store(0, Ordering::Relaxed);
	}

	/// Return from the current frame and drop to the previous one
	pub fn drop_to_previous_frame(&self, return_value: Option<Operand<Reference>>) {
		let _ = self.frame_stack.pop();

		let Some(current_frame) = self.frame_stack.current() else {
			// If there's no current frame it either means:
			//
			// 1. We've reached the end of a manual method invocation, and need to pop the dummy frame
			// 2. We've reached the end of the program
			//
			// Either way, the remaining operand ends up in the hands of the caller.
			self.set_remaining_operand(return_value);

			return;
		};

		tracing::debug!(target: "JavaThread", "Dropping back to frame for method `{:?}`", current_frame.method());

		// Restore the pc of the frame
		let previous_pc = current_frame.stashed_pc();
		self.pc.store(previous_pc, Ordering::Relaxed);

		// Push the return value of the previous frame if there is one
		if let Some(return_value) = return_value {
			current_frame.stack_mut().push_op(return_value);
		}
	}

	pub fn run(&self) {
		while let Some(current_frame) = self.frame_stack.current() {
			Interpreter::instruction(current_frame);
		}
	}

	/// Throw an exception on this thread
	///
	/// `object_ref` can be [`Reference::Null`], in which case a `NullPointerException` is thrown
	///
	/// # Panics
	///
	/// This will panic if `object_ref` is non-null, but not a subclass of `java/lang/Throwable`.
	/// This should never occur post-verification.
	pub fn throw_exception(&self, object_ref: Reference) {
		exceptions::throw(self, object_ref);
	}
}