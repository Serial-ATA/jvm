use crate::error::{JniError, Result};
use crate::version::JniVersion;

impl super::JniEnv {
	/// Returns the version of the native method interface.
	///
	/// # Errors
	///
	/// If an invalid version is returned, for some reason.
	pub fn get_version(&self) -> Result<JniVersion> {
		let ret;
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).GetVersion)(self.0.cast::<jni_sys::JNIEnv>());
		}

		JniVersion::from_raw(ret).ok_or(JniError::Unknown)
	}
}
