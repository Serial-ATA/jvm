use common::int_types::{s4, s8};

pub trait OperandLike: PartialOrd {
	/// Add rhs to self
	fn add(&mut self, rhs: Self);

	/// Subtract rhs from self
	fn sub(&mut self, rhs: Self);

	/// Multiply self by rhs
	fn mul(&mut self, rhs: Self);

	/// Divide self by rhs
	fn div(&mut self, rhs: Self);

	/// Remainder of self / rhs
	fn rem(&mut self, rhs: Self);

	/// Negates self
	fn neg(&mut self);

	/// Convert int to byte
	fn i2b(&mut self);

	/// Convert int to char
	fn i2c(&mut self);

	/// Convert int to double
	fn i2d(&mut self);

	/// Convert int to float
	fn i2f(&mut self);

	/// Convert int to long
	fn i2l(&mut self);

	/// Convert int to short
	fn i2s(&mut self);

	/// Convert long to int
	fn l2i(&mut self);

	/// Convert long to double
	fn l2d(&mut self);

	/// Convert long to float
	fn l2f(&mut self);

	/// Convert double to float
	fn d2f(&mut self);

	/// Convert double to int
	fn d2i(&mut self);

	/// Convert double to long
	fn d2l(&mut self);

	/// Convert float to double
	fn f2d(&mut self);

	/// Convert float to int
	fn f2i(&mut self);

	/// Convert float to long
	fn f2l(&mut self);

	/// Unwrap an Operand of type `int`
	fn expect_int(&self) -> s4;

	/// Unwrap an Operand of type `long`
	fn expect_long(&self) -> s8;

	/// Unwrap an Operand of type `float`
	fn expect_float(&self) -> f32;

	/// Unwrap an Operand of type `double`
	fn expect_double(&self) -> f64;

	/// Operand is an `integer`
	fn is_int(&self) -> bool;

	/// Operand is a `long`
	fn is_long(&self) -> bool;

	/// Operand is a `float`
	fn is_float(&self) -> bool;

	/// Operand is a `double`
	fn is_double(&self) -> bool;

	/// Operand is a `reference`
	fn is_reference(&self) -> bool;
}
