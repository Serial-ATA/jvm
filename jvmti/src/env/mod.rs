/// Safer wrapper around [`jvmti_sys::jvmtiEnv`]
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct JvmtiEnv(*mut jvmti_sys::jvmtiEnv);

impl JvmtiEnv {
	/// Get the inner pointer to this JVMTI env
	pub fn raw(&self) -> *mut jvmti_sys::jvmtiEnv {
		self.0
	}

	/// Create a [`JvmtiEnv`] from a raw pointer
	///
	/// # Safety
	///
	/// The caller *must* ensure that the pointer provided was obtained from the VM.
	pub unsafe fn from_raw(env: *mut jvmti_sys::jvmtiEnv) -> Self {
		Self(env)
	}
}
