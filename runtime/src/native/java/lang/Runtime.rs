use crate::native::{JNIEnv, NativeReturn};
use crate::stack::local_stack::LocalStack;

pub fn availableProcessors(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("java.lang.Runtime#availableProcessors")
}
pub fn freeMemory(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("java.lang.Runtime#freeMemory")
}
pub fn totalMemory(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("java.lang.Runtime#totalMemory")
}
pub fn maxMemory(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("java.lang.Runtime#maxMemory")
}
pub fn gc(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("java.lang.Runtime#gc")
}
