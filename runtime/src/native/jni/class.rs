use core::ffi::c_char;
use jni::{jboolean, jbyte, jclass, jobject, jsize, JNIEnv};

pub extern "system" fn DefineClass(
	env: *mut JNIEnv,
	name: *const c_char,
	loader: jobject,
	buf: *const jbyte,
	len: jsize,
) -> jclass {
	unimplemented!("jni::DefineClass")
}

pub extern "system" fn FindClass(env: *mut JNIEnv, name: *const c_char) -> jclass {
	unimplemented!("jni::FindClass")
}

pub extern "system" fn GetSuperclass(env: *mut JNIEnv, sub: jclass) -> jclass {
	unimplemented!("jni::GetSuperclass")
}

pub extern "system" fn IsAssignableFrom(env: *mut JNIEnv, sub: jclass, sup: jclass) -> jboolean {
	unimplemented!("jni::IsAssignableFrom")
}
