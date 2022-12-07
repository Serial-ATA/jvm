use std::fmt::{Debug, Formatter};
use crate::heap::reference::MethodRef;
use crate::stack::local_stack::LocalStack;
use crate::stack::operand_stack::OperandStack;
use crate::thread::ThreadRef;

use std::sync::atomic::Ordering;
use std::sync::Arc;

use classfile::ConstantPoolRef;
use classfile::traits::PtrType;
use classfile::types::{u1, u2, u4};

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.6
#[rustfmt::skip]
#[derive(Debug)]
pub struct Frame {
    // Each frame has:

    // its own array of local variables (§2.6.1)
	pub locals: LocalStack,
    // its own operand stack (§2.6.2)
	pub stack: OperandStack,
    // and a reference to the run-time constant pool (§2.5.5)
	pub constant_pool: ConstantPoolRef,
	pub method: MethodRef,
	pub thread: ThreadRef,
}

#[repr(transparent)]
#[derive(Debug)]
pub struct FrameRef(Arc<FramePtr>);

impl FrameRef {
	fn new(ptr: FramePtr) -> Self {
		Self(Arc::new(ptr))
	}

	pub fn thread(&self) -> ThreadRef {
		Arc::clone(&self.0.get().thread)
	}

	pub fn method(&self) -> MethodRef {
		Arc::clone(&self.0.get().method)
	}

	pub fn get_operand_stack_mut(&self) -> &mut OperandStack {
		&mut self.0.get_mut().stack
	}

	pub fn read_byte(&self) -> u1 {
		let frame = self.0.get_mut();
		let thread = frame.thread.get();

		let pc = thread.pc.fetch_add(1, Ordering::Relaxed);
		frame.method.code.code[pc]
	}

	pub fn read_byte2(&mut self) -> u2 {
		let b1 = u2::from(self.read_byte());
		let b2 = u2::from(self.read_byte());

		b1 << 8 | b2
	}

	pub fn read_byte4(&mut self) -> u4 {
		let b1 = u4::from(self.read_byte());
		let b2 = u4::from(self.read_byte());
		let b3 = u4::from(self.read_byte());
		let b4 = u4::from(self.read_byte());

		b1 << 24 | b2 << 16 | b3 << 8 | b4
	}
}

// A pointer to a Class instance
//
// This can *not* be constructed by hand, as dropping it will
// deallocate the class.
#[derive(PartialEq)]
pub struct FramePtr(usize);

impl PtrType<Frame, FrameRef> for FramePtr {
	fn new(val: Frame) -> FrameRef {
		let boxed = Box::new(val);
		let ptr = Box::into_raw(boxed);
		FrameRef::new(Self(ptr as usize))
	}

	#[inline(always)]
	fn as_raw(&self) -> *const Frame {
		self.0 as *const Frame
	}

	#[inline(always)]
	fn as_mut_raw(&self) -> *mut Frame {
		self.0 as *mut Frame
	}

	fn get(&self) -> &Frame {
		unsafe { &(*self.as_raw()) }
	}

	fn get_mut(&self) -> &mut Frame {
		unsafe { &mut (*self.as_mut_raw()) }
	}
}

impl Drop for FramePtr {
	fn drop(&mut self) {
		let _ = unsafe { Box::from_raw(self.0 as *mut Frame) };
	}
}

impl Debug for FramePtr {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let frame = self.get();
		f.write_fmt(format_args!("{:?}", frame))
	}
}
