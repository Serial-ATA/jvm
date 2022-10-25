use std::sync::atomic::{AtomicUsize, Ordering};
use classfile::{u1, ConstantPool, u2};
use instructions::{LocalStack, OperandStack};

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
	pub code: &'a [u1],
    // TODO: move to thread
    // https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.5.1
    // Each Java Virtual Machine thread has its own pc (program counter) register [...]
    // the pc register contains the address of the Java Virtual Machine instruction currently being executed
    pub pc: AtomicUsize,
}

impl<'a> Frame<'a> {
	pub fn new(max_stack: usize, max_locals: usize, constant_pool: &'a ConstantPool, code: &'a [u1]) -> Self {
		Self {
            locals: LocalStack::new(max_locals),
            stack: OperandStack::new(max_stack),
			constant_pool,
			code,
            pc: AtomicUsize::new(0)
		}
	}

    pub fn read_byte(&mut self) -> u1 {
        let pc = self.pc.fetch_add(1, Ordering::Relaxed);
        self.code[pc]
    }

    pub fn read_byte2(&mut self) -> u2 {
        let b1 = u2::from(self.read_byte());
        let b2 = u2::from(self.read_byte());

        b1 << 8 | b2
    }
}
