use crate::native::jni::invocation_api::main_java_vm;
use jni::sys::{JNIEnv, JavaVM, jint};

#[unsafe(no_mangle)]
pub extern "system" fn GetJavaVM(env: *mut JNIEnv, vm: *mut *mut JavaVM) -> jint {
	if vm.is_null() {
		return -1;
	}

	unsafe { *vm = main_java_vm().raw().cast_mut() };
	return 0;
}
