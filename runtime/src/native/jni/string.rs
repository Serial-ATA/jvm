use core::ffi::c_char;
use jni::sys::{JNIEnv, jboolean, jchar, jsize, jstring};

#[unsafe(no_mangle)]
pub extern "system" fn NewString(env: *mut JNIEnv, unicode: *const jchar, len: jsize) -> jstring {
	unimplemented!("jni::NewString");
}

#[unsafe(no_mangle)]
pub extern "system" fn GetStringLength(env: *mut JNIEnv, str: jstring) -> jsize {
	unimplemented!("jni::GetStringLength");
}

#[unsafe(no_mangle)]
pub extern "system" fn GetStringChars(
	env: *mut JNIEnv,
	str: jstring,
	isCopy: *mut jboolean,
) -> *const jchar {
	unimplemented!("jni::GetStringChars")
}

#[unsafe(no_mangle)]
pub extern "system" fn ReleaseStringChars(env: *mut JNIEnv, str: jstring, chars: *const jchar) {
	unimplemented!("jni::ReleaseStringChars");
}

#[unsafe(no_mangle)]
pub extern "system" fn NewStringUTF(env: *mut JNIEnv, utf: *const c_char) -> jstring {
	unimplemented!("jni::NewStringUTF");
}

#[unsafe(no_mangle)]
pub extern "system" fn GetStringUTFLength(env: *mut JNIEnv, str: jstring) -> jsize {
	unimplemented!("jni::GetStringUTFLength");
}

#[unsafe(no_mangle)]
pub extern "system" fn GetStringUTFChars(
	env: *mut JNIEnv,
	str: jstring,
	isCopy: *mut jboolean,
) -> *const c_char {
	unimplemented!("jni::GetStringUTFChars")
}

#[unsafe(no_mangle)]
pub extern "system" fn ReleaseStringUTFChars(env: *mut JNIEnv, str: jstring, chars: *const c_char) {
	unimplemented!("jni::ReleaseStringUTFChars");
}

#[unsafe(no_mangle)]
pub extern "system" fn GetStringRegion(
	env: *mut JNIEnv,
	str: jstring,
	start: jsize,
	len: jsize,
	buf: *mut jchar,
) {
	unimplemented!("jni::GetStringRegion")
}

#[unsafe(no_mangle)]
pub extern "system" fn GetStringUTFRegion(
	env: *mut JNIEnv,
	str: jstring,
	start: jsize,
	len: jsize,
	buf: *mut c_char,
) {
	unimplemented!("jni::GetStringUTFRegion")
}

#[unsafe(no_mangle)]
pub extern "system" fn GetStringCritical(
	env: *mut JNIEnv,
	string: jstring,
	isCopy: *mut jboolean,
) -> *const jchar {
	unimplemented!("jni::GetStringCritical")
}

#[unsafe(no_mangle)]
pub extern "system" fn ReleaseStringCritical(
	env: *mut JNIEnv,
	string: jstring,
	cstring: *const jchar,
) {
	unimplemented!("jni::ReleaseStringCritical");
}
