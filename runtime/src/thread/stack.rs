use crate::objects::reference::Reference;
use crate::thread::exceptions::{Throws, throw};

use common::int_types::{s4, s8};
use instructions::{Operand, StackLike};
use std::alloc;
use std::alloc::Layout;
use std::fmt::Debug;
use std::ops::Deref;
use std::range::Range;

macro_rules! trace_stack {
	($operation:ident, $value:ident) => {{
		{
			tracing::trace!(
				target: "stack",
				value = ?$value,
				"{}",
				stringify!($operation),
			);
		}
	}};
	($operation:ident) => {{
		{
			tracing::trace!(target: "stack", "{}", stringify!($operation));
		}
	}};
}

/// Mutable handle to a [`ThreadStack`]
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct ThreadStackHandle {
	ptr: *mut ThreadStack,
}

impl ThreadStackHandle {
	pub fn new(ptr: *mut ThreadStack) -> ThreadStackHandle {
		ThreadStackHandle { ptr }
	}

	pub fn set(&mut self, offset: isize, value: Operand<Reference>) {
		unsafe { (*self.ptr).set(offset, value) }
	}

	pub fn set_absolute(&mut self, index: usize, value: Operand<Reference>) {
		unsafe { (*self.ptr).set_absolute(index, value) }
	}

	pub(in crate::thread) fn set_stack_pointer(&mut self, sp: usize) {
		unsafe { (*self.ptr).set_stack_pointer(sp) }
	}
}

impl ThreadStackHandle {
	pub fn clear(&self) {
		unsafe { (*self.ptr).clear() }
	}

	pub fn push_op(&self, op: Operand<Reference>) {
		unsafe { (*self.ptr).push_op(op) }
	}

	pub fn push_int(&self, int: s4) {
		unsafe { (*self.ptr).push_int(int) }
	}

	pub fn push_float(&self, float: f32) {
		unsafe { (*self.ptr).push_float(float) }
	}

	pub fn push_double(&self, double: f64) {
		unsafe { (*self.ptr).push_double(double) }
	}

	pub fn push_long(&self, long: s8) {
		unsafe { (*self.ptr).push_long(long) }
	}

	pub fn push_reference(&self, reference: Reference) {
		unsafe { (*self.ptr).push_reference(reference) }
	}

	pub fn pop(&self) -> Operand<Reference> {
		unsafe { (*self.ptr).pop() }
	}

	pub fn pop2(&self) {
		unsafe { (*self.ptr).pop2() }
	}

	pub fn popn(&self, count: usize) -> Vec<Operand<Reference>> {
		unsafe { (*self.ptr).popn(count) }
	}

	pub fn pop_int(&self) -> s4 {
		unsafe { (*self.ptr).pop_int() }
	}

	pub fn pop_float(&self) -> f32 {
		unsafe { (*self.ptr).pop_float() }
	}

	pub fn pop_double(&self) -> f64 {
		unsafe { (*self.ptr).pop_double() }
	}

	pub fn pop_long(&self) -> s8 {
		unsafe { (*self.ptr).pop_long() }
	}

	pub fn pop_reference(&self) -> Reference {
		unsafe { (*self.ptr).pop_reference() }
	}

	pub fn dup(&self) {
		unsafe { (*self.ptr).dup() }
	}

	pub fn dup_x1(&self) {
		unsafe { (*self.ptr).dup_x1() }
	}

	pub fn dup_x2(&self) {
		unsafe { (*self.ptr).dup_x2() }
	}

	pub fn dup2(&self) {
		unsafe { (*self.ptr).dup2() }
	}

	pub fn dup2_x1(&self) {
		unsafe { (*self.ptr).dup2_x1() }
	}

	pub fn dup2_x2(&self) {
		unsafe { (*self.ptr).dup2_x2() }
	}

	pub fn swap(&self) {
		unsafe { (*self.ptr).swap() }
	}
}

impl Deref for ThreadStackHandle {
	type Target = ThreadStack;

