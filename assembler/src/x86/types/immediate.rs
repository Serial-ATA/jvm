/// An immediate byte value. The imm8 symbol is a signed number between –128 and +127 inclusive.
/// For instructions in which imm8 is combined with a word or doubleword operand, the immediate value is sign-
/// extended to form a word or doubleword. The upper byte of the word is filled with the topmost bit of the
/// immediate value.
#[repr(transparent)]
pub struct Imm8(pub i8);

impl From<i8> for Imm8 {
	fn from(value: i8) -> Self {
		Imm8(value)
	}
}

/// An immediate word value used for instructions whose operand-size attribute is 16 bits. This is a
/// number between –32,768 and +32,767 inclusive.
#[repr(transparent)]
pub struct Imm16(pub i16);

impl From<i16> for Imm16 {
	fn from(value: i16) -> Self {
		Imm16(value)
	}
}

/// An immediate doubleword value used for instructions whose operand-size attribute is 32 bits.
/// It allows the use of a number between +2,147,483,647 and –2,147,483,648 inclusive.
#[repr(transparent)]
pub struct Imm32(pub i32);

impl From<i32> for Imm32 {
	fn from(value: i32) -> Self {
		Imm32(value)
	}
}

/// An immediate quadword value used for instructions whose operand-size attribute is 64 bits.
/// The value allows the use of a number between +9,223,372,036,854,775,807 and –9,223,372,036,854,775,808 inclusive.
#[repr(transparent)]
pub struct Imm64(pub i64);

impl From<i64> for Imm64 {
	fn from(value: i64) -> Self {
		Imm64(value)
	}
}
