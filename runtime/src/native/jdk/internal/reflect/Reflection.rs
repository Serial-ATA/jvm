use crate::native::{JNIEnv, NativeReturn};
use crate::stack::local_stack::LocalStack;

pub fn getCallerClass(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.reflect.Reflection#getCallerClass")
}
pub fn getClassAccessFlags(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.reflect.Reflection#getClassAccessFlags")
}

pub fn areNestMates(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.reflect.Reflection#areNestMates")
}
