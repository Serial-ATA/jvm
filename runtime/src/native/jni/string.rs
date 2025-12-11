use crate::classes;
use crate::native::java::lang::String::LATIN1;
use crate::native::jni::{IntoJni, ReferenceJniExt, reference_from_jobject};
use crate::objects::reference::Reference;
use crate::thread::JavaThread;
use crate::thread::exceptions::throw;

use core::ffi::c_char;
use std::{ptr, slice};

use ::jni::sys::{JNIEnv, jboolean, jchar, jsize, jstring};
use common::unicode;
use libc::strlen;

#[unsafe(no_mangle)]
pub unsafe extern "system" fn NewString(
	env: *mut JNIEnv,
	unicode: *const jchar,
	len: jsize,
) -> jstring {
	// SAFETY: Have the trust that the caller gave us a valid buffer
	let value = unsafe { slice::from_raw_parts(unicode, len as usize) };

	Reference::class(classes::java::lang::String::new(value)).into_jstring()
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetStringLength(env: *mut JNIEnv, str: jstring) -> jsize {
	let Some(str) = (unsafe { reference_from_jobject(str) }) else {
		panic!("GetStringLength called on null object");
	};

	classes::java::lang::String::length(str.extract_class()) as jsize
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
	let Some(str) = (unsafe { reference_from_jobject(str) }) else {
		panic!("GetStringRegion called on null object");
	};

	if len == 0 {
		// Nothing to copy
		return;
	}

	let str_instance = str.extract_class();
	let coder = classes::java::lang::String::coder(str_instance);
	let value = classes::java::lang::String::value(str_instance);
	let str_length_in_chars = classes::java::lang::String::length(str_instance);

	if start < 0 || len < 0 || start > str_length_in_chars as jsize - len {
		throw!(JavaThread::current(), StringIndexOutOfBoundsException);
	}

	if coder == LATIN1 {
		for (index, b) in value
			.as_bytes()
			.iter()
			.copied()
			.take(len as usize)
			.enumerate()
		{
			// SAFETY: Assuming the caller passed in a valid buffer >= value.len()
			unsafe { buf.add(index).write(b as jchar) };
		}
		return;
	}

	let buf = unsafe { slice::from_raw_parts_mut(buf, len as usize) };
	value
		.write_region::<jchar>(start, buf)
		.expect("preconditions already checked")
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
