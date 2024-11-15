use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::Neg;

use common::int_types::{s1, s2, s4, s8, u2, u4, u8};

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

impl<Reference: Debug + Clone> Operand<Reference> {
	/// Add rhs to self
	pub fn add(&mut self, rhs: Self) {
		if self.is_int() {
			let lhs = self.expect_int();
			let rhs = rhs.expect_int();
			*self = Operand::Int(lhs.overflowing_add(rhs).0);
			return;
		}

		if self.is_long() {
			let lhs = self.expect_long();
			let rhs = rhs.expect_long();
			*self = Operand::Long(lhs.overflowing_add(rhs).0);
			return;
		}

		if self.is_float() {
			let lhs = self.expect_float();
			let rhs = rhs.expect_float();
			*self = Operand::Float(lhs + rhs);
			return;
		}

		if self.is_double() {
			let lhs = self.expect_double();
			let rhs = rhs.expect_double();
			*self = Operand::Double(lhs + rhs);
			return;
		}

		panic!("Invalid operand type for `add` instruction");
	}

	/// Subtract rhs from self
	pub fn sub(&mut self, rhs: Self) {
		if self.is_int() {
			let lhs = self.expect_int();
			let rhs = rhs.expect_int();
			*self = Operand::Int(lhs.overflowing_sub(rhs).0);
			return;
		}

		if self.is_long() {
			let lhs = self.expect_long();
			let rhs = rhs.expect_long();
			*self = Operand::Long(lhs.overflowing_sub(rhs).0);
			return;
		}

		if self.is_float() {
			let lhs = self.expect_float();
			let rhs = rhs.expect_float();
			*self = Operand::Float(lhs - rhs);
			return;
		}

		if self.is_double() {
			let lhs = self.expect_double();
			let rhs = rhs.expect_double();
			*self = Operand::Double(lhs - rhs);
			return;
		}

		panic!("Invalid operand type for `sub` instruction");
	}

	/// Multiply self by rhs
	pub fn mul(&mut self, rhs: Self) {
		if self.is_int() {
			let lhs = self.expect_int();
			let rhs = rhs.expect_int();
			*self = Operand::Int(lhs.overflowing_mul(rhs).0);
			return;
		}

		if self.is_long() {
			let lhs = self.expect_long();
			let rhs = rhs.expect_long();
			*self = Operand::Long(lhs.overflowing_mul(rhs).0);
			return;
		}

		if self.is_float() {
			let lhs = self.expect_float();
			let rhs = rhs.expect_float();
			*self = Operand::Float(lhs * rhs);
			return;
		}

		if self.is_double() {
			let lhs = self.expect_double();
			let rhs = rhs.expect_double();
			*self = Operand::Double(lhs * rhs);
			return;
		}

		panic!("Invalid operand type for `mul` instruction");
	}

	/// Divide self by rhs
	pub fn div(&mut self, rhs: Self) {
		if self.is_int() {
			let lhs = self.expect_int();
			let rhs = rhs.expect_int();
			*self = Operand::Int(lhs.overflowing_div(rhs).0);
			return;
		}

		if self.is_long() {
			let lhs = self.expect_long();
			let rhs = rhs.expect_long();
			*self = Operand::Long(lhs.overflowing_div(rhs).0);
			return;
		}

		if self.is_float() {
			let lhs = self.expect_float();
			let rhs = rhs.expect_float();
			*self = Operand::Float(lhs / rhs);
			return;
		}

		if self.is_double() {
			let lhs = self.expect_double();
			let rhs = rhs.expect_double();
			*self = Operand::Double(lhs / rhs);
			return;
		}

		panic!("Invalid operand type for `div` instruction");
	}

	/// Remainder of self / rhs
	pub fn rem(&mut self, rhs: Self) {
		if self.is_int() {
			let lhs = self.expect_int();
			let rhs = rhs.expect_int();
			*self = Operand::Int(lhs.overflowing_rem(rhs).0);
			return;
		}

		if self.is_long() {
			let lhs = self.expect_long();
			let rhs = rhs.expect_long();
			*self = Operand::Long(lhs.overflowing_rem(rhs).0);
			return;
		}

		if self.is_float() {
			let lhs = self.expect_float();
			let rhs = rhs.expect_float();
			*self = Operand::Float(lhs % rhs);
			return;
		}

		if self.is_double() {
			let lhs = self.expect_double();
			let rhs = rhs.expect_double();
			*self = Operand::Double(lhs % rhs);
			return;
		}

		panic!("Invalid operand type for `rem` instruction");
	}

