use crate::error::{JniError, Result};
use crate::objects::JObject;

use jni_sys::{JNI_OK, jint};

impl super::JniEnv {
	// TODO: PushLocalFrame
	// TODO: PopLocalFrame
	/// Creates a new global reference to the object referred to by the `obj` argument.
	///
	/// The `obj` argument may be a global or local reference.
	///
	/// ## PARAMETERS
	///
	/// `obj`: a global or local reference.
	///
	/// ## RETURNS
	///
	/// Returns a global reference to the given `obj`.
	///
	/// May return `NULL` if:
	///
	///  * `obj` refers to `null`
	///  * The system has run out of memory
	///  * `obj` was a weak global reference and has already been garbage collected
	pub fn new_global_ref(&self, obj: impl Into<JObject>) -> Result<JObject> {
		let obj = obj.into();

		let ret;
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).NewGlobalRef)(self.0.cast::<jni_sys::JNIEnv>(), obj.raw());
		}

		if self.exception_check() {
			return Err(JniError::ExceptionThrown);
		}

		Ok(unsafe { JObject::from_raw(ret) })
	}
	// TODO: DeleteGlobalRef
	/// Creates a new local reference that refers to the same object as `obj`.
	///
	/// The `obj` argument may be a global or local reference.
	///
	/// ## RETURNS
	///
	/// Returns a local reference to the given `obj`.
	///
	/// Returns `NULL` if ref refers to null.
	pub fn new_local_ref(&self, obj: impl Into<JObject>) -> Result<JObject> {
		let obj = obj.into();

		let ret;
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).NewLocalRef)(self.0.cast::<jni_sys::JNIEnv>(), obj.raw());
		}

		if self.exception_check() {
			return Err(JniError::ExceptionThrown);
		}

		Ok(unsafe { JObject::from_raw(ret) })
	}

	/// Deletes the local reference pointed to by `obj`.
	///
	/// ## PARAMETERS
	///
	/// `obj`: a local reference.
	pub fn delete_local_ref(&self, obj: impl Into<JObject>) -> Result<()> {
		let obj = obj.into();

		let _ret: ();
		unsafe {
			let invoke_interface = self.as_native_interface();
			_ret =
				((*invoke_interface).DeleteLocalRef)(self.0.cast::<jni_sys::JNIEnv>(), obj.raw());
		}

		if self.exception_check() {
			return Err(JniError::ExceptionThrown);
		}

		Ok(())
	}

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
