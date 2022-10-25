use std::cmp::Ordering;
use std::ops::Neg;

#[derive(Debug, Copy, Clone, PartialEq)]
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
	// Used by local variable stack, both as the initial value and
	// for storing longs/doubles since those are expected to take up two indices according to spec
	Empty,
	// TODO: References
}

impl Operand {
	/// Add rhs to self
	pub fn add(&mut self, rhs: Operand) {
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
	pub fn sub(&mut self, rhs: Operand) {
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
	pub fn mul(&mut self, rhs: Operand) {
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
	pub fn div(&mut self, rhs: Operand) {
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
	pub fn rem(&mut self, rhs: Operand) {
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

	/// Convert int to byte
	pub fn i2b(&mut self) {
		match self {
			// The value on the top of the operand stack must be of type int.
			// It is popped from the operand stack, truncated to a byte, then sign-extended to an int result.
			Operand::Int(i) => *self = Operand::Int(i32::from(*i as i8)),
			_ => panic!("Invalid operand type for `i2b` instruction: {:?}", self),
		}
	}

	/// Convert int to char
	pub fn i2c(&mut self) {
		match self {
			Operand::Int(i) => *self = Operand::Int(i32::from(*i as u16)),
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
			Operand::Int(i) => *self = Operand::Long(i64::from(*i)),
			_ => panic!("Invalid operand type for `i2l` instruction: {:?}", self),
		}
	}

	/// Convert int to short
	pub fn i2s(&mut self) {
		match self {
			// The value on the top of the operand stack must be of type int.
			// It is popped from the operand stack, truncated to a short, then sign-extended to an int result.
			Operand::Int(i) => *self = Operand::Int(i32::from(*i as i16)),
			_ => panic!("Invalid operand type for `i2s` instruction: {:?}", self),
		}
	}

	/// Convert long to int
	pub fn l2i(&mut self) {
		match self {
			Operand::Long(l) => *self = Operand::Int(*l as i32),
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
			Operand::Double(d) => *self = Operand::Int(*d as i32),
			_ => panic!("Invalid operand type for `d2i` instruction: {:?}", self),
		}
	}

	/// Convert double to long
	pub fn d2l(&mut self) {
		match self {
			Operand::Double(d) => *self = Operand::Long(*d as i64),
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
			Operand::Float(f) => *self = Operand::Int(*f as i32),
			_ => panic!("Invalid operand type for `f2i` instruction: {:?}", self),
		}
	}

	/// Convert float to long
	pub fn f2l(&mut self) {
		match self {
			Operand::Float(f) => *self = Operand::Long(*f as i64),
			_ => panic!("Invalid operand type for `f2l` instruction: {:?}", self),
		}
	}

	pub fn expect_int(&self) -> i32 {
		match self {
			Operand::Int(i) => *i,
			_ => panic!("Expected operand type `int`")
		}
	}

	pub fn expect_float(&self) -> f32 {
		match self {
			Operand::Float(f) => *f,
			_ => panic!("Expected operand type `float`")
		}
	}

	pub fn expect_double(&self) -> f64 {
		match self {
			Operand::Double(d) => *d,
			_ => panic!("Expected operand type `double`")
		}
	}

	pub fn expect_long(&self) -> i64 {
		match self {
			Operand::Long(l) => *l,
			_ => panic!("Expected operand type `long`")
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
