use jni::sys::{jint, JNIEnv};

pub extern "system" fn GetVersion(env: *mut JNIEnv) -> jint {
	unimplemented!("jni::GetVersion")
}
