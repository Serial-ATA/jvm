use crate::string::JniString;

use std::ffi::c_void;

use jni_sys::JNINativeMethod;

/// Safer wrapper around `jni_sys::JNINativeMethod`
pub struct NativeMethod {
	pub name: JniString,
	pub signature: JniString,
	pub fnPtr: *mut c_void,
}

impl NativeMethod {
	/// Create a [`NativeMethod`] from a raw pointer
	///
	/// # Safety
	///
	/// The caller *must* ensure that the pointer provided was obtained from the VM.
	pub unsafe fn from_raw(raw: JNINativeMethod) -> Self {
		let name = unsafe { JniString::from_raw(raw.name) };
		let signature = unsafe { JniString::from_raw(raw.signature) };

		Self {
			name,
			signature,
			fnPtr: raw.fnPtr,
		}
	}
}
