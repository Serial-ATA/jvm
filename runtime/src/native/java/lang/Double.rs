use crate::native::NativeReturn;
use crate::stack::local_stack::LocalStack;

use common::int_types::{s8, u8};
use instructions::Operand;

pub fn doubleToRawLongBits(locals: LocalStack) -> NativeReturn {
	let double = locals[0].expect_double();
	Some(Operand::Long(double.to_bits() as s8))
}
pub fn longBitsToDouble(locals: LocalStack) -> NativeReturn {
	let long = locals[0].expect_long();
	Some(Operand::Double(f64::from_bits(long as u8)))
}
