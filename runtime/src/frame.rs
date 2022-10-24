use std::sync::atomic::{AtomicUsize, Ordering};
use classfile::{u1, ConstantPool, u2};
use instructions::DefaultStack;

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.6
pub struct Frame<'a> {
	// TODO
	// pub locals: Vec<Local>,
	pub stack: DefaultStack,
	pub constant_pool: &'a ConstantPool,
	pub code: &'a [u1],
    pub pc: AtomicUsize, // Address of the currently executed instruction
}

impl<'a> Frame<'a> {
	pub fn new(stack_size: usize, constant_pool: &'a ConstantPool, code: &'a [u1]) -> Self {
		Self {
			stack: DefaultStack::new(stack_size),
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
