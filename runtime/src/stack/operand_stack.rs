use crate::reference::Reference;

use std::cmp::Ordering;
use std::ops::Neg;

use instructions::{OperandLike, StackLike};

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.6.2
#[derive(Debug, Clone, PartialEq)]
pub struct OperandStack {
	inner: Vec<Operand>,
}

impl OperandStack {
	pub fn new(capacity: usize) -> Self {
		Self {
			inner: Vec::with_capacity(capacity),
		}
	}
}

impl StackLike<Operand, Reference> for OperandStack {
	fn push_op(&mut self, op: Operand) {
		self.inner.push(op);
	}

	fn push_int(&mut self, int: i32) {
		self.inner.push(Operand::Int(int));
	}

	fn push_float(&mut self, float: f32) {
		self.inner.push(Operand::Float(float));
	}

	fn push_double(&mut self, double: f64) {
		self.inner.push(Operand::Double(double));
	}

	fn push_long(&mut self, long: i64) {
		self.inner.push(Operand::Long(long));
	}

	fn push_reference(&mut self, reference: Reference) {
		self.inner.push(Operand::Reference(reference))
	}

	fn pop(&mut self) -> Operand {
		match self.inner.pop() {
			Some(op) => op,
			_ => panic!("Stack underflow error!"),
		}
	}

	fn pop2(&mut self) {
		self.inner.pop();
		self.inner.pop();
	}

	fn pop_int(&mut self) -> i32 {
		let op = self.pop();
		match op {
			Operand::Constm1 => -1,
			Operand::Const0 => 0,
			Operand::Const1 => 1,
			Operand::Const2 => 2,
			Operand::Const3 => 3,
			Operand::Const4 => 4,
			Operand::Const5 => 5,
			Operand::Int(int) => int,
			_ => panic!("Unexpected operand type, wanted `int` got {:?}", op),
		}
	}

	fn pop_float(&mut self) -> f32 {
		let op = self.pop();
		match op {
			Operand::Constm1 => -1.0,
			Operand::Const0 => 0.0,
			Operand::Const1 => 1.0,
			Operand::Const2 => 2.0,
			Operand::Const3 => 3.0,
			Operand::Const4 => 4.0,
			Operand::Const5 => 5.0,
			Operand::Float(float) => float,
			_ => panic!("Unexpected operand type, wanted `float` got {:?}", op),
		}
	}

	fn pop_double(&mut self) -> f64 {
		let op = self.pop();
		match op {
			Operand::Constm1 => -1.0,
			Operand::Const0 => 0.0,
			Operand::Const1 => 1.0,
			Operand::Const2 => 2.0,
			Operand::Const3 => 3.0,
			Operand::Const4 => 4.0,
			Operand::Const5 => 5.0,
			Operand::Double(double) => double,
			_ => panic!("Unexpected operand type, wanted `double` got {:?}", op),
		}
	}

	fn pop_long(&mut self) -> i64 {
		let op = self.pop();
		match op {
			Operand::Constm1 => -1,
			Operand::Const0 => 0,
			Operand::Const1 => 1,
			Operand::Const2 => 2,
			Operand::Const3 => 3,
			Operand::Const4 => 4,
			Operand::Const5 => 5,
			Operand::Long(long) => long,
			_ => panic!("Unexpected operand type, wanted `long` got {:?}", op),
		}
	}

	fn dup(&mut self) {
		let top_of_stack = self.pop();
		self.inner.push(top_of_stack.clone());
		self.inner.push(top_of_stack);
	}

	fn dup_x1(&mut self) {
		todo!()
	}

	fn dup_x2(&mut self) {
		todo!()
	}

	fn dup2(&mut self) {
		todo!()
	}

	fn dup2_x1(&mut self) {
		todo!()
	}

	fn dup2_x2(&mut self) {
		todo!()
	}

	fn swap(&mut self) {
		let val = self.pop();
		let val2 = self.pop();
		self.inner.push(val);
		self.inner.push(val2);
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
	Constm1,
	Const0,
	Const1,
	Const2,
	Const3,
	Const4,
	Const5,
	Int(i32),
	Float(f32),
	Double(f64),
	Long(i64),
	Reference(Reference),
	// Used by local variable stack, both as the initial value and
	// for storing longs/doubles since those are expected to take up two indices according to spec
	Empty,
}

impl OperandLike for Operand {
	/// Add rhs to self
	fn add(&mut self, rhs: Operand) {
		match (&self, rhs) {
			(Operand::Int(lhs), Operand::Int(rhs)) => {
				*self = Operand::Int(lhs.overflowing_add(rhs).0)
			},
			(Operand::Long(lhs), Operand::Long(rhs)) => {
				*self = Operand::Long(lhs.overflowing_add(rhs).0)
			},
			(Operand::Float(lhs), Operand::Float(rhs)) => *self = Operand::Float(lhs + rhs),
			(Operand::Double(lhs), Operand::Double(rhs)) => *self = Operand::Double(lhs + rhs),
			_ => panic!("Invalid operand type for `add` instruction"),
		}
	}

