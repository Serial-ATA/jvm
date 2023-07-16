use crate::string::JString;

use std::ffi::c_void;

use jni_sys::JNINativeMethod;

/// Safer wrapper around `jni_sys::JNINativeMethod`
pub struct NativeMethod {
	pub name: JString,
	pub signature: JString,
	pub fnPtr: *mut c_void,
}

impl From<JNINativeMethod> for NativeMethod {
	fn from(value: JNINativeMethod) -> Self {
		let name = unsafe { JString::from_raw(value.name) };
		let signature = unsafe { JString::from_raw(value.signature) };

		Self {
			name,
			signature,
			fnPtr: value.fnPtr,
		}
	}
}