	fn deref(&self) -> &Self::Target {
		unsafe { &*self.ptr }
	}
}

impl Debug for ThreadStackHandle {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		unsafe { (*self.ptr).fmt(f) }
	}
}

/// The [operand stack] and [local stack] of a [`JavaThread`]
///
/// [operand stack]: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.6.2
/// [local stack]: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.6.1
/// [`JavaThread`]: crate::thread::JavaThread
#[derive(PartialEq)]
pub struct ThreadStack {
	cur: *mut Operand<Reference>,
	base: *mut Operand<Reference>,
	top: *mut Operand<Reference>,
}

/// An iterator over a [`ThreadStack`]
///
/// See [`ThreadStack::iter()`]
pub struct ThreadStackIter<'a> {
	stack: &'a ThreadStack,
	cur: usize,
}

impl<'a> Iterator for ThreadStackIter<'a> {
	type Item = &'a Operand<Reference>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.cur == 0 {
			return None;
		}

		// SAFETY: `self.cur` is guaranteed to be within bounds of the stack
		let ret = unsafe { self.stack.base.add(self.cur).as_ref_unchecked() };
		self.cur -= 1;

		Some(ret)
	}
}

impl Debug for ThreadStack {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.iter()).finish()
	}
}

impl Drop for ThreadStack {
	fn drop(&mut self) {
		let capacity = unsafe { self.top.offset_from_unsigned(self.base) };
		let layout = Layout::array::<Operand<Reference>>(capacity + 1).expect("valid layout");

		// SAFETY: Allocation came from `Self::new()`
		unsafe {
			alloc::dealloc(self.base as *mut u8, layout);
		}
	}
}

impl ThreadStack {
	/// Create a new [`ThreadStack`] with the specified `capacity`
	///
	/// # Exceptions
	///
	/// This may throw [`ExceptionKind::OutOfMemoryError`]
	///
	/// [`ExceptionKind::OutOfMemoryError`]: crate::thread::exceptions::ExceptionKind::OutOfMemoryError
	pub fn new(capacity: usize) -> Throws<Self> {
		// + 1 for the dummy base operand
		let element_capacity = capacity + 1;

		let layout = Layout::array::<Operand<Reference>>(element_capacity).expect("valid layout");
		let base = unsafe { alloc::alloc_zeroed(layout).cast::<Operand<Reference>>() };
		if base.is_null() {
			throw!(@DEFER OutOfMemoryError);
		}

		let top = unsafe { base.add(capacity) };
		Throws::Ok(Self {
			base,
			top,
			cur: base,
		})
	}

	/// The number of occupied slots on the stack
	///
	/// # Examples
	///
	/// ```
	/// use instructions::StackLike;
	/// use jvm::thread::stack::ThreadStack;
	///
	/// let mut stack = ThreadStack::new(15).unwrap();
	/// assert_eq!(stack.len(), 0);
	///
	/// stack.push_int(5);
	/// assert_eq!(stack.len(), 1);
	/// ```
	pub fn len(&self) -> usize {
		// SAFETY: `cur` is always >= `base`
		unsafe { self.cur.offset_from_unsigned(self.base) }
	}

	/// Returns `true` if no slots are occupied
	///
	/// # Examples
	///
	/// ```
	/// use jvm::thread::stack::ThreadStack;
	///
	/// let mut stack = ThreadStack::new(15).unwrap();
	/// assert!(stack.is_empty());
	/// ```
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	/// The amount of space remaining on the stack
	///
	/// # Examples
	///
	/// ```
	/// use instructions::StackLike;
	/// use jvm::thread::stack::ThreadStack;
	///
	/// const STACK_SIZE: usize = 2;
	///
	/// let mut stack = ThreadStack::new(STACK_SIZE).unwrap();
	/// assert_eq!(stack.remaining(), STACK_SIZE);
	///
	/// stack.push_int(1);
	/// assert_eq!(stack.remaining(), 1);
	///
	/// stack.push_int(2);
	/// assert_eq!(stack.remaining(), 0);
	/// ```
	pub fn remaining(&self) -> usize {
		// SAFETY: `top` is always guaranteed to be >= `cur`
		unsafe { self.top.offset_from_unsigned(self.cur) }
	}