	/// Subtract rhs from self
	fn sub(&mut self, rhs: Operand) {
		match (&self, rhs) {
			(Operand::Int(lhs), Operand::Int(rhs)) => {
				*self = Operand::Int(lhs.overflowing_sub(rhs).0)
			},
			(Operand::Long(lhs), Operand::Long(rhs)) => {
				*self = Operand::Long(lhs.overflowing_sub(rhs).0)
			},
			(Operand::Float(lhs), Operand::Float(rhs)) => *self = Operand::Float(lhs - rhs),
			(Operand::Double(lhs), Operand::Double(rhs)) => *self = Operand::Double(lhs - rhs),
			_ => panic!("Invalid operand type for `sub` instruction"),
		}
	}

	/// Multiply self by rhs
	fn mul(&mut self, rhs: Operand) {
		match (&self, rhs) {
			(Operand::Int(lhs), Operand::Int(rhs)) => {
				*self = Operand::Int(lhs.overflowing_mul(rhs).0)
			},
			(Operand::Long(lhs), Operand::Long(rhs)) => {
				*self = Operand::Long(lhs.overflowing_mul(rhs).0)
			},
			(Operand::Float(lhs), Operand::Float(rhs)) => *self = Operand::Float(lhs * rhs),
			(Operand::Double(lhs), Operand::Double(rhs)) => *self = Operand::Double(lhs * rhs),
			_ => panic!("Invalid operand type for `mul` instruction"),
		}
	}

	/// Divide self by rhs
	fn div(&mut self, rhs: Operand) {
		match (&self, rhs) {
			(Operand::Int(lhs), Operand::Int(rhs)) => {
				*self = Operand::Int(lhs.overflowing_div(rhs).0)
			},
			(Operand::Long(lhs), Operand::Long(rhs)) => {
				*self = Operand::Long(lhs.overflowing_div(rhs).0)
			},
			(Operand::Float(lhs), Operand::Float(rhs)) => *self = Operand::Float(lhs / rhs),
			(Operand::Double(lhs), Operand::Double(rhs)) => *self = Operand::Double(lhs / rhs),
			_ => panic!("Invalid operand type for `div` instruction"),
		}
	}

	/// Remainder of self / rhs
	fn rem(&mut self, rhs: Operand) {
		match (&self, rhs) {
			(Operand::Int(lhs), Operand::Int(rhs)) => {
				*self = Operand::Int(lhs.overflowing_rem(rhs).0)
			},
			(Operand::Long(lhs), Operand::Long(rhs)) => {
				*self = Operand::Long(lhs.overflowing_rem(rhs).0)
			},
			(Operand::Float(lhs), Operand::Float(rhs)) => *self = Operand::Float(lhs / rhs),
			(Operand::Double(lhs), Operand::Double(rhs)) => *self = Operand::Double(lhs / rhs),
			_ => panic!("Invalid operand type for `rem` instruction"),
		}
	}

	/// Negates self
	fn neg(&mut self) {
		match &self {
			Operand::Int(lhs) => *self = Operand::Int(lhs.neg()),
			Operand::Long(lhs) => *self = Operand::Long(lhs.neg()),
			Operand::Float(lhs) => *self = Operand::Float(lhs.neg()),
			Operand::Double(lhs) => *self = Operand::Double(lhs.neg()),
			_ => panic!("Invalid operand type for `neg` instruction"),
		}
	}

	/// Convert int to byte
	fn i2b(&mut self) {
		match self {
			// The value on the top of the operand stack must be of type int.
			// It is popped from the operand stack, truncated to a byte, then sign-extended to an int result.
			Operand::Int(i) => *self = Operand::Int(i32::from(*i as i8)),
			_ => panic!("Invalid operand type for `i2b` instruction: {:?}", self),
		}
	}

	/// Convert int to char
	fn i2c(&mut self) {
		match self {
			Operand::Int(i) => *self = Operand::Int(i32::from(*i as u16)),
			_ => panic!("Invalid operand type for `i2c` instruction: {:?}", self),
		}
	}

