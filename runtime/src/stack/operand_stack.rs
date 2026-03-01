use crate::objects::reference::Reference;
use crate::thread::exceptions::{Throws, throw};

use std::alloc;
use std::alloc::Layout;
use std::fmt::Debug;

use common::int_types::{s4, s8};
use instructions::{Operand, StackLike};

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

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.6.2
#[derive(Clone, PartialEq)]
pub struct OperandStack {
	cur: *mut Operand<Reference>,
	base: *mut Operand<Reference>,
	top: *mut Operand<Reference>,
}

impl<'a> IntoIterator for &'a OperandStack {
	type Item = &'a Operand<Reference>;
	type IntoIter = OperandStackIter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		OperandStackIter {
			stack: self,
			cur: self.len(),
		}
	}
}

pub struct OperandStackIter<'a> {
	stack: &'a OperandStack,
	cur: usize,
}

impl<'a> Iterator for OperandStackIter<'a> {
	type Item = &'a Operand<Reference>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.cur == 0 {
			return None;
		}

		let ret = Some(unsafe { self.stack.base.add(self.cur).as_ref_unchecked() });
		self.cur -= 1;

		ret
	}
}

impl Debug for OperandStack {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self).finish()
	}
}

impl Drop for OperandStack {
	fn drop(&mut self) {
		let capacity = unsafe { self.top.offset_from_unsigned(self.base) };
		let layout = Layout::array::<Operand<Reference>>(capacity + 1).expect("valid layout");

		unsafe {
			alloc::dealloc(self.base as *mut u8, layout);
		}
	}
}

impl OperandStack {
	/// Create a new [`OperandStack`] with the specified `capacity`
	///
	/// # Exceptions
	///
	/// This may throw [`ExceptionKind::OutOfMemoryError`]
	///
	/// [`ExceptionKind::OutOfMemoryError`]: crate::thread::exceptions::ExceptionKind::OutOfMemoryError
	pub fn new(capacity: usize) -> Throws<Self> {
		let layout = Layout::array::<Operand<Reference>>(capacity + 1).expect("valid layout");
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
	/// use jvm::stack::operand_stack::OperandStack;
	///
	/// let mut stack = OperandStack::new(15).unwrap();
	/// assert_eq!(stack.len(), 0);
	///
	/// stack.push_int(5);
	/// assert_eq!(stack.len(), 1);
	/// ```
	pub fn len(&self) -> usize {
		unsafe { self.cur.offset_from(self.base) as usize }
	}

	/// Returns `true` if no slots are occupied
	///
	/// # Examples
	///
	/// ```
	/// use jvm::stack::operand_stack::OperandStack;
	///
	/// let mut stack = OperandStack::new(15).unwrap();
	/// assert!(stack.is_empty());
	/// ```
	pub fn is_empty(&self) -> bool {
		self.len() == 0
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
	/// use jvm::stack::operand_stack::OperandStack;
	///
	/// let mut stack = OperandStack::new(15).unwrap();
	/// stack.push_int(1);
	/// stack.push_int(2);
	/// stack.push_int(3);
	///
	/// assert_eq!(stack.at(3), Operand::Int(1));
	/// ```
	pub fn at(&self, offset: usize) -> Operand<Reference> {
		assert!(offset <= self.len());
		let offset_from_base = self.len() - offset;

		// SAFETY: `offset` was verified to be within bounds
		unsafe {
			// `base` is always an unoccupied slot
			let ptr = self.base.add(offset_from_base + 1);
			(&*ptr).clone()
		}
	}
}

impl StackLike<Reference> for OperandStack {
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
		self.push(Operand::Int(int));
	}

	fn push_float(&mut self, float: f32) {
		trace_stack!(push_float, float);
		self.push(Operand::Float(float));
	}

	fn push_double(&mut self, double: f64) {
		trace_stack!(push_double, double);
		self.push(Operand::Double(double));
		self.push(Operand::Empty);
	}

	fn push_long(&mut self, long: s8) {
		trace_stack!(push_long, long);
		self.push(Operand::Long(long));
		self.push(Operand::Empty);
	}

	fn push_reference(&mut self, reference: Reference) {
		trace_stack!(push_reference, reference);
		self.push(Operand::Reference(reference))
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

			// As is `cursor - 1`, which will at it lowest be `self.base`
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

		self.push(value.clone());
		self.push(value);
	}

	fn dup_x1(&mut self) {
		trace_stack!(dup_x1);
		let value1 = self.pop();
		let value2 = self.pop();
		// The dup_x1 instruction must not be used unless both value1 and value2 are values of a category 1 computational type (§2.11.1).
		assert!(!matches!(value1, Operand::Long(_) | Operand::Double(_)));
		assert!(!matches!(value2, Operand::Long(_) | Operand::Double(_)));

		self.push(value1.clone());
		self.push(value2);
		self.push(value1);
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
			self.push(value1.clone());
			self.push(value3);
			self.push(value2);
			self.push(value1);
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

		self.push(value1.clone());
		self.push(value2);
		self.push(value1);
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
			self.push(value1.clone());
			self.push(value1);
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
		self.push(value2.clone());
		self.push(value1.clone());
		self.push(value2);
		self.push(value1);
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
		self.push(val);
		self.push(val2);
	}
}
