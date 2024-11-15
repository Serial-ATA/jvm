use jni::sys::{jint, JNIEnv, JavaVM};

#[no_mangle]
pub extern "system" fn GetJavaVM(env: *mut JNIEnv, vm: *mut *mut JavaVM) -> jint {
	unimplemented!("jni::GetJavaVM");
}
