use crate::objects::reference::Reference;

use std::fmt::Debug;
use std::ops::{Index, IndexMut};

use common::box_slice;
use instructions::Operand;

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.6.1
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
	/// # Safety
	///
	/// This expects that all `Long` and `Double` operands have a corresponding `Empty` slot.
	///
	/// # Panics
	///
	/// This will panic if the stack size doesn't fit the existing arguments.
	pub unsafe fn new_with_args(mut args: Vec<Operand<Reference>>, stack_size: usize) -> Self {
		assert!(stack_size >= args.len());
		args.extend(std::iter::repeat_n(Operand::Empty, stack_size - args.len()));
		Self {
			inner: args.into_boxed_slice(),
		}
	}

	/// The total number of slots this `LocalStack` uses
	///
	/// This includes the empty slots used by `Long` and `Double` operands. See [`Self::occupied_slots`]
	/// for the number of slots that are actually occupied by an operand.
	pub fn total_slots(&self) -> usize {
		self.inner.len()
	}

	/// The number of slots that are actually occupied by an operand
	///
	/// This does not include the empty slots used by `Long` and `Double` operands. See [`Self::total_slots`]
	/// for the total number of slots used by this `LocalStack`.
	pub fn occupied_slots(&self) -> usize {
		self.inner
			.iter()
			.filter(|operand| !matches!(operand, Operand::Empty))
			.count()
	}

	pub fn iter(&self) -> LocalStackIter<'_> {
		self.into_iter()
	}
}

impl<'a> IntoIterator for &'a LocalStack {
	type Item = Operand<Reference>;
	type IntoIter = LocalStackIter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		LocalStackIter {
			inner: self.inner.iter(),
			remaining: self.occupied_slots(),
		}
	}
}

pub struct LocalStackIter<'a> {
	inner: std::slice::Iter<'a, Operand<Reference>>,
	remaining: usize,
}

impl Iterator for LocalStackIter<'_> {
	type Item = Operand<Reference>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.inner.next() {
			None => None,
			Some(Operand::Empty) => unreachable!("empty slots should never be encountered"),
			Some(operand) => {
				if matches!(operand, Operand::Long(_) | Operand::Double(_)) {
					// Skip the next slot
					assert_eq!(self.inner.next(), Some(&Operand::Empty));
				}

				self.remaining -= 1;
				Some(operand.clone())
			},
		}
	}
}

impl<'a> ExactSizeIterator for LocalStackIter<'a> {
	fn len(&self) -> usize {
		self.remaining
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
