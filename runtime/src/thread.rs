use crate::class_instance::{ClassInstance, Instance};
use crate::frame::Frame;
use crate::interpreter::Interpreter;
use crate::java_call;
use crate::method::Method;
use crate::native::jni::invocation_api::new_env;
use crate::reference::{ClassInstanceRef, Reference};
use crate::stack::local_stack::LocalStack;
use crate::string_interner::StringInterner;

use std::cell::{SyncUnsafeCell, UnsafeCell};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::Arc;

use classfile::accessflags::MethodAccessFlags;
use common::int_types::s4;
use common::traits::PtrType;
use instructions::{Operand, StackLike};
use jni::env::JniEnv;
use symbols::sym;

#[thread_local]
static CURRENT_JAVA_THREAD: SyncUnsafeCell<Option<JavaThread>> = SyncUnsafeCell::new(None);

pub struct JVMOptions {
	pub dry_run: bool,
	pub system_properties: Option<Vec<String>>,
	pub showversion: bool,
	pub show_version: bool,
}

#[derive(Debug)]
enum StackFrame {
	Real(Frame),
	Fake,
}

impl StackFrame {
	fn is_fake(&self) -> bool {
		matches!(self, StackFrame::Fake)
	}
}

#[derive(Debug)]
pub struct FrameStack {
	inner: UnsafeCell<Vec<StackFrame>>,
}

impl FrameStack {
	// TODO
	fn new() -> Self {
		FrameStack {
			inner: UnsafeCell::new(Vec::with_capacity(1024)),
		}
	}

	fn current(&self) -> Option<&mut Frame> {
		let current_frame = self.__inner_mut().last_mut();
		match current_frame {
			Some(StackFrame::Real(r)) => Some(r),
			_ => None,
		}
	}

	pub fn depth(&self) -> usize {
		self.__inner().len()
	}

	pub fn iter(&self) -> impl DoubleEndedIterator<Item = &Frame> {
		self.__inner().iter().filter_map(|frame| match frame {
			StackFrame::Real(frame) => Some(frame),
			StackFrame::Fake => None,
		})
	}

	pub fn get(&self, position: usize) -> Option<&Frame> {
		match self.__inner().get(position) {
			Some(StackFrame::Real(frame)) => Some(frame),
			None => None,
			_ => unreachable!(),
		}
	}

	fn push(&self, frame: StackFrame) {
		self.__inner_mut().push(frame);
	}

	fn pop(&self) -> Option<StackFrame> {
		self.__inner_mut().pop()
	}

	fn pop_real(&self) -> Option<Frame> {
		match self.__inner_mut().pop() {
			Some(StackFrame::Real(r)) => Some(r),
			_ => None,
		}
	}

	fn pop_dummy(&self) {
		match self.__inner_mut().pop() {
			Some(StackFrame::Fake) => return,
			_ => panic!("Expected a dummy frame!"),
		}
	}

	fn clear(&self) {
		self.__inner_mut().clear();
	}

	fn __inner(&self) -> &mut Vec<StackFrame> {
		unsafe { &mut *self.inner.get() }
	}

	fn __inner_mut(&self) -> &mut Vec<StackFrame> {
		unsafe { &mut *self.inner.get() }
	}
}

#[repr(C)]
pub struct JavaThread {
	env: JniEnv,
	obj: UnsafeCell<Option<Reference>>,

	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.5.1
	// Each Java Virtual Machine thread has its own pc (program counter) register [...]
	// the pc register contains the address of the Java Virtual Machine instruction currently being executed
	pub pc: AtomicIsize,
	frame_stack: FrameStack,

