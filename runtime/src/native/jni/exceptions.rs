use core::ffi::c_char;
use jni::sys::{jboolean, jclass, jint, jthrowable, JNIEnv};

#[no_mangle]
pub extern "system" fn Throw(env: *mut JNIEnv, obj: jthrowable) -> jint {
	unimplemented!("jni::Throw");
}

#[no_mangle]
pub extern "system" fn ThrowNew(env: *mut JNIEnv, clazz: jclass, msg: *const c_char) -> jint {
	unimplemented!("jni::ThrowNew");
}

#[no_mangle]
pub extern "system" fn ExceptionOccurred(env: *mut JNIEnv) -> jthrowable {
	unimplemented!("jni::ExceptionOccurred");
}

#[no_mangle]
pub extern "system" fn ExceptionDescribe(env: *mut JNIEnv) {
	unimplemented!("jni::ExceptionDescribe");
}

#[no_mangle]
pub extern "system" fn ExceptionClear(env: *mut JNIEnv) {
	unimplemented!("jni::ExceptionClear");
}

#[no_mangle]
pub extern "system" fn FatalError(env: *mut JNIEnv, msg: *const c_char) -> ! {
	unimplemented!("jni::FatalError");
}

#[no_mangle]
pub extern "system" fn ExceptionCheck(env: *mut JNIEnv) -> jboolean {
	unimplemented!("jni::ExceptionCheck");
}
