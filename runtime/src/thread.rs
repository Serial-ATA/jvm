use crate::classpath::classloader::ClassLoader;
use crate::frame::{Frame, FramePtr, FrameRef};
use crate::interpreter::Interpreter;
use crate::native::{JNIEnv, NativeMethodDef};
use crate::reference::{MethodRef, Reference};
use crate::stack::local_stack::LocalStack;
use crate::stack::operand_stack::OperandStack;

use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::Arc;

use common::int_types::u1;
use common::traits::PtrType;
use instructions::{Operand, StackLike};

pub struct JVMOptions {
	pub dry_run: bool,
	pub system_properties: Option<Vec<String>>,
	pub showversion: bool,
	pub show_version: bool,
}

pub type ThreadRef = Arc<ThreadPtr>;

#[derive(Debug)]
pub struct Thread {
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.5.1
	// Each Java Virtual Machine thread has its own pc (program counter) register [...]
	// the pc register contains the address of the Java Virtual Machine instruction currently being executed
	pub pc: AtomicIsize,
	pub frame_stack: Vec<FrameRef>,
}

impl Thread {
	pub fn new() -> ThreadRef {
		let thread = Self {
			pc: AtomicIsize::new(0),
			frame_stack: Vec::new(),
		};

		ThreadPtr::new(thread)
	}

	pub fn new_main(
		class_name: &[u1],
		jvm_options: JVMOptions,
		_main_args: Vec<String>,
	) -> ThreadRef {
		let thread = Thread::new();
		crate::initialization::initialize(Arc::clone(&thread));

		let class = ClassLoader::Bootstrap.load(class_name).unwrap();
		let main_method = class.get().get_main_method().unwrap();

		if !jvm_options.dry_run {
			// TODO: Convert rust string args to java strings to pass to main
			Thread::invoke_method(Arc::clone(&thread), main_method, None);
		}

		thread
	}

	// This is just `invoke_method`, but it calls `run` since we aren't
	// in the main method yet.
	pub fn pre_main_invoke_method(
		thread: ThreadRef,
		method: MethodRef,
		args: Option<Vec<Operand<Reference>>>,
	) {
		Self::invoke_method(Arc::clone(&thread), method, args);
		Thread::run(&thread)
	}

	fn invoke_method(thread: ThreadRef, method: MethodRef, args: Option<Vec<Operand<Reference>>>) {
		let max_locals = method.code.max_locals;
		let mut local_stack = LocalStack::new(max_locals as usize);

		if let Some(args) = args {
			for (idx, arg) in args.into_iter().enumerate() {
				local_stack[idx] = arg;
			}
		}

		Self::invoke_method_with_local_stack(thread, method, local_stack);
	}

	pub fn invoke_method_with_local_stack(
		thread: ThreadRef,
		method: MethodRef,
		locals: LocalStack,
	) {
		// Native methods do not require a stack frame. We just call and leave the stack
		// behind until we return.
		if method.is_native() {
			let fn_ptr = super::native::lookup_method(NativeMethodDef {
				class: &method.class.get().name,
				name: &method.name,
				descriptor: &method.descriptor,
			});

			let env = JNIEnv {
				current_thread: Arc::clone(&thread),
			};

			// Push the return value onto the frame's stack
			if let Some(ret) = fn_ptr(env, locals) {
				thread
					.get()
					.current_frame()
					.unwrap()
					.get_operand_stack_mut()
					.push_op(ret);
			}

			return;
		}

		let max_stack = method.code.max_stack;

		let constant_pool = Arc::clone(&method.class.unwrap_class_instance().constant_pool);

		let frame = Frame {
			locals,
			stack: OperandStack::new(max_stack as usize),
			constant_pool,
			method,
			thread: Arc::clone(&thread),
			cached_pc: AtomicIsize::default(),
		};

		let thread = thread.get_mut();

		thread.stash_and_reset_pc();
		thread.frame_stack.push(FramePtr::new(frame));
	}

	pub fn stash_and_reset_pc(&mut self) {
		if let Some(current_frame) = self.current_frame() {
			current_frame.stash_pc()
		}

		self.pc.store(0, Ordering::Relaxed);
	}

