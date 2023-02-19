use crate::native::{JNIEnv, NativeReturn};
use crate::stack::local_stack::LocalStack;

pub fn latestUserDefinedLoader0(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.VM#latestUserDefinedLoader0")
}
pub fn getuid(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.VM#getuid")
}
pub fn geteuid(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.VM#geteuid")
}
pub fn getgid(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.VM#getgid")
}
pub fn getegid(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.VM#getegid")
}
pub fn getNanoTimeAdjustment(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.VM#getNanoTimeAdjustment")
}
pub fn getRuntimeArguments(_: JNIEnv, _: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.VM#getRuntimeArguments")
}
pub fn initialize(_: JNIEnv, _: LocalStack) -> NativeReturn {
	// https://github.com/openjdk/jdk/blob/7abe26935ab4356de54acee93390a0d8be1ea289/src/java.base/share/native/libjava/VM.c#L44
	None
}
