use core::ffi::c_char;
use jni::sys::{jboolean, jclass, jint, jthrowable, JNIEnv};

pub extern "system" fn Throw(env: *mut JNIEnv, obj: jthrowable) -> jint {
	unimplemented!("jni::Throw");
}

pub extern "system" fn ThrowNew(env: *mut JNIEnv, clazz: jclass, msg: *const c_char) -> jint {
	unimplemented!("jni::ThrowNew");
}

pub extern "system" fn ExceptionOccurred(env: *mut JNIEnv) -> jthrowable {
	unimplemented!("jni::ExceptionOccurred");
}

pub extern "system" fn ExceptionDescribe(env: *mut JNIEnv) {
	unimplemented!("jni::ExceptionDescribe");
}

pub extern "system" fn ExceptionClear(env: *mut JNIEnv) {
	unimplemented!("jni::ExceptionClear");
}

pub extern "system" fn FatalError(env: *mut JNIEnv, msg: *const c_char) -> ! {
	unimplemented!("jni::FatalError");
}

pub extern "system" fn ExceptionCheck(env: *mut JNIEnv) -> jboolean {
	unimplemented!("jni::ExceptionCheck");
}
