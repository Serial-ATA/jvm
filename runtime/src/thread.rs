use crate::class_instance::ClassInstance;
use crate::frame::{Frame, FramePtr, FrameRef};
use crate::interpreter::Interpreter;
use crate::java_call;
use crate::method::Method;
use crate::native::jni::invocation_api::new_env;
use crate::reference::{ClassInstanceRef, ClassRef, Reference};
use crate::stack::local_stack::LocalStack;
use crate::stack::operand_stack::OperandStack;
use crate::string_interner::StringInterner;

use std::cell::UnsafeCell;
use std::mem::ManuallyDrop;
use std::ops::Deref;
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
static mut CURRENT_JAVA_THREAD: Option<UnsafeCell<JavaThread>> = None;

pub struct JVMOptions {
	pub dry_run: bool,
	pub system_properties: Option<Vec<String>>,
	pub showversion: bool,
	pub show_version: bool,
}

pub type ThreadRef = Arc<ThreadPtr>;

#[derive(Debug)]
enum StackFrame {
	Ref(FrameRef),
	Fake,
}

impl PartialEq<FrameRef> for StackFrame {
	fn eq(&self, other: &FrameRef) -> bool {
		match self {
			StackFrame::Ref(rf) => rf == other,
			StackFrame::Fake => false,
		}
	}
}

impl StackFrame {
	fn is_fake(&self) -> bool {
		matches!(self, StackFrame::Fake)
	}
}

#[repr(C)]
pub struct JavaThread {
	env: JniEnv,
	obj: Option<Reference>,

	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.5.1
	// Each Java Virtual Machine thread has its own pc (program counter) register [...]
	// the pc register contains the address of the Java Virtual Machine instruction currently being executed
	pub pc: AtomicIsize,
	pub frame_stack: Vec<StackFrame>,

	// TODO: HACK!!!!
	remaining_operand: Option<Operand<Reference>>,
}

impl PartialEq for JavaThread {
	fn eq(&self, other: &Self) -> bool {
		self.env() == other.env()
	}
}

/// Global accessors
impl JavaThread {
	pub unsafe fn for_env(env: *const JniEnv) -> *mut JavaThread {
		unsafe {
			let ptr = env.sub(core::mem::offset_of!(JavaThread, env));
			ptr.cast::<Self>() as _
		}
	}

