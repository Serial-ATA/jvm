use crate::string::JCesu8String;

use std::ffi::c_void;

use jni_sys::JNINativeMethod;

/// Safer wrapper around `jni_sys::JNINativeMethod`
pub struct NativeMethod {
	pub name: JCesu8String,
	pub signature: JCesu8String,
	pub fnPtr: *mut c_void,
}

impl From<JNINativeMethod> for NativeMethod {
	fn from(value: JNINativeMethod) -> Self {
		let name = unsafe { JCesu8String::from_raw(value.name) };
		let signature = unsafe { JCesu8String::from_raw(value.signature) };

		Self {
			name,
			signature,
			fnPtr: value.fnPtr,
		}
	}
}
