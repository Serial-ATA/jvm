use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::Neg;

use common::int_types::{s1, s2, s4, s8, u2};

#[derive(Debug, Clone, PartialEq)]
pub enum ConstOperandType {
	Int,
	Long,
	Float,
	Double,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand<Reference> {
	Constm1,
	Const0(ConstOperandType),
	Const1(ConstOperandType),
	Const2(ConstOperandType),
	Const3,
	Const4,
	Const5,
	Int(s4),
	Float(f32),
	Double(f64),
	Long(s8),
	Reference(Reference),
	// Used by local variable stack, both as the initial value and
	// for storing longs/doubles since those are expected to take up two indices according to spec
	Empty,
}

impl<Reference: Debug> Operand<Reference> {
	/// Add rhs to self
	pub fn add(&mut self, rhs: Self) {
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
	pub fn sub(&mut self, rhs: Self) {
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
	pub fn mul(&mut self, rhs: Self) {
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
	pub fn div(&mut self, rhs: Self) {
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
	pub fn rem(&mut self, rhs: Self) {
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
	pub fn neg(&mut self) {
		match &self {
			Operand::Int(lhs) => *self = Operand::Int(lhs.neg()),
			Operand::Long(lhs) => *self = Operand::Long(lhs.neg()),
			Operand::Float(lhs) => *self = Operand::Float(lhs.neg()),
			Operand::Double(lhs) => *self = Operand::Double(lhs.neg()),
			_ => panic!("Invalid operand type for `neg` instruction"),
		}
	}

	/// Shifts self left
	pub fn shl(&mut self, rhs: Self) {
		let rhs = rhs.expect_int();

		if self.is_int() {
			let lhs = self.expect_int();
			assert!((0..32).contains(&rhs));
			*self = Operand::Int(lhs << rhs);
			return;
		}

		if self.is_long() {
			let lhs = self.expect_long();
			assert!((0..64).contains(&rhs));
			*self = Operand::Long(lhs << s8::from(rhs));
			return;
		}

		panic!("Invalid operand type for `shl` instruction")
	}

	/// Shifts self right
	pub fn shr(&mut self, rhs: Self) {
		let rhs = rhs.expect_int();

		if self.is_int() {
			let lhs = self.expect_int();
			assert!((0..32).contains(&rhs));
			*self = Operand::Int(lhs >> rhs);
			return;
		}

		if self.is_long() {
			let lhs = self.expect_long();
			assert!((0..64).contains(&rhs));
			*self = Operand::Long(lhs >> s8::from(rhs));
			return;
		}

		panic!("Invalid operand type for `shr` instruction")
	}

	/// Convert int to byte
	pub fn i2b(&mut self) {
		match self {
			// The value on the top of the operand stack must be of type int.
			// It is popped from the operand stack, truncated to a byte, then sign-extended to an int result.
			Operand::Int(i) => *self = Operand::Int(s4::from(*i as s1)),
			_ => panic!("Invalid operand type for `i2b` instruction: {:?}", self),
		}
	}

	/// Convert int to char
	pub fn i2c(&mut self) {
		match self {
			Operand::Int(i) => *self = Operand::Int(s4::from(*i as u2)),
			_ => panic!("Invalid operand type for `i2c` instruction: {:?}", self),
		}
	}

	/// Convert int to double
	pub fn i2d(&mut self) {
		match self {
			Operand::Int(i) => *self = Operand::Double(f64::from(*i)),
			_ => panic!("Invalid operand type for `i2d` instruction: {:?}", self),
		}
	}

	/// Convert int to float
	pub fn i2f(&mut self) {
		match self {
			Operand::Int(i) => *self = Operand::Float(*i as f32),
			_ => panic!("Invalid operand type for `i2f` instruction: {:?}", self),
		}
	}

	/// Convert int to long
	pub fn i2l(&mut self) {
		match self {
			Operand::Int(i) => *self = Operand::Long(s8::from(*i)),
			_ => panic!("Invalid operand type for `i2l` instruction: {:?}", self),
		}
	}

	/// Convert int to short
	pub fn i2s(&mut self) {
		match self {
			// The value on the top of the operand stack must be of type int.
			// It is popped from the operand stack, truncated to a short, then sign-extended to an int result.
			Operand::Int(i) => *self = Operand::Int(s4::from(*i as s2)),
			_ => panic!("Invalid operand type for `i2s` instruction: {:?}", self),
		}
	}

	/// Convert long to int
	pub fn l2i(&mut self) {
		match self {
			Operand::Long(l) => *self = Operand::Int(*l as s4),
			_ => panic!("Invalid operand type for `l2i` instruction: {:?}", self),
		}
	}

	/// Convert long to double
	pub fn l2d(&mut self) {
		match self {
			Operand::Long(l) => *self = Operand::Double(*l as f64),
			_ => panic!("Invalid operand type for `l2d` instruction: {:?}", self),
		}
	}

	/// Convert long to float
	pub fn l2f(&mut self) {
		match self {
			Operand::Long(l) => *self = Operand::Float(*l as f32),
			_ => panic!("Invalid operand type for `l2f` instruction: {:?}", self),
		}
	}

	/// Convert double to float
	pub fn d2f(&mut self) {
		match self {
			Operand::Double(d) => *self = Operand::Float(*d as f32),
			_ => panic!("Invalid operand type for `d2f` instruction: {:?}", self),
		}
	}

	/// Convert double to int
	pub fn d2i(&mut self) {
		match self {
			Operand::Double(d) => *self = Operand::Int(*d as s4),
			_ => panic!("Invalid operand type for `d2i` instruction: {:?}", self),
		}
	}

	/// Convert double to long
	pub fn d2l(&mut self) {
		match self {
			Operand::Double(d) => *self = Operand::Long(*d as s8),
			_ => panic!("Invalid operand type for `d2l` instruction: {:?}", self),
		}
	}

	/// Convert float to double
	pub fn f2d(&mut self) {
		match self {
			Operand::Float(f) => *self = Operand::Double(f64::from(*f)),
			_ => panic!("Invalid operand type for `f2d` instruction: {:?}", self),
		}
	}

	/// Convert float to int
	pub fn f2i(&mut self) {
		match self {
			Operand::Float(f) => *self = Operand::Int(*f as s4),
			_ => panic!("Invalid operand type for `f2i` instruction: {:?}", self),
		}
	}

	/// Convert float to long
	pub fn f2l(&mut self) {
		match self {
			Operand::Float(f) => *self = Operand::Long(*f as s8),
			_ => panic!("Invalid operand type for `f2l` instruction: {:?}", self),
		}
	}

	/// Unwrap an Operand of type `int`
	pub fn expect_int(&self) -> s4 {
		match self {
			Operand::Constm1 => -1,
			Operand::Const0(ConstOperandType::Int) => 0,
			Operand::Const1(ConstOperandType::Int) => 1,
			Operand::Const2(ConstOperandType::Int) => 2,
			Operand::Const3 => 3,
			Operand::Const4 => 4,
			Operand::Const5 => 5,
			Operand::Int(i) => *i,
			_ => panic!("Expected operand type `int`"),
		}
	}

	/// Unwrap an Operand of type `long`
	pub fn expect_long(&self) -> s8 {
		match self {
			Operand::Const0(ConstOperandType::Long) => 0,
			Operand::Const1(ConstOperandType::Long) => 1,
			Operand::Long(l) => *l,
			_ => panic!("Expected operand type `long`"),
		}
	}

	/// Unwrap an Operand of type `float`
	pub fn expect_float(&self) -> f32 {
		match self {
			Operand::Const0(ConstOperandType::Float) => 0.,
			Operand::Const1(ConstOperandType::Float) => 1.,
			Operand::Const2(ConstOperandType::Float) => 2.,
			Operand::Float(f) => *f,
			_ => panic!("Expected operand type `float`"),
		}
	}

	/// Unwrap an Operand of type `double`
	pub fn expect_double(&self) -> f64 {
		match self {
			Operand::Const0(ConstOperandType::Double) => 0.,
			Operand::Const1(ConstOperandType::Double) => 1.,
			Operand::Double(d) => *d,
			_ => panic!("Expected operand type `double`"),
		}
	}

	/// Operand is an `int`
	pub fn is_int(&self) -> bool {
		matches!(
			self,
			Self::Int(_)
				| Self::Constm1 | Self::Const0(ConstOperandType::Int)
				| Self::Const1(ConstOperandType::Int)
				| Self::Const2(ConstOperandType::Int)
				| Self::Const3 | Self::Const4
				| Self::Const5
		)
	}

	/// Operand is a `long`
	pub fn is_long(&self) -> bool {
		matches!(
			self,
			Self::Long(_)
				| Self::Const0(ConstOperandType::Long)
				| Self::Const1(ConstOperandType::Long)
		)
	}

	/// Operand is a `float`
	pub fn is_float(&self) -> bool {
		matches!(
			self,
			Self::Float(_)
				| Self::Const0(ConstOperandType::Float)
				| Self::Const1(ConstOperandType::Float)
				| Self::Const2(ConstOperandType::Float)
		)
	}

	/// Operand is a `double`
	pub fn is_double(&self) -> bool {
		matches!(
			self,
			Self::Double(_)
				| Self::Const0(ConstOperandType::Double)
				| Self::Const1(ConstOperandType::Double)
		)
	}

	/// Operand is a `reference`
	pub fn is_reference(&self) -> bool {
		matches!(self, Self::Reference(_))
	}
}

impl<Reference: Debug + PartialEq> PartialOrd for Operand<Reference> {
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
