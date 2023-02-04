use crate::native::{JNIEnv, NativeReturn};
use crate::stack::local_stack::LocalStack;

use common::int_types::s4;
use instructions::Operand;

pub fn isBigEndian(_: JNIEnv, _: LocalStack) -> NativeReturn {
	Some(Operand::Int(s4::from(cfg!(target_endian = "big"))))
}
