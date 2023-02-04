use crate::native::{JNIEnv, NativeReturn};
use crate::stack::local_stack::LocalStack;

pub fn fillInStackTrace(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("java.lang.Throwable#fillInStackTrace")
}
