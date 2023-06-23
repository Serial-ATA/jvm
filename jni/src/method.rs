use std::ffi::{c_void, CString};

use jni_sys::JNINativeMethod;

/// Safer wrapper around `jni_sys::JNINativeMethod`
pub struct NativeMethod {
	pub name: CString,
	pub signature: CString,
	pub fnPtr: *mut c_void,
}

impl From<JNINativeMethod> for NativeMethod {
	fn from(value: JNINativeMethod) -> Self {
		let name = unsafe { CString::from_raw(value.name) };
		let signature = unsafe { CString::from_raw(value.signature) };

		Self {
			name,
			signature,
			fnPtr: value.fnPtr,
		}
	}
}
