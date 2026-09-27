use crate::objects::reference::Reference;
use crate::thread::JavaThread;

use std::fmt::Debug;
use std::ops::{Index, IndexMut};

use ::jni::objects::JObject;
use common::box_slice;
use instructions::Operand;

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.6.1
#[derive(Clone, PartialEq)]
pub struct LocalStack<REFERENCE = Reference> {
	inner: Box<[Operand<REFERENCE>]>,
}

impl<REFERENCE: Debug> Debug for LocalStack<REFERENCE> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.inner.iter()).finish()
	}
}

impl LocalStack<Reference> {
	/// Convert the `LocalStack` into a JNI-compatible stack
	pub fn into_jni(self, thread: &'static JavaThread) -> LocalStack<JObject> {
		LocalStack {
			inner: self
				.inner
				.into_iter()
				.map(|e| match e {
					Operand::Int(v) => Operand::Int(v),
					Operand::Float(v) => Operand::Float(v),
					Operand::Double(v) => Operand::Double(v),
					Operand::Long(v) => Operand::Long(v),
					Operand::Reference(v) => {
						Operand::Reference(thread.jni_refs().allocate_wrapped(v))
					},
					Operand::Empty => Operand::Empty,
				})
				.collect(),
		}
	}
}

impl<REFERENCE: Clone + PartialEq + Debug> LocalStack<REFERENCE> {
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
	pub unsafe fn new_with_args(mut args: Vec<Operand<REFERENCE>>, stack_size: usize) -> Self {
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

	pub fn iter(&self) -> LocalStackIter<'_, REFERENCE> {
		self.into_iter()
	}
}

impl<'a, REFERENCE: Clone + PartialEq + Debug> IntoIterator for &'a LocalStack<REFERENCE> {
	type Item = &'a Operand<REFERENCE>;
	type IntoIter = LocalStackIter<'a, REFERENCE>;

	fn into_iter(self) -> Self::IntoIter {
		LocalStackIter {
			inner: self.inner.iter(),
			remaining: self.occupied_slots(),
		}
	}
}

pub struct LocalStackIter<'a, REFERENCE = Reference> {
	inner: std::slice::Iter<'a, Operand<REFERENCE>>,
	remaining: usize,
}

impl<'a, REFERENCE: PartialEq + Debug> Iterator for LocalStackIter<'a, REFERENCE> {
	type Item = &'a Operand<REFERENCE>;

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
				Some(operand)
			},
		}
	}
}

impl<REFERENCE: PartialEq + Debug> ExactSizeIterator for LocalStackIter<'_, REFERENCE> {
	fn len(&self) -> usize {
		self.remaining
	}
}

// Local variables are addressed by indexing. The index of the first local variable is zero.
// An integer is considered to be an index into the local variable array if and only if that integer
// is between zero and one less than the size of the local variable array.
impl<REFERENCE> Index<usize> for LocalStack<REFERENCE> {
	type Output = Operand<REFERENCE>;

	fn index(&self, index: usize) -> &Self::Output {
		&self.inner[index]
	}
}

impl<REFERENCE> IndexMut<usize> for LocalStack<REFERENCE> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		self.inner.index_mut(index)
	}
}
