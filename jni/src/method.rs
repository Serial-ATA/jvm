use crate::string::JniString;

use std::ffi::c_void;

use jni_sys::JNINativeMethod;

/// Safer wrapper around `jni_sys::JNINativeMethod`
pub struct NativeMethod {
	pub name: JniString,
	pub signature: JniString,
	pub fnPtr: *mut c_void,
}

impl From<JNINativeMethod> for NativeMethod {
	fn from(value: JNINativeMethod) -> Self {
		let name = unsafe { JniString::from_raw(value.name) };
		let signature = unsafe { JniString::from_raw(value.signature) };

		Self {
			name,
			signature,
			fnPtr: value.fnPtr,
		}
	}
}
