pub mod exceptions;
pub mod frame;
use frame::stack::{FrameStack, StackFrame};
mod builder;
pub use builder::JavaThreadBuilder;
mod hash;
pub mod pool;
pub mod stack;

use crate::classes::java::lang::Thread::ThreadStatus;
use crate::interpreter::Interpreter;
use crate::native::java::lang::String::StringInterner;
use crate::native::jni::IntoJni;
use crate::native::jni::invocation_api::new_env;
use crate::native::method::NativeMethodPtr;
use crate::objects::instance::class::{ClassInstance, ClassInstanceRef};
use crate::objects::instance::object::Object;
use crate::objects::method::Method;
use crate::objects::reference::Reference;
use crate::stack::local_stack::LocalStack;
use crate::symbols::sym;
use crate::thread::exceptions::{Exception, ExceptionKind, Throws};
use crate::thread::frame::Frame;
use crate::thread::frame::native::NativeFrame;
use crate::thread::stack::{ThreadStack, ThreadStackHandle};
use crate::{classes, globals, java_call};

use std::cell::{Cell, SyncUnsafeCell, UnsafeCell};
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicBool, AtomicIsize, AtomicU8, Ordering};
use std::thread::JoinHandle;

use crate::objects::constant_pool::cp_types::{InvokeDynamicEntry, MethodEntry};
use classfile::FieldType;
use classfile::accessflags::MethodAccessFlags;
use common::int_types::u1;
use instructions::{Operand, StackLike};
use jni::env::JniEnv;
use jni::sys::JNIEnv;

#[thread_local]
static CURRENT_JAVA_THREAD: Cell<Option<&'static JavaThread>> = Cell::new(None);

/// The state of a [`JavaThread`]
///
/// This is distinct from [`ThreadStatus`], which is the status on the `java.lang.Thread` instance.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JavaThreadState {
	/// Executing Java code
	Running = 0,
	/// Unwinding the frame stack to find an exception handler
	///
	/// See [`JavaThread::handle_pending_exception()`]
	Unwinding = 1,
	/// In the process of exiting, see [`JavaThread::exit()`]
	Exiting = 2,
}

#[derive(Copy, Clone, Debug)]
pub enum ControlFlow {
	Continue,
	Break,
	ExceptionThrown,
}

#[repr(C)]
pub struct JavaThread {
	env: JNIEnv,
	obj: UnsafeCell<Option<Reference>>,
	os_thread: UnsafeCell<Option<JoinHandle<()>>>,

	hash_state: Cell<hash::HashState>,

	control_flow: UnsafeCell<ControlFlow>,

	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.5.1
	// Each Java Virtual Machine thread has its own pc (program counter) register [...]
	// the pc register contains the address of the Java Virtual Machine instruction currently being executed
	pub pc: AtomicIsize,
	operand_stack: UnsafeCell<ThreadStack>,
	frame_stack: FrameStack,

	remaining_operand: UnsafeCell<Option<Operand<Reference>>>,

	pending_exception: UnsafeCell<Option<Reference>>,
	state: AtomicU8,

	/// Used in tests to prevent this thread from actually running any Java code
	#[cfg(test)]
	pub sealed: AtomicBool,
}

// TODO: This isn't *actually* safe, but the way it's currently used is
unsafe impl Sync for JavaThread {}
unsafe impl Send for JavaThread {}

impl PartialEq for JavaThread {
	fn eq(&self, other: &Self) -> bool {
		self.env == other.env
	}
}

/// Global accessors
impl JavaThread {
	// Used in `JavaThreadBuilder::finish`
	fn new(obj: Option<Reference>, stack_size: usize) -> Throws<Self> {
		let seed = 1;
		let operand_stack = UnsafeCell::new(ThreadStack::new(stack_size)?);

		Throws::Ok(JavaThread {
			env: unsafe { new_env() },
			obj: UnsafeCell::new(obj),
			os_thread: UnsafeCell::new(None),

			hash_state: Cell::new(hash::HashState::new(seed)),

			control_flow: UnsafeCell::new(ControlFlow::Continue),

			pc: AtomicIsize::new(0),
			operand_stack,
			frame_stack: FrameStack::new(),
			remaining_operand: UnsafeCell::new(None),

			pending_exception: UnsafeCell::new(None),
			state: AtomicU8::new(JavaThreadState::Running as u8),

			#[cfg(test)]
			sealed: AtomicBool::new(false),
		})
	}

