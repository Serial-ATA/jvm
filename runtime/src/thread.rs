use crate::frame::{Frame, FramePtr, FrameRef};
use crate::interpreter::Interpreter;
use crate::reference::MethodRef;
use crate::stack::local_stack::LocalStack;
use crate::stack::operand_stack::OperandStack;

use std::fmt::{Debug, Formatter};
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;

use classfile::traits::PtrType;

pub type ThreadRef = Arc<ThreadPtr>;

#[derive(Debug)]
pub struct Thread {
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.5.1
	// Each Java Virtual Machine thread has its own pc (program counter) register [...]
	// the pc register contains the address of the Java Virtual Machine instruction currently being executed
	pub pc: AtomicUsize,
	pub frame_stack: Vec<FrameRef>,
}

impl Thread {
	pub fn new() -> ThreadRef {
		let thread = Self {
			pc: AtomicUsize::new(0),
			frame_stack: Vec::new(),
		};

		ThreadPtr::new(thread)
	}

	pub fn invoke_method(thread: &ThreadRef, method: MethodRef) {
		let max_stack = method.code.max_stack;
		let max_locals = method.code.max_locals;

		let constant_pool = Arc::clone(&method.class.get().constant_pool);

		let frame = Frame {
			locals: LocalStack::new(max_locals as usize),
			stack: OperandStack::new(max_stack as usize),
			constant_pool,
			method,
			thread: Arc::clone(thread),
		};

		thread.get_mut().frame_stack.push(FramePtr::new(frame));
	}

	pub fn run(thread: &ThreadRef) {
		let thread = thread.get_mut();
		while let Some(frame) = thread.frame_stack.pop() {
			let mut interpreter = Interpreter::new(frame);
			interpreter.run();
		}
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

impl Debug for ThreadPtr {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let thread = self.get();
		f.write_fmt(format_args!("{:?}", thread))
	}
}
