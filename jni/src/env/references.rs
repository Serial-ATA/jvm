use crate::error::{JniError, Result};

use jni_sys::{JNI_OK, jint};

impl super::JniEnv {
	// TODO: PushLocalFrame
	// TODO: PopLocalFrame
	// TODO: NewGlobalRef
	// TODO: DeleteGlobalRef
	// TODO: DeleteLocalRef
	// TODO: NewLocalRef
	/// Ensures that at least `capacity` local references can be created in the current thread.
	///
	/// # Errors
	///
	/// This will error if an exception is thrown.
	///
	/// Possible exceptions:
	///
	/// * `OutOfMemoryError`: The specified `capacity` exceeds the limit
	pub fn ensure_local_capacity(&self, capacity: u32) -> Result<bool> {
		let ret;
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).EnsureLocalCapacity)(
				self.0.cast::<jni_sys::JNIEnv>(),
				std::cmp::min(capacity, i32::MAX as u32) as jint,
			);
		}

		if self.exception_check() {
			return Err(JniError::ExceptionThrown);
		}

		Ok(ret == JNI_OK)
	}
}