	/// Get a slice of the stack from `[base..base + len]`
	///
	/// NOTE: The elements will be bottom-to-top
	///
	/// # Examples
	///
	/// ```rust
	/// use instructions::{Operand, StackLike};
	/// use jvm::thread::stack::ThreadStack;
	///
	/// let mut stack = ThreadStack::new(15).unwrap();
	/// stack.push_int(1);
	/// stack.push_int(2);
	/// stack.push_int(3);
	///
	/// let slice = stack.slice(0, 3);
	/// assert_eq!(slice, &[1, 2, 3]);
	/// ```
	pub fn slice(&self, base: usize, len: usize) -> &[Operand<Reference>] {
		assert!(base <= self.len() && len <= self.len());
		unsafe {
			// +1 since `self.base` is never a valid offset, see `Self::new()`
			let start = self.base.add(base + 1);
			std::slice::from_raw_parts(start, len)
		}
	}

	fn full(&self) -> bool {
		self.cur == self.top
	}

	fn push(&mut self, op: Operand<Reference>) {
		assert!(!self.full(), "stack overflow");
		unsafe {
			self.cur = self.cur.add(1);
			self.cur.write(op)
		};
	}

	/// Seek the stack pointer from its current position by `offset` elements
	///
	/// # Safety
	///
	/// Seeking by `offset` elements must remain within bounds.
	pub unsafe fn seek_stack_pointer(&mut self, offset: isize) {
		let new = unsafe { self.cur.offset(offset) };
		assert!(new >= self.base && new < self.top);
		self.cur = new;
	}

	/// Get the pointer to the start of the stack
	///
	/// NOTE: This does **not** point to a valid operand. The first element is always undefined.
	pub fn base(&self) -> *mut Operand<Reference> {
		self.base
	}

	/// Get the pointer to the current position in the stack
	///
	/// NOTE: This may not point to a valid operand.
	pub fn stack_pointer(&self) -> *mut Operand<Reference> {
		self.cur
	}

	/// Set the stack pointer
	pub(in crate::thread) fn set_stack_pointer(&mut self, sp: usize) {
		debug_assert!(sp <= self.len());
		self.cur = unsafe { self.base.add(sp) };
	}

	fn raw(&self, offset: isize) -> *mut Operand<Reference> {
		let offset_from_base;
		if offset.is_negative() {
			assert!(offset.unsigned_abs() <= self.len());
			offset_from_base = ((self.len() as isize) + offset) as usize;
		} else {
			assert!((offset as usize) <= self.len());
			offset_from_base = self.len() - (offset as usize) - 1;
		}

		// SAFETY: `offset` was verified to be within bounds
		unsafe {
			// +1 since `base` is always an unoccupied slot
			self.base.add(offset_from_base + 1)
		}
	}

	/// Gets the value `offset` slots down from the top of the stack
	///
	/// # Panics
	///
	/// This will panic if `offset` is out of bounds.
	///
	/// # Examples
	///
	/// ```
	/// use instructions::{Operand, StackLike};
	/// use jvm::thread::stack::ThreadStack;
	///
	/// let mut stack = ThreadStack::new(15).unwrap();
	/// stack.push_int(1);
	/// stack.push_int(2);
	/// stack.push_int(3);
	///
	/// assert_eq!(stack.at(0), Operand::Int(3));
	///
	/// // We can also use negative offsets for accesses relative
	/// // to the bottom
	/// assert_eq!(stack.at(-3), Operand::Int(1));
	/// ```
	pub fn at(&self, offset: isize) -> Operand<Reference> {
		unsafe { (&*self.raw(offset)).clone() }
	}

