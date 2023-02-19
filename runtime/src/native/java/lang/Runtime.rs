use crate::native::{JNIEnv, NativeReturn};
use crate::stack::local_stack::LocalStack;

use common::int_types::{s4, s8};
use instructions::Operand;

pub fn availableProcessors(_: JNIEnv, _: LocalStack) -> NativeReturn {
	Some(Operand::Int(num_cpus::get() as s4))
}
pub fn freeMemory(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("java.lang.Runtime#freeMemory")
}
pub fn totalMemory(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("java.lang.Runtime#totalMemory")
}
pub fn maxMemory(_: JNIEnv, _: LocalStack) -> NativeReturn {
	// TODO: Xmx
	Some(Operand::Long(s8::MAX))
}
pub fn gc(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("java.lang.Runtime#gc")
}
