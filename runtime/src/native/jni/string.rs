use core::ffi::c_char;
use jni::sys::{jboolean, jchar, jsize, jstring, JNIEnv};

#[no_mangle]
pub extern "system" fn NewString(env: *mut JNIEnv, unicode: *const jchar, len: jsize) -> jstring {
	unimplemented!("jni::NewString");
}

#[no_mangle]
pub extern "system" fn GetStringLength(env: *mut JNIEnv, str: jstring) -> jsize {
	unimplemented!("jni::GetStringLength");
}

#[no_mangle]
pub extern "system" fn GetStringChars(
	env: *mut JNIEnv,
	str: jstring,
	isCopy: *mut jboolean,
) -> *const jchar {
	unimplemented!("jni::GetStringChars")
}

#[no_mangle]
pub extern "system" fn ReleaseStringChars(env: *mut JNIEnv, str: jstring, chars: *const jchar) {
	unimplemented!("jni::ReleaseStringChars");
}

#[no_mangle]
pub extern "system" fn NewStringUTF(env: *mut JNIEnv, utf: *const c_char) -> jstring {
	unimplemented!("jni::NewStringUTF");
}

#[no_mangle]
pub extern "system" fn GetStringUTFLength(env: *mut JNIEnv, str: jstring) -> jsize {
	unimplemented!("jni::GetStringUTFLength");
}

#[no_mangle]
pub extern "system" fn GetStringUTFChars(
	env: *mut JNIEnv,
	str: jstring,
	isCopy: *mut jboolean,
) -> *const c_char {
	unimplemented!("jni::GetStringUTFChars")
}

#[no_mangle]
pub extern "system" fn ReleaseStringUTFChars(env: *mut JNIEnv, str: jstring, chars: *const c_char) {
	unimplemented!("jni::ReleaseStringUTFChars");
}

#[no_mangle]
pub extern "system" fn GetStringRegion(
	env: *mut JNIEnv,
	str: jstring,
	start: jsize,
	len: jsize,
	buf: *mut jchar,
) {
	unimplemented!("jni::GetStringRegion")
}

#[no_mangle]
pub extern "system" fn GetStringUTFRegion(
	env: *mut JNIEnv,
	str: jstring,
	start: jsize,
	len: jsize,
	buf: *mut c_char,
) {
	unimplemented!("jni::GetStringUTFRegion")
}

#[no_mangle]
pub extern "system" fn GetStringCritical(
	env: *mut JNIEnv,
	string: jstring,
	isCopy: *mut jboolean,
) -> *const jchar {
	unimplemented!("jni::GetStringCritical")
}

#[no_mangle]
pub extern "system" fn ReleaseStringCritical(
	env: *mut JNIEnv,
	string: jstring,
	cstring: *const jchar,
) {
	unimplemented!("jni::ReleaseStringCritical");
}