	remaining_operand: UnsafeCell<Option<Operand<Reference>>>,
}

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
		match opt {
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
		opt.as_ref()
	}

	/// Sets the current Java [`JavaThread`]
	///
	/// # Panics
	///
	/// This will panic if there is already a current thread set
	pub unsafe fn set_current_thread(thread: JavaThread) {
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

/// `java.lang.Thread$FieldHolder` accessors
pub mod java_lang_Thread {
	use crate::class_instance::Instance;
	use crate::reference::Reference;
	use crate::thread::ThreadStatus;
	use crate::JavaThread;
	use common::int_types::s4;
	use common::traits::PtrType;
	use instructions::Operand;
	use symbols::sym;

	pub fn set_field_offsets() {
		// java.lang.Thread fields
		{
			let class = crate::globals::classes::java_lang_Thread();

			let mut field_set = 0;
			for (index, field) in class.instance_fields().enumerate() {
				if field.name == sym!(holder) {
					unsafe {
						crate::globals::field_offsets::set_thread_holder_field_offset(index);
					}

					field_set |= 1;
					continue;
				}

				if field.name == sym!(eetop) {
					unsafe {
						crate::globals::field_offsets::set_thread_eetop_field_offset(index);
					}

					field_set |= 1 << 1;
					continue;
				}
			}

			assert_eq!(
				field_set, 0b11,
				"Not all fields were found in java/lang/Thread"
			);
		}

		set_field_holder_offsets();
	}

	// java.lang.Thread$FieldHolder fields
	fn set_field_holder_offsets() {
		let class = crate::globals::classes::java_lang_Thread_FieldHolder();

		let mut field_set = 0;
		for (index, field) in class.fields().enumerate() {
			match field.name.as_str() {
				"priority" => unsafe {
					crate::globals::field_offsets::set_field_holder_priority_field_offset(index);
					field_set |= 1;
				},
				"daemon" => unsafe {
					crate::globals::field_offsets::set_field_holder_daemon_field_offset(index);
					field_set |= 1 << 1;
				},
				"threadStatus" => unsafe {
					crate::globals::field_offsets::set_field_holder_thread_status_field_offset(
						index,
					);
					field_set |= 1 << 2;
				},
				_ => {},
			}
		}

		assert_eq!(
			field_set, 0b111,
			"Not all fields were found in java/lang/Thread$FieldHolder"
		);
	}

	pub(super) fn set_eetop(obj: Reference, eetop: jni::sys::jlong) {
		let offset = crate::globals::field_offsets::thread_eetop_field_offset();

		let instance = obj.extract_class();
		instance
			.get_mut()
			.put_field_value0(offset, Operand::Long(eetop));
	}

	fn set_field_holder_field(obj: Reference, offset: usize, value: Operand<Reference>) {
		let class_instance = obj.extract_class();

		let field_holder_offset = crate::globals::field_offsets::thread_holder_field_offset();
		let field_holder_ref = &class_instance
			.get_mut()
			.get_field_value0(field_holder_offset);

		let field_holder_instance = field_holder_ref.expect_reference().extract_class();
		field_holder_instance
			.get_mut()
			.put_field_value0(offset, value);
	}

	pub fn set_priority(obj: Reference, priority: s4) {
		let offset = crate::globals::field_offsets::field_holder_priority_field_offset();
		set_field_holder_field(obj, offset, Operand::Int(priority));
	}

	fn set_daemon(_obj: Reference, _daemon: bool) {
		todo!()
	}

	pub(super) fn set_thread_status(obj: Reference, thread_status: ThreadStatus) {
		let offset = crate::globals::field_offsets::field_holder_thread_status_field_offset();
		set_field_holder_field(obj, offset, Operand::Int(thread_status as s4));
	}
}

// Actions for the related `java.lang.Thread` instance
impl JavaThread {
	/// Get the `JavaThread` associated with `obj`
	///
	/// # Safety
	///
	/// The caller must ensure that `obj` is a `java.lang.Thread` object obtained from [`Self::obj()`]
	pub unsafe fn for_obj(obj: ClassInstanceRef) -> Option<*mut JavaThread> {
		let eetop_offset = crate::globals::field_offsets::thread_eetop_field_offset();
		let field_value_ptr = unsafe { obj.get().get_field_value_raw(eetop_offset) };
		let field_value = unsafe { field_value_ptr.as_ref() };
		let eetop = field_value.expect_long();
		if eetop == 0 {
			// Thread is not alive
			return None;
		}

		Some(eetop as *mut JavaThread)
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

		java_lang_Thread::set_thread_status(obj, ThreadStatus::Runnable);
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
	pub fn new() -> Self {
		Self {
			env: unsafe { JniEnv::from_raw(new_env()) },
			obj: UnsafeCell::new(None),

			pc: AtomicIsize::new(0),
			frame_stack: FrameStack::new(),
			remaining_operand: UnsafeCell::new(None),
		}
	}

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

		let constant_pool = Arc::clone(&method.class.unwrap_class_instance().constant_pool);

		let frame = Frame::new(self, locals, max_stack, constant_pool, method);

		self.stash_and_reset_pc();
		self.frame_stack.push(StackFrame::Real(frame));
	}

	// Native methods do not require a stack frame. We just call and leave the stack behind until we return.
	fn invoke_native(&self, method: &Method, locals: LocalStack) {
		// Try to lookup and set the method prior to calling
		crate::native::lookup::lookup_native_method(method, self);

		let fn_ptr = super::native::lookup_method(method);

		// Push the return value onto the frame's stack
		if let Some(ret) = fn_ptr(self.env(), locals) {
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
		// https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-6.html#jvms-6.5.athrow
		// The objectref must be of type reference and must refer to an object that is an instance of class Throwable or of a subclass of Throwable.
		if object_ref.is_null() {
			self.throw_npe();
			return;
		}

		let class_instance = object_ref.extract_class();

		let throwable_class = crate::globals::classes::java_lang_Throwable();
		assert!(
			class_instance.get().class() == throwable_class
				|| class_instance.get().is_subclass_of(&throwable_class)
		);

		// Search each frame for an exception handler
		self.stash_and_reset_pc();
		while let Some(current_frame) = self.frame_stack.current() {
			let current_frame_pc = current_frame.stashed_pc();

			// If an exception handler that matches objectref is found, it contains the location of the code intended to handle this exception.
			if let Some(handler_pc) = current_frame
				.method()
				.find_exception_handler(class_instance.get().class(), current_frame_pc)
			{
				// The pc register is reset to that location,the operand stack of the current frame is cleared, objectref
				// is pushed back onto the operand stack, and execution continues.
				self.pc.store(handler_pc, Ordering::Relaxed);

				let stack = current_frame.stack_mut();
				stack.clear();
				stack.push_reference(object_ref);

				return;
			}

			let _ = self.frame_stack.pop();
		}

		// No handler found, we have to print the stack trace and exit
		self.frame_stack.clear();

		let print_stack_trace = class_instance
			.get()
			.class()
			.vtable()
			.find(
				sym!(printStackTrace_name),
				sym!(void_method_signature),
				MethodAccessFlags::NONE,
			)
			.expect("java/lang/Throwable#printStackTrace should exist");

		let mut locals = LocalStack::new(1);
		locals[0] = Operand::Reference(object_ref);

		self.invoke_method_with_local_stack(print_stack_trace, locals);
	}

	/// Throw a `NullPointerException` on this thread
	pub fn throw_npe(&self) {
		todo!()
	}
}