	/// Get the `JavaThread` associated with `env`
	///
	/// # Safety
	///
	/// The caller must ensure that `env` is a pointer obtained from [`Self::env()`], and that it is
	/// valid for the current thread.
	pub unsafe fn for_env(env: *const JNIEnv) -> *mut JavaThread {
		unsafe {
			let ptr = env.sub(core::mem::offset_of!(JavaThread, env));
			ptr.cast::<Self>().cast_mut()
		}
	}

	/// Get a pointer to the current `JavaThread` for this thread
	pub fn current_ptr() -> *const JavaThread {
		match Self::current_opt() {
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
	#[cfg(not(test))] // Tests have a custom impl, see src/test_utils/thread.rs
	pub fn current() -> &'static JavaThread {
		Self::current_opt().expect("current JavaThread should be available")
	}

	/// Get the current `JavaThread` for this thread
	///
	/// This will return `None` if there is no `JavaThread` available.
	pub fn current_opt() -> Option<&'static JavaThread> {
		CURRENT_JAVA_THREAD.get()
	}

	/// Sets the current Java [`JavaThread`]
	///
	/// # Panics
	///
	/// This will panic if there is already a current thread set
	pub unsafe fn set_current_thread(thread: &'static JavaThread) {
		CURRENT_JAVA_THREAD.update(|current| {
			assert!(
				current.is_none(),
				"attempting to overwrite an existing JavaThread"
			);
			Some(thread)
		});
	}

	pub unsafe fn unset_current_thread() {
		CURRENT_JAVA_THREAD.update(|current| {
			assert!(
				current.is_some(),
				"JavaThread already unset (how was this called?)"
			);
			None
		});
	}
}

// Actions for the related `java.lang.Thread` instance
impl JavaThread {
	/// Spawn the thread and begin executing its Java code
	///
	/// This is used in [`JavaThreadBuilder::finish()`], which is called from the
	/// JNI `AttachCurrentThread`. The `main` thread does NOT ever call this.
	pub(crate) fn start(&'static self) {
		let os_thread_ptr = self.os_thread.get();
		let handle = std::thread::spawn(move || {
			// Call `java.lang.Thread#run` with the obj associated with this `JavaThread`.
			let obj = self.obj().expect("obj should exist");

			let thread_class = crate::globals::classes::java_lang_Thread();
			let run_method = thread_class
				.resolve_method_step_two(sym!(run_name), sym!(void_method_signature))
				.unwrap();

			java_call!(self, run_method, Operand::Reference(obj));
		});
		unsafe {
			*os_thread_ptr = Some(handle);
		}
	}

	/// Allocates a new `java.lang.Thread` for this `JavaThread`
	///
	/// This is called from the JNI `AttachCurrentThread`/`AttachCurrentThreadAsDaemon`.
	pub fn attach_thread_obj(
		&'static self,
		name: Option<&str>,
		thread_group: Reference,
		daemon: bool,
	) {
		let thread_class = crate::globals::classes::java_lang_Thread();
		let thread_instance = Reference::class(ClassInstance::new(thread_class));

		// Set the obj early, since the java.lang.Thread constructor calls Thread#current.
		self.set_obj(thread_instance);
		let obj = self.obj().unwrap();

		classes::java::lang::Thread::set_eetop(
			obj.extract_class(),
			JavaThread::current_ptr() as jni::sys::jlong,
		);

		if let Some(name) = name {
			let name_obj = StringInterner::intern(name);
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
				Operand::Reference(thread_instance),
				Operand::Reference(thread_group),
				Operand::Reference(Reference::class(name_obj)),
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
				Operand::Reference(thread_instance),
				Operand::Reference(thread_group),
				Operand::Reference(Reference::null())
			);
		}

		if daemon {
			unimplemented!("jni::AttachCurrentThreadAsDaemon");
		}

		let holder = classes::java::lang::Thread::holder(obj.extract_class());
		classes::java::lang::Thread::holder::set_threadStatus(
			holder.extract_class(),
			ThreadStatus::Runnable,
		);
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
		*obj_opt
	}

	pub fn name(&self) -> String {
		let Some(obj) = self.obj() else {
			return String::from("Unknown thread");
		};

		let name = classes::java::lang::Thread::name(obj.extract_class());
		if name.is_null() {
			return String::from("<un-named>");
		}

		classes::java::lang::String::extract(name.extract_class())
	}
}

