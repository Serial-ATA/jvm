mod array;
mod class;
mod exceptions;
mod field;
mod method;
mod monitor;
mod nio;
mod object;
mod references;
mod reflection;
mod register;
pub use register::*;
mod string;
mod version;
mod vm;
mod weak;

/// Safer wrapper around `jni_sys::JNIEnv`
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct JniEnv(*mut jni_sys::JNIEnv);

impl JniEnv {
	pub fn raw(&self) -> *mut jni_sys::JNIEnv {
		self.0
	}

	/// Create a [`JniEnv`] from a raw pointer
	///
	/// # Safety
	///
	/// The caller *must* ensure that the pointer provided was obtained from the VM.
	pub unsafe fn from_raw(env: *mut jni_sys::JNIEnv) -> Self {
		Self(env)
	}

	unsafe fn as_native_interface(&self) -> *const jni_sys::JNINativeInterface_ {
		assert!(!self.0.is_null());

		// Assuming this was created using the safe APIs, the pointer will always be valid
		unsafe { (*self.0).cast::<jni_sys::JNINativeInterface_>() }
	}
}
