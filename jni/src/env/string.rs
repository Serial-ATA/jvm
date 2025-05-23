use crate::error::{JniError, Result};
use crate::objects::JString;
use crate::string::JniString;

impl super::JniEnv {
	// TODO: NewString
	// TODO: GetStringLength
	// TODO: GetStringChars
	// TODO: ReleaseStringChars

	/// Construct a new `java.lang.String` from a modified UTF-8 string.
	///
	/// # Parameters
	///
	/// * `utf`: The modified UTF-8 string.
	///
	/// # Errors
	///
	/// This will error if an exception is thrown.
	///
	/// Possible exceptions:
	///
	/// * `OutOfMemoryError`: The system runs out of memory.
	pub fn new_string_utf(&self, utf: impl Into<JniString>) -> Result<JString> {
		let utf = utf.into();

		let ret;
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).NewStringUTF)(self.0 as _, utf.as_cstr().as_ptr());
		}

		if self.exception_check() {
			return Err(JniError::ExceptionThrown);
		}

		if ret.is_null() {
			return Err(JniError::Unknown);
		}

		Ok(unsafe { JString::from_raw(ret) })
	}

	// TODO: GetStringUTFLength
	// TODO: GetStringUTFChars
	// TODO: ReleaseStringUTFChars
	// TODO: GetStringRegion
	// TODO: GetStringUTFRegion
	// TODO: GetStringCritical
	// TODO: ReleaseStringCritical
}
