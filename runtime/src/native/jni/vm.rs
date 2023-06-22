use jni::{jint, JNIEnv, JavaVM};

pub extern "system" fn GetJavaVM(env: *mut JNIEnv, vm: *mut *mut JavaVM) -> jint {
	unimplemented!("jni::GetJavaVM");
}