	pub fn current() -> &'static JavaThread {
		Self::current_opt().expect("current JavaThread should be available")
	}

	pub fn current_opt() -> Option<&'static JavaThread> {
		unsafe {
			CURRENT_JAVA_THREAD
				.as_ref()
				.map(|thread| core::mem::transmute(thread.get() as *const _))
		}
	}

	pub fn current_mut() -> &'static mut JavaThread {
		Self::current_opt_mut().expect("current JavaThread should be available")
	}

	pub fn current_opt_mut() -> Option<&'static mut JavaThread> {
		unsafe { CURRENT_JAVA_THREAD.as_mut().map(|thread| thread.get_mut()) }
	}

	/// Sets the current Java [`JavaThread`]
	///
	/// # Panics
	///
	/// This will panic if there is already a current thread set
	pub unsafe fn set_current_thread(thread: JavaThread) {
		assert!(
			CURRENT_JAVA_THREAD.is_none(),
			"attempting to overwrite an existing JavaThread"
		);
		CURRENT_JAVA_THREAD = Some(UnsafeCell::new(thread));
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
impl JavaThread {
	pub fn set_field_holder_offsets() {
		// java.lang.Thread#holder
		{
			let class = crate::globals::classes::java_lang_Thread();
			for (index, field) in class
				.fields()
				.filter(|field| !field.is_static())
				.enumerate()
			{
				if field.name == b"holder" {
					unsafe {
						crate::globals::field_offsets::set_thread_holder_field_offset(index);
					}
				}
			}
		}

		// java.lang.Thread$FieldHolder fields
		{
			let class = crate::globals::classes::java_lang_Thread_FieldHolder();
			for (index, field) in class.static_fields().enumerate() {
				match &*field.name {
					b"priority" => unsafe {
						crate::globals::field_offsets::set_field_holder_priority_field_offset(index)
					},
					b"daemon" => unsafe {
						crate::globals::field_offsets::set_field_holder_daemon_field_offset(index)
					},
					b"threadStatus" => unsafe {
						crate::globals::field_offsets::set_field_holder_thread_status_field_offset(
							index,
						)
					},
					_ => {},
				}
			}
		}
	}

	fn set_field_holder_field(&mut self, offset: usize, value: Operand<Reference>) {
		let obj = self.obj.as_ref().map(Reference::clone).unwrap();
		let class_instance = obj.extract_class();

		let field_holder_offset = crate::globals::field_offsets::thread_holder_field_offset();
		let field_holder_ref = &class_instance.get_mut().fields[field_holder_offset];

		let field_holder_instance = field_holder_ref.expect_reference().extract_class();
		field_holder_instance.get_mut().fields[offset] = value;
	}

	fn set_priority(&mut self, _priority: s4) {
		assert!(self.obj.is_some());
		todo!()
	}

	fn set_daemon(&mut self, _daemon: bool) {
		assert!(self.obj.is_some());
		todo!()
	}

	fn set_thread_status(&mut self, thread_status: ThreadStatus) {
		assert!(self.obj.is_some());
		let offset = crate::globals::field_offsets::field_holder_thread_status_field_offset();
		self.set_field_holder_field(offset, Operand::Int(thread_status as s4));
	}
}

impl JavaThread {
	pub fn new() -> Self {
		Self {
			env: unsafe { JniEnv::from_raw(new_env()) },
			obj: None,

			pc: AtomicIsize::new(0),
			frame_stack: Vec::new(),
			remaining_operand: None,
		}
	}

	pub fn env(&self) -> NonNull<JniEnv> {
		unsafe {
			NonNull::new_unchecked(
				std::ptr::from_ref(self).add(core::mem::offset_of!(JavaThread, env)) as _,
			)
		}
	}

	/// Allocates a new `java.lang.Thread` for this `JavaThread`
	///
	/// This is called from the JNI `AttachCurrentThread`/`AttachCurrentThreadAsDaemon`.
	pub fn attach_thread_obj(&mut self, name: Option<&str>, thread_group: Reference, daemon: bool) {
		assert!(self.obj.is_none());

		let thread_class = crate::globals::classes::java_lang_Thread();
		let thread_instance = ClassInstance::new(ClassRef::clone(&thread_class));

		if let Some(name) = name {
			let string_object = StringInterner::get_java_string(name);
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

		self.obj = Some(Reference::class(thread_instance));
	}

	pub fn init_obj(&mut self, thread_group: Reference) {
		let thread_class = crate::globals::classes::java_lang_Thread();
		let thread_instance = ClassInstance::new(ClassRef::clone(&thread_class));

		// Set the obj early, since the java.lang.Thread constructor calls Thread#current.
		self.set_obj(Reference::class(ClassInstanceRef::clone(&thread_instance)));

		let init_method = thread_class
			.vtable()
			.find(
				sym!(object_initializer_name),
				sym!(ThreadGroup_String_void_signature),
				MethodAccessFlags::NONE,
			)
			.expect("method should exist");

		let thread_name = StringInterner::get_java_string("main");

		java_call!(
			self,
			init_method,
			Operand::Reference(Reference::class(ClassInstanceRef::clone(&thread_instance))),
			Operand::Reference(thread_group),
			Operand::Reference(Reference::class(thread_name)),
		);

		self.set_thread_status(ThreadStatus::Runnable);
	}

	pub fn set_obj(&mut self, obj: Reference) {
		self.obj = Some(obj);
	}

	pub fn obj(&self) -> Option<Reference> {
		self.obj.as_ref().map(Reference::clone)
	}

	pub fn frames(&self) -> impl DoubleEndedIterator<Item = &FrameRef> {
		self.frame_stack.iter().filter_map(|frame| match frame {
			StackFrame::Ref(frame_ref) => Some(frame_ref),
			StackFrame::Fake => None,
		})
	}

	pub fn frame_at(&self, position: usize) -> Option<FrameRef> {
		match self.frame_stack.get(position) {
			Some(StackFrame::Ref(frame_ref)) => Some(frame_ref.clone()),
			None => None,
			_ => unreachable!(),
		}
	}

	pub fn stack_depth(&self) -> usize {
		self.frame_stack.len()
	}

	// TODO: HACK!!!
	pub fn pull_remaining_operand(&mut self) -> Option<Operand<Reference>> {
		self.remaining_operand.take()
	}

	pub fn invoke_method_with_local_stack(&mut self, method: &'static Method, locals: LocalStack) {
		if method.is_native() {
			tracing::debug!(target: "JavaThread", "Invoking native method `{:?}`", method);
			self.invoke_native(method, locals);
			tracing::debug!(target: "JavaThread", "Native method finished");
			return;
		}

		let max_stack = method.code.max_stack;

		let constant_pool = Arc::clone(&method.class.unwrap_class_instance().constant_pool);

		let frame = Frame {
			locals,
			stack: OperandStack::new(max_stack as usize),
			constant_pool,
			method,
			thread: Arc::new(ThreadPtr(&raw mut *self)),
			cached_pc: AtomicIsize::default(),
		};

		self.stash_and_reset_pc();
		self.frame_stack.push(StackFrame::Ref(FramePtr::new(frame)));
	}

	// Native methods do not require a stack frame. We just call and leave the stack behind until we return.
	fn invoke_native(&mut self, method: &Method, locals: LocalStack) {
		// Try to lookup and set the method prior to calling
		crate::native::lookup::lookup_native_method(method, self);

		let fn_ptr = super::native::lookup_method(method);

		// Push the return value onto the frame's stack
		if let Some(ret) = fn_ptr(self.env(), locals) {
			self.current_frame()
				.unwrap()
				.get_operand_stack_mut()
				.push_op(ret);
		}

		return;
	}

	pub fn stash_and_reset_pc(&mut self) {
		if let Some(current_frame) = self.current_frame() {
			current_frame.stash_pc()
		}

		self.pc.store(0, Ordering::Relaxed);
	}

	pub fn current_frame(&self) -> Option<FrameRef> {
		let current_frame = self.frame_stack.last();
		match current_frame {
			Some(StackFrame::Ref(r)) => Some(r.clone()),
			_ => None,
		}
	}

	pub fn drop_to_previous_frame(
		&mut self,
		current_frame: FrameRef,
		return_value: Option<Operand<Reference>>,
	) {
		// We consume the current frame from the interpreter and wrap it in `ManuallyDrop`
		// We then immediately drop the frame from the frame stack, ensuring it is only freed once.
		let _md = ManuallyDrop::new(current_frame);
		let frame = self.frame_stack.pop().expect("frame stack is empty");

		assert_eq!(&frame, &*_md);
		drop(frame);

		if let Some(current_frame) = self.current_frame() {
			tracing::debug!(target: "JavaThread", "Dropping back to frame for method `{:?}`", current_frame.method());

			// Restore the pc of the frame
			let previous_pc = current_frame.get_stashed_pc();
			self.pc.store(previous_pc, Ordering::Relaxed);

			// Push the return value of the previous frame if there is one
			if let Some(return_value) = return_value {
				current_frame.get_operand_stack_mut().push_op(return_value);
			}

			return;
		}

		// TODO: HACK!!!
		self.remaining_operand = return_value;
	}

	pub fn run(&mut self) {
		while let Some(current_frame) = self.current_frame() {
			Interpreter::instruction(current_frame);
		}

		if let Some(true) = self.frame_stack.last().map(StackFrame::is_fake) {
			let _ = self.frame_stack.pop();
		}
	}

	pub fn throw_exception(&mut self, object_ref: Reference) {
		// https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-6.html#jvms-6.5.athrow
		// The objectref must be of type reference and must refer to an object that is an instance of class Throwable or of a subclass of Throwable.
		if object_ref.is_null() {
			self.throw_npe();
			return;
		}

		let class_instance = object_ref.extract_class();

		let throwable_class = crate::globals::classes::java_lang_Throwable();
		assert!(
			class_instance.get().class == throwable_class
				|| class_instance.get().class.is_subclass_of(throwable_class)
		);

		// Search each frame for an exception handler
		self.stash_and_reset_pc();
		while let Some(current_frame) = self.current_frame() {
			let current_frame_pc = current_frame.get_stashed_pc();

			// If an exception handler that matches objectref is found, it contains the location of the code intended to handle this exception.
			if let Some(handler_pc) = current_frame
				.method()
				.find_exception_handler(Arc::clone(&class_instance.get().class), current_frame_pc)
			{
				// The pc register is reset to that location,the operand stack of the current frame is cleared, objectref
				// is pushed back onto the operand stack, and execution continues.
				self.pc = AtomicIsize::new(handler_pc);

				let stack = current_frame.get_operand_stack_mut();
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
			.class
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

	pub fn throw_npe(&mut self) {
		todo!()
	}
}

// A pointer to a Thread instance
//
// This can *not* be constructed by hand, as dropping it will
// deallocate the thread.
#[derive(PartialEq)]
pub struct ThreadPtr(*mut JavaThread);

impl PtrType<JavaThread, ThreadRef> for ThreadPtr {
	fn new(val: JavaThread) -> ThreadRef {
		let boxed = Box::new(val);
		let ptr = Box::into_raw(boxed);
		ThreadRef::new(Self(ptr as _))
	}

	#[inline(always)]
	fn as_raw(&self) -> *const JavaThread {
		self.0 as _
	}

	#[inline(always)]
	fn as_mut_raw(&self) -> *mut JavaThread {
		self.0
	}

	fn get(&self) -> &JavaThread {
		unsafe { &(*self.as_raw()) }
	}

	fn get_mut(&self) -> &mut JavaThread {
		unsafe { &mut (*self.as_mut_raw()) }
	}
}

impl Drop for ThreadPtr {
	fn drop(&mut self) {
		let _ = unsafe { Box::from_raw(self.0) };
	}
}

impl Deref for ThreadPtr {
	type Target = JavaThread;

	fn deref(&self) -> &Self::Target {
		unsafe { &(*self.as_raw()) }
	}
}