	/// Get the operand at `index` from the bottom of the stack
	///
	/// # Examples
	///
	/// ```
	/// use instructions::{Operand, StackLike};
	/// use jvm::thread::stack::ThreadStack;
	///
	/// let mut stack = ThreadStack::new(15).unwrap();
	/// stack.push_int(1);
	/// stack.push_int(2);
	/// stack.push_int(3);
	///
	/// assert_eq!(stack.absolute(0), Operand::Int(1));
	/// assert_eq!(stack.absolute(1), Operand::Int(2));
	/// assert_eq!(stack.absolute(2), Operand::Int(3));
	/// ```
	pub fn absolute(&self, index: usize) -> Operand<Reference> {
		assert!(index < self.len());
		unsafe { (&*self.base.add(index + 1)).clone() }
	}

	/// Set the value of the slot `index` slots down from the top of the stack
	///
	/// # Panics
	///
	/// This will panic if `index` is out of bounds.
	///
	/// # Examples
	///
	/// ```
	/// use instructions::{Operand, StackLike};
	/// use jvm::thread::stack::ThreadStack;
	///
	/// let mut stack = ThreadStack::new(15).unwrap();
	/// stack.push_int(1);
	/// stack.push_int(2);
	/// stack.push_int(3);
	///
	/// assert_eq!(stack.at(-3), Operand::Int(1));
	///
	/// stack.set(-3, Operand::Int(5));
	///
	/// assert_eq!(stack.at(-3), Operand::Int(5));
	/// ```
	pub fn set(&mut self, offset: isize, value: Operand<Reference>) {
		// SAFETY: `Self::raw()` verifies the offset
		unsafe { *self.raw(offset) = value }
	}

	/// Set the value of the slot `index` slots from the bottom
	///
	/// See [`Self::absolute()`]
	///
	/// # Panics
	///
	/// This will panic if `index` is out of bounds.
	///
	/// # Examples
	///
	/// ```
	/// use instructions::{Operand, StackLike};
	/// use jvm::thread::stack::ThreadStack;
	///
	/// let mut stack = ThreadStack::new(15).unwrap();
	/// stack.push_int(1);
	/// stack.push_int(2);
	/// stack.push_int(3);
	///
	/// assert_eq!(stack.absolute(0), Operand::Int(1));
	///
	/// stack.set(0, Operand::Int(5));
	///
	/// assert_eq!(stack.absolute(0), Operand::Int(5));
	/// ```
	pub fn set_absolute(&mut self, index: usize, value: Operand<Reference>) {
		assert!(index < self.len());
		unsafe { *self.base.add(index + 1) = value }
	}

	/// Get an iterator over the stack
	///
	/// The iterator yields items from top-to-bottom
	///
	/// # Examples
	///
	/// ```
	/// use instructions::{Operand, StackLike};
	/// use jvm::thread::stack::ThreadStack;
	///
	/// let mut stack = ThreadStack::new(3).unwrap();
	/// stack.push_int(1);
	/// stack.push_int(2);
	/// stack.push_int(3);
	///
	/// let mut iter = stack.iter();
	///
	/// // Iterates top-to-bottom
	/// assert_eq!(iter.next(), Some(&Operand::Int(3)));
	/// assert_eq!(iter.next(), Some(&Operand::Int(2)));
	/// assert_eq!(iter.next(), Some(&Operand::Int(1)));
	/// assert_eq!(iter.next(), None);
	/// ```
	pub fn iter(&self) -> ThreadStackIter<'_> {
		ThreadStackIter {
			stack: self,
			cur: self.len(),
		}
	}
}

impl StackLike<Reference> for ThreadStack {
	fn clear(&mut self) {
		self.cur = self.base;
	}

	fn push_op(&mut self, op: Operand<Reference>) {
		trace_stack!(push_op, op);
		let needs_empty = matches!(op, Operand::Long(_) | Operand::Double(_));
		self.push(op);
		if needs_empty {
			self.push(Operand::Empty)
		}
	}

