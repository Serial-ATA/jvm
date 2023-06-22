use jni::{jint, jobject, JNIEnv};

pub extern "system" fn MonitorEnter(env: *mut JNIEnv, obj: jobject) -> jint {
	unimplemented!("jni::MonitorEnter");
}

pub extern "system" fn MonitorExit(env: *mut JNIEnv, obj: jobject) -> jint {
	unimplemented!("jni::MonitorExit");
}
