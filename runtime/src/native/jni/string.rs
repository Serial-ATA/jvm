use crate::classes;
use crate::native::jni::IntoJni;
use crate::objects::reference::Reference;

use core::ffi::c_char;
use std::{ptr, slice};

use common::unicode;
use jni::sys::{JNIEnv, jboolean, jchar, jsize, jstring};
use libc::strlen;

#[unsafe(no_mangle)]
pub unsafe extern "system" fn NewString(
	env: *mut JNIEnv,
	unicode: *const jchar,
	len: jsize,
) -> jstring {
	unimplemented!("jni::NewString");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetStringLength(env: *mut JNIEnv, str: jstring) -> jsize {
	unimplemented!("jni::GetStringLength");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetStringChars(
	env: *mut JNIEnv,
	str: jstring,
	isCopy: *mut jboolean,
) -> *const jchar {
	unimplemented!("jni::GetStringChars")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn ReleaseStringChars(
	env: *mut JNIEnv,
	str: jstring,
	chars: *const jchar,
) {
	unimplemented!("jni::ReleaseStringChars");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn NewStringUTF(env: *mut JNIEnv, utf: *const c_char) -> jstring {
	if utf.is_null() {
		return ptr::null_mut();
	}

	// SAFETY: It's entirely up to the caller to pass in a valid string
	let len = unsafe { strlen(utf) };

	// SAFETY: c_char is always 8 bits
	let utf = unsafe { slice::from_raw_parts(utf.cast::<u8>(), len) };

	let Ok(utf_8) = unicode::decode(utf) else {
		// I guess this is the best we can do?
		return ptr::null_mut();
	};

	let new_string = classes::java::lang::String::new(utf_8);
	Reference::class(new_string).into_jni() as _
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetStringUTFLength(env: *mut JNIEnv, str: jstring) -> jsize {
	unimplemented!("jni::GetStringUTFLength");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetStringUTFChars(
	env: *mut JNIEnv,
	str: jstring,
	isCopy: *mut jboolean,
) -> *const c_char {
	unimplemented!("jni::GetStringUTFChars")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn ReleaseStringUTFChars(
	env: *mut JNIEnv,
	str: jstring,
	chars: *const c_char,
) {
	unimplemented!("jni::ReleaseStringUTFChars");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetStringRegion(
	env: *mut JNIEnv,
	str: jstring,
	start: jsize,
	len: jsize,
	buf: *mut jchar,
) {
	unimplemented!("jni::GetStringRegion")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetStringUTFRegion(
	env: *mut JNIEnv,
	str: jstring,
	start: jsize,
	len: jsize,
	buf: *mut c_char,
) {
	unimplemented!("jni::GetStringUTFRegion")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetStringCritical(
	env: *mut JNIEnv,
	string: jstring,
	isCopy: *mut jboolean,
) -> *const jchar {
	unimplemented!("jni::GetStringCritical")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn ReleaseStringCritical(
	env: *mut JNIEnv,
	string: jstring,
	cstring: *const jchar,
) {
	unimplemented!("jni::ReleaseStringCritical");
}
