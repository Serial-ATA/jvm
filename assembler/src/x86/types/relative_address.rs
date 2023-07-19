/// A relative address in the range from 128 bytes before the end of the instruction to 127 bytes
/// after the end of the instruction.
pub struct Rel8(pub i8);

impl From<i8> for Rel8 {
	fn from(value: i8) -> Self {
		Rel8(value)
	}
}

/// A relative address within the same code segment as the instruction assembled.
/// The rel16 symbol applies to instructions with an operand-size attribute of 16 bits
pub struct Rel16(pub i16);

impl From<i16> for Rel16 {
	fn from(value: i16) -> Self {
		Rel16(value)
	}
}

/// A relative address within the same code segment as the instruction assembled.
/// The rel32 symbol applies to instructions with an operand-size attribute of 32 bits.
pub struct Rel32(pub i32);

impl From<i32> for Rel32 {
	fn from(value: i32) -> Self {
		Rel32(value)
	}
}