	fn push_int(&mut self, int: s4) {
		trace_stack!(push_int, int);
		self.push_op(Operand::Int(int));
	}

	fn push_float(&mut self, float: f32) {
		trace_stack!(push_float, float);
		self.push_op(Operand::Float(float));
	}

	fn push_double(&mut self, double: f64) {
		trace_stack!(push_double, double);
		self.push_op(Operand::Double(double));
	}

	fn push_long(&mut self, long: s8) {
		trace_stack!(push_long, long);
		self.push_op(Operand::Long(long));
	}

	fn push_reference(&mut self, reference: Reference) {
		trace_stack!(push_reference, reference);
		self.push_op(Operand::Reference(reference))
	}

	fn pop(&mut self) -> Operand<Reference> {
		assert!(!self.is_empty(), "stack underflow");
		let op = unsafe {
			let op = *self.cur;
			self.cur = self.cur.sub(1);
			op
		};

		if op == Operand::Empty {
			trace_stack!(pop, op);
			let op = self.pop();
			trace_stack!(pop, op);
			match op {
				op if op.is_long() || op.is_double() => return op,
				_ => {
					panic!("Expected long or double to occupy stack slot!");
				},
			}
		}

		trace_stack!(pop, op);
		op
	}

	fn pop2(&mut self) {
		trace_stack!(pop2);
		self.pop();
		self.pop();
	}

	fn popn(&mut self, count: usize) -> Vec<Operand<Reference>> {
		trace_stack!(popn, count);
		if count == 0 {
			return Vec::new();
		}

		let len = self.len();
		assert!(len >= count);

		let mut current = self.cur;
		let mut operands_encountered = 0;

		while operands_encountered < count {
			if current == self.base {
				break;
			}

			// SAFETY: `cursor` is guaranteed to be within bounds
			let op = unsafe { &*current };

			// As is `cursor - 1`, which will at its lowest be `self.base`
			current = unsafe { current.sub(1) };

			match op {
				// Not a real operand, should be followed up by a Long/Double
				Operand::Empty => {},
				_ => operands_encountered += 1,
			}
		}

		if operands_encountered != count {
			panic!("stack underflow");
		}

		// SAFETY: `current` will always be <= `self.cur`
		let total_slots = unsafe { self.cur.offset_from_unsigned(current) };
		let split = unsafe { std::slice::from_raw_parts(current.add(1), total_slots) };
		let ret = split.to_vec();

		self.cur = current;
		ret
	}

	fn pop_int(&mut self) -> s4 {
		trace_stack!(pop_int);
		let op = self.pop();
		match op {
			Operand::Int(int) => int,
			_ => panic!("Unexpected operand type, wanted `int` got {:?}", op),
		}
	}

	fn pop_float(&mut self) -> f32 {
		trace_stack!(pop_float);
		let op = self.pop();
		match op {
			Operand::Float(float) => float,
			_ => panic!("Unexpected operand type, wanted `float` got {:?}", op),
		}
	}

	fn pop_double(&mut self) -> f64 {
		trace_stack!(pop_double);
		let op = self.pop();
		match op {
			Operand::Double(double) => double,
			_ => panic!("Unexpected operand type, wanted `double` got {:?}", op),
		}
	}

	fn pop_long(&mut self) -> s8 {
		trace_stack!(pop_long);
		let op = self.pop();
		match op {
			Operand::Long(long) => long,
			_ => panic!("Unexpected operand type, wanted `long` got {:?}", op),
		}
	}

	fn pop_reference(&mut self) -> Reference {
		trace_stack!(pop_reference);
		let op = self.pop();

		match op {
			Operand::Reference(ref_) => ref_,
			_ => panic!("Unexpected operand type, wanted `reference` got {:?}", op),
		}
	}

