use super::operand_stack::Operand;

use std::ops::Index;

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.6.1
#[derive(Debug, Clone, PartialEq)]
pub struct LocalStack {
	inner: Box<[Operand]>,
}

impl LocalStack {
	pub fn new(stack_size: usize) -> Self {
		Self {
			// The length of the local variable array of a frame is determined at compile-time
			inner: vec![Operand::Empty; stack_size].into_boxed_slice(),
		}
	}
}

// Local variables are addressed by indexing. The index of the first local variable is zero.
// An integer is considered to be an index into the local variable array if and only if that integer
// is between zero and one less than the size of the local variable array.
impl Index<usize> for LocalStack {
	type Output = Operand;

	fn index(&self, index: usize) -> &Self::Output {
		&self.inner[index]
	}
}