	/// Convert int to double
	fn i2d(&mut self) {
		match self {
			Operand::Int(i) => *self = Operand::Double(f64::from(*i)),
			_ => panic!("Invalid operand type for `i2d` instruction: {:?}", self),
		}
	}

	/// Convert int to float
	fn i2f(&mut self) {
		match self {
			Operand::Int(i) => *self = Operand::Float(*i as f32),
			_ => panic!("Invalid operand type for `i2f` instruction: {:?}", self),
		}
	}

	/// Convert int to long
	fn i2l(&mut self) {
		match self {
			Operand::Int(i) => *self = Operand::Long(i64::from(*i)),
			_ => panic!("Invalid operand type for `i2l` instruction: {:?}", self),
		}
	}

	/// Convert int to short
	fn i2s(&mut self) {
		match self {
			// The value on the top of the operand stack must be of type int.
			// It is popped from the operand stack, truncated to a short, then sign-extended to an int result.
			Operand::Int(i) => *self = Operand::Int(i32::from(*i as i16)),
			_ => panic!("Invalid operand type for `i2s` instruction: {:?}", self),
		}
	}

	/// Convert long to int
	fn l2i(&mut self) {
		match self {
			Operand::Long(l) => *self = Operand::Int(*l as i32),
			_ => panic!("Invalid operand type for `l2i` instruction: {:?}", self),
		}
	}

	/// Convert long to double
	fn l2d(&mut self) {
		match self {
			Operand::Long(l) => *self = Operand::Double(*l as f64),
			_ => panic!("Invalid operand type for `l2d` instruction: {:?}", self),
		}
	}

	/// Convert long to float
	fn l2f(&mut self) {
		match self {
			Operand::Long(l) => *self = Operand::Float(*l as f32),
			_ => panic!("Invalid operand type for `l2f` instruction: {:?}", self),
		}
	}

	/// Convert double to float
	fn d2f(&mut self) {
		match self {
			Operand::Double(d) => *self = Operand::Float(*d as f32),
			_ => panic!("Invalid operand type for `d2f` instruction: {:?}", self),
		}
	}

	/// Convert double to int
	fn d2i(&mut self) {
		match self {
			Operand::Double(d) => *self = Operand::Int(*d as i32),
			_ => panic!("Invalid operand type for `d2i` instruction: {:?}", self),
		}
	}

	/// Convert double to long
	fn d2l(&mut self) {
		match self {
			Operand::Double(d) => *self = Operand::Long(*d as i64),
			_ => panic!("Invalid operand type for `d2l` instruction: {:?}", self),
		}
	}

	/// Convert float to double
	fn f2d(&mut self) {
		match self {
			Operand::Float(f) => *self = Operand::Double(f64::from(*f)),
			_ => panic!("Invalid operand type for `f2d` instruction: {:?}", self),
		}
	}

	/// Convert float to int
	fn f2i(&mut self) {
		match self {
			Operand::Float(f) => *self = Operand::Int(*f as i32),
			_ => panic!("Invalid operand type for `f2i` instruction: {:?}", self),
		}
	}

	/// Convert float to long
	fn f2l(&mut self) {
		match self {
			Operand::Float(f) => *self = Operand::Long(*f as i64),
			_ => panic!("Invalid operand type for `f2l` instruction: {:?}", self),
		}
	}

	fn expect_int(&self) -> i32 {
		match self {
			Operand::Int(i) => *i,
			_ => panic!("Expected operand type `int`"),
		}
	}

	fn expect_float(&self) -> f32 {
		match self {
			Operand::Float(f) => *f,
			_ => panic!("Expected operand type `float`"),
		}
	}

	fn expect_double(&self) -> f64 {
		match self {
			Operand::Double(d) => *d,
			_ => panic!("Expected operand type `double`"),
		}
	}

	fn expect_long(&self) -> i64 {
		match self {
			Operand::Long(l) => *l,
			_ => panic!("Expected operand type `long`"),
		}
	}
}

impl PartialOrd for Operand {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match (self, other) {
			(Operand::Int(lhs), Operand::Int(rhs)) => lhs.partial_cmp(rhs),
			(Operand::Float(lhs), Operand::Float(rhs)) => lhs.partial_cmp(rhs),
			(Operand::Double(lhs), Operand::Double(rhs)) => lhs.partial_cmp(rhs),
			(Operand::Long(lhs), Operand::Long(rhs)) => lhs.partial_cmp(rhs),
			_ => panic!(
				"Invalid operand type for `cmp` instruction: {:?} cmp {:?}",
				self, other
			),
		}
	}
}
