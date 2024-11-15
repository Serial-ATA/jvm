use jni::sys::{jint, jobject, JNIEnv};

#[no_mangle]
pub extern "system" fn PushLocalFrame(env: *mut JNIEnv, capacity: jint) -> jint {
	unimplemented!("jni::PushLocalFrame");
}

#[no_mangle]
pub extern "system" fn PopLocalFrame(env: *mut JNIEnv, result: jobject) -> jobject {
	unimplemented!("jni::PopLocalFrame");
}

#[no_mangle]
pub extern "system" fn NewGlobalRef(env: *mut JNIEnv, lobj: jobject) -> jobject {
	unimplemented!("jni::NewGlobalRef");
}

#[no_mangle]
pub extern "system" fn DeleteGlobalRef(env: *mut JNIEnv, gref: jobject) {
	unimplemented!("jni::DeleteGlobalRef");
}

#[no_mangle]
pub extern "system" fn DeleteLocalRef(env: *mut JNIEnv, obj: jobject) {
	unimplemented!("jni::DeleteLocalRef");
}

#[no_mangle]
pub extern "system" fn NewLocalRef(env: *mut JNIEnv, ref_: jobject) -> jobject {
	unimplemented!("jni::NewLocalRef");
}

#[no_mangle]
pub extern "system" fn EnsureLocalCapacity(env: *mut JNIEnv, capacity: jint) -> jint {
	unimplemented!("jni::EnsureLocalCapacity");
}