	/// Negates self
	pub fn neg(&mut self) {
		if self.is_int() {
			let value = self.expect_int();
			*self = Operand::Int(value.neg());
			return;
		}

		if self.is_long() {
			let value = self.expect_long();
			*self = Operand::Long(value.neg());
			return;
		}

		if self.is_float() {
			let value = self.expect_float();
			*self = Operand::Float(value.neg());
			return;
		}

		if self.is_double() {
			let value = self.expect_double();
			*self = Operand::Double(value.neg());
			return;
		}

		panic!("Invalid operand type for `neg` instruction");
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

	/// Bitwise AND of self and rhs
	pub fn and(&mut self, rhs: Self) {
		if self.is_int() {
			let lhs = self.expect_int();
			let rhs = rhs.expect_int();
			*self = Operand::Int(lhs & rhs);
			return;
		}

		if self.is_long() {
			let lhs = self.expect_long();
			let rhs = rhs.expect_long();
			*self = Operand::Long(lhs & rhs);
			return;
		}

		panic!("Invalid operand type for `and` instruction")
	}

	/// Bitwise OR of self and rhs
	pub fn or(&mut self, rhs: Self) {
		if self.is_int() {
			let lhs = self.expect_int();
			let rhs = rhs.expect_int();
			*self = Operand::Int(lhs | rhs);
			return;
		}

		if self.is_long() {
			let lhs = self.expect_long();
			let rhs = rhs.expect_long();
			*self = Operand::Long(lhs | rhs);
			return;
		}

		panic!("Invalid operand type for `or` instruction")
	}

	/// Logical shift right
	pub fn ushr(&mut self, rhs: Self) {
		if self.is_int() {
			let lhs = self.expect_int();
			let rhs = rhs.expect_int();
			assert!((0..32).contains(&rhs));
			*self = Operand::Int(((lhs as u4) >> (rhs & 0x1F) as u4) as s4);
			return;
		}

		if self.is_long() {
			let lhs = self.expect_long();
			let rhs = rhs.expect_int();
			assert!((0..64).contains(&rhs));
			*self = Operand::Long(((lhs as u8) >> (rhs & 0x3F) as u8) as s8);
			return;
		}

		panic!("Invalid operand type for `ushr` instruction")
	}

	/// Bitwise XOR of self and rhs
	pub fn xor(&mut self, rhs: Self) {
		if self.is_int() {
			let lhs = self.expect_int();
			let rhs = rhs.expect_int();
			*self = Operand::Int(lhs ^ rhs);
			return;
		}

		if self.is_long() {
			let lhs = self.expect_long();
			let rhs = rhs.expect_long();
			*self = Operand::Long(lhs ^ rhs);
			return;
		}

		panic!("Invalid operand type for `xor` instruction")
	}

	/// Convert int to byte
	pub fn i2b(&mut self) {
		if !self.is_int() {
			panic!("Invalid operand type for `i2b` instruction: {:?}", self);
		}

		// The value on the top of the operand stack must be of type int.
		// It is popped from the operand stack, truncated to a byte, then sign-extended to an int result.
		*self = Operand::Int(s4::from(self.expect_int() as s1));
	}

	/// Convert int to char
	pub fn i2c(&mut self) {
		if !self.is_int() {
			panic!("Invalid operand type for `i2c` instruction: {:?}", self);
		}

		*self = Operand::Int(s4::from(self.expect_int() as u2));
	}

	/// Convert int to double
	pub fn i2d(&mut self) {
		if !self.is_int() {
			panic!("Invalid operand type for `i2d` instruction: {:?}", self);
		}

		*self = Operand::Double(f64::from(self.expect_int()));
	}

	/// Convert int to float
	pub fn i2f(&mut self) {
		if !self.is_int() {
			panic!("Invalid operand type for `i2f` instruction: {:?}", self);
		}

		*self = Operand::Float(self.expect_int() as f32);
	}

	/// Convert int to long
	pub fn i2l(&mut self) {
		if !self.is_int() {
			panic!("Invalid operand type for `i2l` instruction: {:?}", self);
		}

		*self = Operand::Long(s8::from(self.expect_int()));
	}

	/// Convert int to short
	pub fn i2s(&mut self) {
		if !self.is_int() {
			panic!("Invalid operand type for `i2s` instruction: {:?}", self);
		}

		// The value on the top of the operand stack must be of type int.
		// It is popped from the operand stack, truncated to a short, then sign-extended to an int result.
		*self = Operand::Int(s4::from(self.expect_int() as s2));
	}

	/// Convert long to int
	pub fn l2i(&mut self) {
		if !self.is_long() {
			panic!("Invalid operand type for `l2i` instruction: {:?}", self);
		}

		*self = Operand::Int(self.expect_long() as s4);
	}

	/// Convert long to double
	pub fn l2d(&mut self) {
		if !self.is_long() {
			panic!("Invalid operand type for `l2d` instruction: {:?}", self);
		}

		*self = Operand::Double(self.expect_long() as f64);
	}

	/// Convert long to float
	pub fn l2f(&mut self) {
		if !self.is_long() {
			panic!("Invalid operand type for `l2f` instruction: {:?}", self);
		}

		*self = Operand::Float(self.expect_long() as f32);
	}

	/// Convert double to float
	pub fn d2f(&mut self) {
		if !self.is_double() {
			panic!("Invalid operand type for `d2f` instruction: {:?}", self);
		}

		*self = Operand::Float(self.expect_double() as f32);
	}

	/// Convert double to int
	pub fn d2i(&mut self) {
		if !self.is_double() {
			panic!("Invalid operand type for `d2i` instruction: {:?}", self);
		}

		*self = Operand::Int(self.expect_double() as s4);
	}

	/// Convert double to long
	pub fn d2l(&mut self) {
		if !self.is_double() {
			panic!("Invalid operand type for `d2l` instruction: {:?}", self);
		}

		*self = Operand::Long(self.expect_double() as s8);
	}

	/// Convert float to double
	pub fn f2d(&mut self) {
		if !self.is_float() {
			panic!("Invalid operand type for `f2d` instruction: {:?}", self);
		}

		*self = Operand::Double(f64::from(self.expect_float()));
	}

	/// Convert float to int
	pub fn f2i(&mut self) {
		if !self.is_float() {
			panic!("Invalid operand type for `f2i` instruction: {:?}", self);
		}

		*self = Operand::Int(self.expect_float() as s4);
	}

	/// Convert float to long
	pub fn f2l(&mut self) {
		if !self.is_float() {
			panic!("Invalid operand type for `f2l` instruction: {:?}", self);
		}

		*self = Operand::Long(self.expect_float() as s8);
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

	/// Unwrap an Operand of type `reference`
	pub fn expect_reference(&self) -> Reference {
		match self {
			Operand::Reference(r) => Reference::clone(r),
			_ => panic!("Expected operand type `reference`"),
		}
	}

	/// Operand is an `int`
	pub fn is_int(&self) -> bool {
		matches!(
			self,
			Self::Int(_)
				| Self::Constm1
				| Self::Const0(ConstOperandType::Int)
				| Self::Const1(ConstOperandType::Int)
				| Self::Const2(ConstOperandType::Int)
				| Self::Const3
				| Self::Const4
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

	/// Whether this operand is the same type as the other
	pub fn is_compatible_with(&self, other: &Self) -> bool {
		(self.is_int() && other.is_int())
			|| (self.is_long() && other.is_long())
			|| (self.is_float() && other.is_float())
			|| (self.is_double() && other.is_double())
			|| (self.is_reference() && other.is_reference())
	}
}

impl<Reference: Debug + PartialEq + Clone> PartialOrd for Operand<Reference> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		if self.is_int() {
			let lhs = self.expect_int();
			let rhs = other.expect_int();
			return lhs.partial_cmp(&rhs);
		}

		if self.is_long() {
			let lhs = self.expect_long();
			let rhs = other.expect_long();
			return lhs.partial_cmp(&rhs);
		}

		if self.is_float() {
			let lhs = self.expect_float();
			let rhs = other.expect_float();
			return lhs.partial_cmp(&rhs);
		}

		if self.is_double() {
			let lhs = self.expect_double();
			let rhs = other.expect_double();
			return lhs.partial_cmp(&rhs);
		}

		panic!(
			"Invalid operand type for `cmp` instruction: {:?} cmp {:?}",
			self, other
		)
	}
}

impl<Reference> From<s1> for Operand<Reference> {
	fn from(value: s1) -> Self {
		Operand::Int(value as s4)
	}
}

impl<Reference> From<s2> for Operand<Reference> {
	fn from(value: s2) -> Self {
		Operand::Int(value as s4)
	}
}

impl<Reference> From<s4> for Operand<Reference> {
	fn from(value: s4) -> Self {
		Operand::Int(value)
	}
}

impl<Reference> From<u2> for Operand<Reference> {
	fn from(value: u2) -> Self {
		Operand::Int(value as s4)
	}
}

impl<Reference> From<bool> for Operand<Reference> {
	fn from(value: bool) -> Self {
		Operand::Int(value as s4)
	}
}

impl<Reference> From<f32> for Operand<Reference> {
	fn from(value: f32) -> Self {
		Operand::Float(value)
	}
}

impl<Reference> From<f64> for Operand<Reference> {
	fn from(value: f64) -> Self {
		Operand::Double(value)
	}
}

impl<Reference> From<s8> for Operand<Reference> {
	fn from(value: s8) -> Self {
		Operand::Long(value)
	}
}
