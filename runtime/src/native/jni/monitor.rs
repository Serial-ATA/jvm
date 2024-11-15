use jni::sys::{jint, jobject, JNIEnv};

#[no_mangle]
pub extern "system" fn MonitorEnter(env: *mut JNIEnv, obj: jobject) -> jint {
	unimplemented!("jni::MonitorEnter");
}

#[no_mangle]
pub extern "system" fn MonitorExit(env: *mut JNIEnv, obj: jobject) -> jint {
	unimplemented!("jni::MonitorExit");
}
