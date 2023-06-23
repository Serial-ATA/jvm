use jni::sys::{jint, jobject, JNIEnv};

pub extern "system" fn PushLocalFrame(env: *mut JNIEnv, capacity: jint) -> jint {
	unimplemented!("jni::PushLocalFrame");
}

pub extern "system" fn PopLocalFrame(env: *mut JNIEnv, result: jobject) -> jobject {
	unimplemented!("jni::PopLocalFrame");
}

pub extern "system" fn NewGlobalRef(env: *mut JNIEnv, lobj: jobject) -> jobject {
	unimplemented!("jni::NewGlobalRef");
}

pub extern "system" fn DeleteGlobalRef(env: *mut JNIEnv, gref: jobject) {
	unimplemented!("jni::DeleteGlobalRef");
}

pub extern "system" fn DeleteLocalRef(env: *mut JNIEnv, obj: jobject) {
	unimplemented!("jni::DeleteLocalRef");
}

pub extern "system" fn NewLocalRef(env: *mut JNIEnv, ref_: jobject) -> jobject {
	unimplemented!("jni::NewLocalRef");
}

pub extern "system" fn EnsureLocalCapacity(env: *mut JNIEnv, capacity: jint) -> jint {
	unimplemented!("jni::EnsureLocalCapacity");
}