	fn dup(&mut self) {
		trace_stack!(dup);
		let value = self.pop();
		// The dup instruction must not be used unless value is a value of a category 1 computational type (§2.11.1).
		assert!(!matches!(value, Operand::Long(_) | Operand::Double(_)));

		self.push_op(value.clone());
		self.push_op(value);
	}

	fn dup_x1(&mut self) {
		trace_stack!(dup_x1);
		let value1 = self.pop();
		let value2 = self.pop();
		// The dup_x1 instruction must not be used unless both value1 and value2 are values of a category 1 computational type (§2.11.1).
		assert!(!matches!(value1, Operand::Long(_) | Operand::Double(_)));
		assert!(!matches!(value2, Operand::Long(_) | Operand::Double(_)));

		self.push_op(value1.clone());
		self.push_op(value2);
		self.push_op(value1);
	}

	fn dup_x2(&mut self) {
		let value1 = self.pop();
		let value2 = self.pop();

		// Form 1:
		//
		// ..., value3, value2, value1 →
		//
		// ..., value1, value3, value2, value1
		//
		// where value1, value2, and value3 are all values of a category 1 computational type (§2.11.1).
		if !matches!(value1, Operand::Long(_) | Operand::Double(_))
			&& !matches!(value2, Operand::Long(_) | Operand::Double(_))
		{
			let value3 = self.pop();
			self.push_op(value1.clone());
			self.push_op(value3);
			self.push_op(value2);
			self.push_op(value1);
			return;
		}

		// Form 2:
		//
		// ..., value2, value1 →
		//
		// ..., value1, value2, value1
		//
		// where value1 is a value of a category 1 computational type and value2 is a value of a category 2 computational type (§2.11.1).
		assert!(!matches!(value1, Operand::Long(_) | Operand::Double(_)));
		assert!(matches!(value2, Operand::Long(_) | Operand::Double(_)));

		self.push_op(value1.clone());
		self.push_op(value2);
		self.push_op(value1);
	}

	fn dup2(&mut self) {
		let value1 = self.pop();

		// Form 2:
		//
		// ..., value →
		//
		// ..., value, value
		//
		// where value is a value of a category 2 computational type (§2.11.1).
		if matches!(value1, Operand::Long(_) | Operand::Double(_)) {
			self.push_op(value1.clone());
			self.push_op(value1);
			return;
		}

		let value2 = self.pop();
		assert!(!matches!(value2, Operand::Long(_) | Operand::Double(_)));

		// Form 1:
		//
		// ..., value2, value1 →
		//
		// ..., value2, value1, value2, value1
		//
		// where both value1 and value2 are values of a category 1 computational type (§2.11.1).
		self.push_op(value2.clone());
		self.push_op(value1.clone());
		self.push_op(value2);
		self.push_op(value1);
	}

	fn dup2_x1(&mut self) {
		todo!()
	}

	fn dup2_x2(&mut self) {
		todo!()
	}

