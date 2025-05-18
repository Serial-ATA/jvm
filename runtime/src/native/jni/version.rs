use jni::sys::{JNIEnv, jint};

#[unsafe(no_mangle)]
pub extern "system" fn GetVersion(env: *mut JNIEnv) -> jint {
	unimplemented!("jni::GetVersion")
}
