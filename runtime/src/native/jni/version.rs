use jni::sys::{jint, JNIEnv};

#[no_mangle]
pub extern "system" fn GetVersion(env: *mut JNIEnv) -> jint {
	unimplemented!("jni::GetVersion")
}
