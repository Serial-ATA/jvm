use classfile::{u1, ConstantPool};
use instructions::DefaultStack;

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.6
pub struct Frame<'a> {
	// TODO
	// pub locals: Vec<Local>,
	pub stack: DefaultStack,
	pub constant_pool: &'a ConstantPool,
	pub code: &'a [u1],
}

impl<'a> Frame<'a> {
	pub fn new(stack_size: usize, constant_pool: &'a ConstantPool, code: &'a [u1]) -> Self {
		Self {
			stack: DefaultStack::new(stack_size),
			constant_pool,
			code,
		}
	}
}