	pub fn current_frame(&self) -> Option<FrameRef> {
		let current_frame = self.frame_stack.last();
		current_frame.map(FrameRef::clone)
	}

	pub fn drop_to_previous_frame(&mut self, return_value: Option<Operand<Reference>>) {
		self.frame_stack.pop();

		if let Some(current_frame) = self.current_frame() {
			// Restore the pc of the frame
			let previous_pc = current_frame.get_stashed_pc();
			self.pc.store(previous_pc, Ordering::Relaxed);

			// Push the return value of the previous frame if there is one
			if let Some(return_value) = return_value {
				current_frame.get_operand_stack_mut().push_op(return_value);
			}
		}
	}

	pub fn run(thread: &ThreadRef) {
		let thread = thread.get_mut();
		while let Some(current_frame) = thread.current_frame() {
			Interpreter::instruction(current_frame);
		}
	}

	pub fn throw_exception(thread: ThreadRef, object_ref: Reference) {
		// https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-6.html#jvms-6.5.athrow
		// The objectref must be of type reference and must refer to an object that is an instance of class Throwable or of a subclass of Throwable.
		let class_instance;
		match object_ref {
			Reference::Class(ref instance) => class_instance = instance,
			Reference::Null => {
				thread.get_mut().throw_npe();
				return;
			},
			_ => unreachable!("Invalid reference type on operand stack for athrow instruction"),
		};

		let throwable_class = ClassLoader::Bootstrap
			.load(b"java/lang/Throwable")
			.expect("java/lang/Throwable class should be available");
		assert!(
			class_instance.get().class == throwable_class
				|| class_instance.get().class.is_subclass_of(throwable_class)
		);

		// Search each frame for an exception handler
		thread.get_mut().stash_and_reset_pc();
		while let Some(current_frame) = thread.current_frame() {
			let current_frame_pc = current_frame.get_stashed_pc();

			// If an exception handler that matches objectref is found, it contains the location of the code intended to handle this exception.
			if let Some(handler_pc) = current_frame
				.method()
				.find_exception_handler(Arc::clone(&class_instance.get().class), current_frame_pc)
			{
				// The pc register is reset to that location,the operand stack of the current frame is cleared, objectref
				// is pushed back onto the operand stack, and execution continues.
				thread.get_mut().pc = AtomicIsize::new(handler_pc);

				let stack = current_frame.get_operand_stack_mut();
				stack.clear();
				stack.push_reference(object_ref);

				return;
			}

			let _ = thread.get_mut().frame_stack.pop();
		}

		// No handler found, we have to print the stack trace and exit
		thread.get_mut().frame_stack.clear();

		let print_stack_trace = class_instance
			.get()
			.class
			.get_method(b"printStackTrace", b"()V", 0)
			.expect("java/lang/Throwable#printStackTrace should exist");

		let mut locals = LocalStack::new(1);
		locals[0] = Operand::Reference(object_ref);

		Thread::invoke_method_with_local_stack(thread, print_stack_trace, locals);
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
pub struct ThreadPtr(usize);

impl PtrType<Thread, ThreadRef> for ThreadPtr {
	fn new(val: Thread) -> ThreadRef {
		let boxed = Box::new(val);
		let ptr = Box::into_raw(boxed);
		ThreadRef::new(Self(ptr as usize))
	}

	#[inline(always)]
	fn as_raw(&self) -> *const Thread {
		self.0 as *const Thread
	}

	#[inline(always)]
	fn as_mut_raw(&self) -> *mut Thread {
		self.0 as *mut Thread
	}

	fn get(&self) -> &Thread {
		unsafe { &(*self.as_raw()) }
	}

	fn get_mut(&self) -> &mut Thread {
		unsafe { &mut (*self.as_mut_raw()) }
	}
}

impl Drop for ThreadPtr {
	fn drop(&mut self) {
		let _ = unsafe { Box::from_raw(self.0 as *mut Thread) };
	}
}

impl Deref for ThreadPtr {
	type Target = Thread;

	fn deref(&self) -> &Self::Target {
		unsafe { &(*self.as_raw()) }
	}
}

impl Debug for ThreadPtr {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let thread = self.get();
		f.write_fmt(format_args!("{:?}", thread))
	}
}