impl JavaThread {
	/// Get a pointer to the associated [`JNIEnv`]
	pub fn env(&'static self) -> JniEnv {
		unsafe {
			JniEnv::from_raw(
				std::ptr::from_ref(self).add(core::mem::offset_of!(JavaThread, env)) as _,
			)
		}
	}

	/// Get the frame stack for this thread
	#[inline]
	pub fn frame_stack(&self) -> &FrameStack {
		&self.frame_stack
	}

	/// Get the operand stack for this thread
	#[inline]
	pub fn stack(&self) -> ThreadStackHandle {
		ThreadStackHandle::new(self.operand_stack.get())
	}

	/// Get the current state of this thread
	pub fn state(&self) -> JavaThreadState {
		// SAFETY: The state is only ever set by `set_state`, which restrict the values to valid
		//         variants
		unsafe { std::mem::transmute(self.state.load(Ordering::Relaxed)) }
	}

	/// Set the state of this thread
	pub fn set_state(&self, state: JavaThreadState) {
		self.state.store(state as u8, Ordering::Relaxed);
	}

	pub fn set_remaining_operand(&self, operand: Option<Operand<Reference>>) {
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
		&'static self,
		method: &'static Method,
	) -> Option<Operand<Reference>> {
		#[cfg(test)]
		{
			self.assert_not_sealed();
		}

		if method.is_native() {
			unimplemented!("Manual invocation of native methods");
		}

		self.stash_and_reset_pc();

		self.frame_stack.push(StackFrame::Fake);
		self.invoke_method(method);

		loop {
			match self.control_flow() {
				ControlFlow::Continue => {
					if let Some(current_frame) = self.frame_stack.current() {
						Interpreter::instruction(current_frame);
						continue;
					}

					// Thread finished execution normally
					break;
				},
				ControlFlow::ExceptionThrown => self.on_exception(),
				ControlFlow::Break => {
					break;
				},
			}
		}

		let ret = self.take_remaining_operand();
		// Will pop the dummy frame for us
		self.drop_to_previous_frame(None, true);

		if self.frame_stack.current().is_none() {
			// End of invocation
			self.stash_and_reset_pc();
		}

		ret
	}

	pub fn invoke_method(&'static self, method: &'static Method) {
		#[cfg(test)]
		{
			self.assert_not_sealed();
		}

		if method.is_abstract() {
			Exception::new(ExceptionKind::AbstractMethodError).throw(self);
			return;
		}

		if method.is_native() {
			self.invoke_native(method);
			return;
		}

		self.stash_and_reset_pc();
		match Frame::new(self, method) {
			Throws::Ok(frame) => {
				self.frame_stack.push(StackFrame::Real(frame));
			},
			Throws::Exception(exception) => {
				exception.throw(self);
			},
		}
	}