	fn swap(&mut self) {
		trace_stack!(swap);
		let val = self.pop();
		let val2 = self.pop();
		self.push_op(val);
		self.push_op(val2);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn new_stack(capacity: usize) -> ThreadStack {
		match ThreadStack::new(capacity) {
			Throws::Ok(stack) => stack,
			Throws::Exception(e) => panic!("Failed to allocate ThreadStack: {e}"),
		}
	}

	#[test]
	fn allocation_and_capacity() {
		let mut stack = new_stack(10);

		assert!(stack.is_empty());
		assert_eq!(stack.len(), 0);
		assert_eq!(stack.remaining(), 10);

		stack.push_int(42);
		assert!(!stack.is_empty());
		assert_eq!(stack.len(), 1);
		assert_eq!(stack.remaining(), 9);

		assert_eq!(stack.pop_int(), 42);
		assert!(stack.is_empty());
	}

	#[test]
	fn offsets() {
		let mut stack = new_stack(15);
		stack.push_int(1);
		stack.push_int(2);
		stack.push_int(3);

		// Top-to-bottom indexing
		assert_eq!(stack.at(0), Operand::Int(3));
		assert_eq!(stack.at(1), Operand::Int(2));
		assert_eq!(stack.at(2), Operand::Int(1));

		// Bottom-to-top indexing
		assert_eq!(stack.at(-3), Operand::Int(1));
		assert_eq!(stack.at(-2), Operand::Int(2));
		assert_eq!(stack.at(-1), Operand::Int(3));

		stack.set(1, Operand::Int(99)); // Should overwrite `2`
		assert_eq!(stack.at(1), Operand::Int(99));

		// Verify via pop
		assert_eq!(stack.pop_int(), 3);
		assert_eq!(stack.pop_int(), 99);
		assert_eq!(stack.pop_int(), 1);
	}

	#[test]
	fn category_2_operands() {
		let mut stack = new_stack(10);

		// Longs and Doubles should push the value AND an `Operand::Empty`
		stack.push_long(1);
		assert_eq!(stack.len(), 2);

		stack.push_double(1.23);
		assert_eq!(stack.len(), 4);

		assert_eq!(stack.pop_double(), 1.23);
		assert_eq!(stack.len(), 2);

		assert_eq!(stack.pop_long(), 1);
		assert!(stack.is_empty());
	}

	#[test]
	#[should_panic(expected = "Expected long or double to occupy stack slot!")]
	fn invalid_category_2_pop() {
		let mut stack = new_stack(10);
		stack.push_int(5);
		stack.push_op(Operand::Empty);
		stack.pop();
	}

	#[test]
	fn stack_manipulation() {
		let mut stack = new_stack(10);

		stack.push_int(5);
		stack.dup();
		assert_eq!(stack.len(), 2);
		assert_eq!(stack.pop_int(), 5);
		assert_eq!(stack.pop_int(), 5);

		stack.push_int(1);
		stack.push_int(2);
		stack.swap();
		assert_eq!(stack.pop_int(), 1);
		assert_eq!(stack.pop_int(), 2);

		// DUP_X1
		// ..., value2, value1 -> ..., value1, value2, value1
		stack.push_int(1);
		stack.push_int(2);
		stack.dup_x1();
		assert_eq!(stack.pop_int(), 2);
		assert_eq!(stack.pop_int(), 1);
		assert_eq!(stack.pop_int(), 2);

		// DUP2 (Category 1 types)
		// ..., value2, value1 -> ..., value2, value1, value2, value1
		stack.push_int(3);
		stack.push_int(4);
		stack.dup2();
		assert_eq!(stack.pop_int(), 4);
		assert_eq!(stack.pop_int(), 3);
		assert_eq!(stack.pop_int(), 4);
		assert_eq!(stack.pop_int(), 3);
	}

	#[test]
	fn popn() {
		let mut stack = new_stack(10);

		stack.push_int(2);
		stack.push_double(3.0); // Takes up 2 slots

		// Pop the top 2 logical operands (the double and the int)
		let popped = stack.popn(2);
		assert_eq!(popped.len(), 3); // 3 slots total: Empty, Double(3.0), Int(2)
		assert!(stack.is_empty());
	}

	#[test]
	#[should_panic]
	fn popn_bad() {
		let mut stack = new_stack(10);

		stack.push_double(3.0);

		// While doubles take up 2 slots, they're logically a single operand
		let popped = stack.popn(2);
	}

	#[test]
	fn clear() {
		let mut stack = new_stack(10);
		stack.push_int(1);
		stack.push_int(2);
		assert_eq!(stack.len(), 2);

		stack.clear();
		assert!(stack.is_empty());
	}

	#[test]
	fn handle() {
		let mut stack = new_stack(10);
		let handle = ThreadStackHandle::new(&raw mut stack);

		handle.push_float(1.5);
		handle.push_int(42);

		assert_eq!(handle.len(), 2);
		assert_eq!(handle.pop_int(), 42);
		assert_eq!(handle.pop_float(), 1.5);
		assert!(handle.is_empty());
	}
}
