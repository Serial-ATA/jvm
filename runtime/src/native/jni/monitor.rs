use jni::sys::{JNIEnv, jint, jobject};

#[unsafe(no_mangle)]
pub extern "system" fn MonitorEnter(env: *mut JNIEnv, obj: jobject) -> jint {
	unimplemented!("jni::MonitorEnter");
}

#[unsafe(no_mangle)]
pub extern "system" fn MonitorExit(env: *mut JNIEnv, obj: jobject) -> jint {
	unimplemented!("jni::MonitorExit");
}