	fn invoke_native(&'static self, method: &'static Method) {
		// Try to lookup and set the method prior to calling
		let fn_ptr;
		match crate::native::lookup::lookup_native_method(method, self) {
			Throws::Ok(ptr) => fn_ptr = ptr,
			Throws::Exception(e) => {
				e.throw(self);
				return;
			},
		}

		// TODO: remove LocalStack entirely
		// + 1 for receiver
		let parameter_count =
			method.parameter_count() as usize + if method.is_static() { 0 } else { 1 };

		let params = self.stack().popn(parameter_count);
		let locals = unsafe { LocalStack::new_with_args(params, method.parameter_stack_size()) };

		self.stash_and_reset_pc();

		// See comments on `NativeFrame`
		self.frame_stack
			.push(StackFrame::Native(NativeFrame { method }));

		let ret;
		match fn_ptr {
			NativeMethodPtr::StaticInternal(func) => {
				assert!(method.is_static());
				ret = func(self.env(), method.class(), locals);
			},
			NativeMethodPtr::NonStaticInternal(func) => {
				assert!(!method.is_static());
				ret = func(self.env(), locals);
			},
			#[cfg(feature = "libffi")]
			NativeMethodPtr::External(func) => {
				use jni::sys::{
					jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jobject, jshort,
				};
				use libffi::low::CodePtr;
				use libffi::middle::Arg;

				let env = self.env().raw();
				let target_class = method.class().into_jni();

				let mut locals = locals.iter();

				// To keep the receiver live
				let mut this = MaybeUninit::uninit();

				let receiver;
				if method.is_static() {
					receiver = Arg::new(&target_class);
				} else {
					let this_ref = locals
						.next()
						.expect("should have a receiver")
						.expect_reference();
					this.write(this_ref.raw_tagged());
					receiver = Arg::new(&this);
				}

				let cfi = method.prepare_cfi(&env, receiver, &mut locals);
				ret = unsafe {
					match method.descriptor.return_type {
						FieldType::Byte => Some(Operand::Int(
							cfi.cfi
								.call::<jbyte>(CodePtr::from_ptr(func), &*cfi.args)
								.into(),
						)),
						FieldType::Character => Some(Operand::Int(
							cfi.cfi
								.call::<jchar>(CodePtr::from_ptr(func), &*cfi.args)
								.into(),
						)),
						FieldType::Double => Some(Operand::Double(
							cfi.cfi
								.call::<jdouble>(CodePtr::from_ptr(func), &*cfi.args)
								.into(),
						)),
						FieldType::Float => Some(Operand::Float(
							cfi.cfi
								.call::<jfloat>(CodePtr::from_ptr(func), &*cfi.args)
								.into(),
						)),
						FieldType::Integer => Some(Operand::Int(
							cfi.cfi
								.call::<jint>(CodePtr::from_ptr(func), &*cfi.args)
								.into(),
						)),
						FieldType::Long => Some(Operand::Long(
							cfi.cfi
								.call::<jlong>(CodePtr::from_ptr(func), &*cfi.args)
								.into(),
						)),
						FieldType::Short => Some(Operand::Int(
							cfi.cfi
								.call::<jshort>(CodePtr::from_ptr(func), &*cfi.args)
								.into(),
						)),
						FieldType::Boolean => Some(Operand::Int(
							cfi.cfi
								.call::<jboolean>(CodePtr::from_ptr(func), &*cfi.args)
								.into(),
						)),
						FieldType::Void => {
							cfi.cfi.call::<()>(CodePtr::from_ptr(func), &*cfi.args);
							None
						},
						FieldType::Object(_) | FieldType::Array(_) => {
							Some(Operand::Reference(Reference::from_raw(
								cfi.cfi
									.call::<jobject>(CodePtr::from_ptr(func), &*cfi.args)
									.cast(),
							)))
						},
					}
				};
			},
		}

		// There's a chance that the native frame was consumed while handling an exception, otherwise
		// it should always be present.
		let popped_native_frame = self.frame_stack.pop_native().is_some();

		// Exception from native code, nothing left to do
		if self.has_pending_exception() {
			return;
		}

		assert!(popped_native_frame, "native frame consumed",);

		self.drop_to_previous_frame(ret, false);
	}

	fn stash_and_reset_pc(&self) {
		if let Some(current_frame) = self.frame_stack.current() {
			current_frame.stash()
		}

		self.pc.store(0, Ordering::Relaxed);
	}

	/// Return from the current frame and drop to the previous one
	pub fn drop_to_previous_frame(&self, return_value: Option<Operand<Reference>>, do_pop: bool) {
		if do_pop {
			let _ = self.frame_stack.pop();
		}

		match self.frame_stack.current() {
			Some(current_frame) => {
				unsafe { current_frame.apply_stash() }

				// Push the return value of the previous frame if there is one
				if let Some(return_value) = return_value {
					current_frame.push_op(return_value);
				}
			},
			// If there's no current frame, it either means:
			//
			// 1. We've reached the end of a manual method invocation and need to pop the dummy frame
			// 2. We've reached the end of the program
			//
			// Either way, the remaining operand ends up in the hands of the caller.
			None => {
				self.set_remaining_operand(return_value);
				return;
			},
		}
	}

	pub fn control_flow(&self) -> ControlFlow {
		unsafe { *self.control_flow.get() }
	}

	pub fn set_control_flow(&self, control_flow: ControlFlow) {
		unsafe { *self.control_flow.get() = control_flow }
	}

