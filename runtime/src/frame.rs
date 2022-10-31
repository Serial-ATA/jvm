use crate::heap::reference::MethodRef;
use crate::stack::local_stack::LocalStack;
use crate::stack::operand_stack::OperandStack;

use std::sync::atomic::{AtomicUsize, Ordering};

use classfile::types::{u1, u2, u4};
use classfile::ConstantPool;

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.6
#[rustfmt::skip]
pub struct Frame<'a> {
    // Each frame has:

    // its own array of local variables (§2.6.1)
	pub locals: LocalStack,
    // its own operand stack (§2.6.2)
	pub stack: OperandStack,
    // and a reference to the run-time constant pool (§2.5.5)
	pub constant_pool: &'a ConstantPool,
	pub method: MethodRef,
    // TODO: move to thread
    // https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.5.1
    // Each Java Virtual Machine thread has its own pc (program counter) register [...]
    // the pc register contains the address of the Java Virtual Machine instruction currently being executed
    pub pc: AtomicUsize,
}

impl<'a> Frame<'a> {
	pub fn new(method: MethodRef, constant_pool: &'a ConstantPool) -> Self {
		let max_stack = method.code.max_stack;
		let max_locals = method.code.max_locals;

		Self {
			locals: LocalStack::new(max_locals as usize),
			stack: OperandStack::new(max_stack as usize),
			constant_pool,
			method,
			pc: AtomicUsize::new(0),
		}
	}

	pub fn read_byte(&mut self) -> u1 {
		let pc = self.pc.fetch_add(1, Ordering::Relaxed);
		self.method.code.code[pc]
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
