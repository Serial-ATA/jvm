use crate::objects::reference::Reference;

use std::fmt::Debug;
use std::ops::{Index, IndexMut};

use common::box_slice;
use instructions::Operand;

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.6.1
#[derive(Clone, PartialEq)]
pub struct LocalStack {
	inner: Box<[Operand<Reference>]>,
}

impl Debug for LocalStack {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.inner.iter()).finish()
	}
}

impl LocalStack {
	pub fn new(stack_size: usize) -> Self {
		Self {
			// The length of the local variable array of a frame is determined at compile-time
			inner: box_slice![Operand::Empty; stack_size],
		}
	}

	/// Create a new `LocalStack` with existing arguments
	///
	/// # Panics
	///
	/// This will panic if the stack size doesn't fit the existing arguments.
	pub fn new_with_args(mut args: Vec<Operand<Reference>>, stack_size: usize) -> Self {
		assert!(stack_size >= args.len());
		args.extend(std::iter::repeat_n(Operand::Empty, stack_size - args.len()));
		Self {
			inner: args.into_boxed_slice(),
		}
	}

	pub fn into_inner(self) -> Box<[Operand<Reference>]> {
		self.inner
	}
}

// Local variables are addressed by indexing. The index of the first local variable is zero.
// An integer is considered to be an index into the local variable array if and only if that integer
// is between zero and one less than the size of the local variable array.
impl Index<usize> for LocalStack {
	type Output = Operand<Reference>;

	fn index(&self, index: usize) -> &Self::Output {
		&self.inner[index]
	}
}

impl IndexMut<usize> for LocalStack {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		self.inner.index_mut(index)
	}
}