	pub fn exit(&'static self, destroy_vm: bool) {
		if destroy_vm {
			// Nothing special to do in the case of VM destruction for now
			return;
		}

		self.set_state(JavaThreadState::Exiting);

		let obj = self.obj().expect("thread object should exist");

		if let Some(exception) = self.take_pending_exception() {
			let dispatch_uncaught_exception_method = globals::classes::java_lang_Thread()
				.resolve_method(
					sym!(dispatchUncaughtException),
					sym!(Throwable_void_signature),
				)
				.expect("dispatchUncaughtException method should exist");

			java_call!(
				self,
				dispatch_uncaught_exception_method,
				Operand::Reference(obj),
				Operand::Reference(exception)
			);

			if self.has_pending_exception() {
				let exception = self.take_pending_exception().unwrap();
				eprintln!(
					"Exception: {} thrown from the UncaughtExceptionHandler in thread \"{}\"",
					exception.extract_target_class().name(),
					self.name()
				);
			}
		}

		let exit_method = globals::classes::java_lang_Thread()
			.resolve_method(sym!(exit_name), sym!(void_method_signature))
			.expect("exit method should exist");
		let _result = java_call!(&self, exit_method, Operand::Reference(obj));

		let holder = classes::java::lang::Thread::holder(obj.extract_class());
		classes::java::lang::Thread::holder::set_threadStatus(
			holder.extract_class(),
			ThreadStatus::Terminated,
		);
	}
}

// Exceptions
impl JavaThread {
	pub fn set_pending_exception(&self, exception: Reference) {
		self.set_control_flow(ControlFlow::ExceptionThrown);
		unsafe { *self.pending_exception.get() = Some(exception) }
	}

	pub fn has_pending_exception(&self) -> bool {
		unsafe { (*self.pending_exception.get()).is_some() }
	}

	pub fn pending_exception(&self) -> Option<Reference> {
		unsafe { *self.pending_exception.get() }
	}

	pub fn take_pending_exception(&self) -> Option<Reference> {
		self.set_control_flow(ControlFlow::Continue);
		unsafe { std::ptr::replace(self.pending_exception.get(), None) }
	}

	pub fn discard_pending_exception(&self) {
		let _ = self.take_pending_exception();
	}

	/// Wipe everything from the thread
	///
	/// Likely means we'll be exiting soon...
	fn nuke(&self) {
		self.frame_stack.clear();
		self.pc.store(0, Ordering::Relaxed);
	}

	fn on_exception(&self) {
		self.handle_pending_exception();
		if self.has_pending_exception() {
			// Uncaught exception, nothing further we can do
			self.nuke();
			self.set_control_flow(ControlFlow::Break);
			return;
		}

		// Exception handled, good to continue
		self.set_control_flow(ControlFlow::Continue);
	}

	/// Handle the pending exception on this thread
	pub fn handle_pending_exception(&self) {
		let Some(exception) = self.take_pending_exception() else {
			return;
		};

		self.set_state(JavaThreadState::Unwinding);
		assert!(!exception.is_null(), "failed to construct exception?");

		// https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-6.html#jvms-6.5.athrow
		// The objectref must be of type reference and must refer to an object that is an instance of class Throwable or of a subclass of Throwable.

		let class_instance = exception.extract_class();

		let throwable_class = globals::classes::java_lang_Throwable();
		assert!(
			class_instance.class() == throwable_class
				|| class_instance.is_subclass_of(throwable_class)
		);

		// Search each frame for an exception handler
		self.stash_and_reset_pc();
		while let Some(current_frame) = self.frame_stack.current() {
			let current_frame_pc = current_frame.stashed_pc();

			// If an exception handler that matches objectref is found, it contains the location of the code intended to handle this exception.
			if let Some(handler_pc) = current_frame
				.method()
				.find_exception_handler(class_instance.class(), current_frame_pc)
			{
				// The pc register is reset to that location, the operand stack of the current frame is cleared, objectref
				// is pushed back onto the operand stack, and execution continues.
				self.pc.store(handler_pc, Ordering::Relaxed);
				let _ = current_frame.take_cached_depth();

				current_frame.clear();
				current_frame.push_reference(exception);

				self.set_state(JavaThreadState::Running);

				// The exception was caught
				return;
			}

			let _ = self.frame_stack.pop();
		}

		// Wasn't caught, re-set the exception
		self.set_pending_exception(exception);
	}
}
