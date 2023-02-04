use crate::native::{JNIEnv, NativeReturn};
use crate::stack::local_stack::LocalStack;

use common::int_types::s4;
use instructions::Operand;

pub fn floatToRawIntBits(_: JNIEnv, locals: LocalStack) -> NativeReturn {
	let float = locals[0].expect_float();
	Some(Operand::Int(float.to_bits() as s4))
}
pub fn intBitsToFloat(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("java.lang.Float#intBitsToFloat")
}
