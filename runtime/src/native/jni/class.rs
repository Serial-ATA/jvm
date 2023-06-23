use super::{classref_from_jclass, jclass_from_classref};
use crate::classpath::classloader::ClassLoader;

use core::ffi::{c_char, CStr};
use std::sync::Arc;

use jni::{jboolean, jbyte, jclass, jobject, jsize, JNIEnv};
use symbols::Symbol;

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
	let name = unsafe { CStr::from_ptr(name) };

	if let Some(class) = ClassLoader::lookup_class(Symbol::intern_bytes(name.to_bytes())) {
		return jclass_from_classref(class);
	}

	return core::ptr::null() as jclass;
}

pub extern "system" unsafe fn GetSuperclass(env: *mut JNIEnv, sub: jclass) -> jclass {
	if let Some(class) = classref_from_jclass(sub) {
		if let Some(super_class) = class.super_class.map(Arc::clone) {
			return jclass_from_classref(super_class);
		}
	}

	return core::ptr::null() as jclass;
}

pub extern "system" fn IsAssignableFrom(env: *mut JNIEnv, sub: jclass, sup: jclass) -> jboolean {
	unimplemented!("jni::IsAssignableFrom")
}
