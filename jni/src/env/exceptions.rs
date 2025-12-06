use crate::error::{JniError, Result};
use crate::objects::{JClass, JThrowable};
use crate::string::JniString;

impl super::JniEnv {
	/// Causes `exception` to be thrown
	///
	/// # Errors
	///
	/// If the VM returns an error, then the exception was likely not thrown.
	pub fn throw(&self, exception: JThrowable) -> Result<()> {
		let ret;
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).Throw)(self.0.cast::<jni_sys::JNIEnv>(), exception.raw());
		}

		if ret != 0 {
			return Err(JniError::Unknown);
		}

		Ok(())
	}

	/// Constructs an object of type `class` with the `message` specified, and causes that exception to be thrown.
	///
	/// ## Parameters
	///
	/// `class`: a subclass of `java.lang.Throwable`
	/// `message`: the message used to construct the java.lang.Throwable object
	///
	/// ## Errors
	///
	/// If the VM returns an error, then the exception was likely not thrown.
	pub fn throw_new(&self, class: JClass, message: Option<JniString>) -> Result<()> {
		let ret;
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).ThrowNew)(
				self.0.cast::<jni_sys::JNIEnv>(),
				class.raw(),
				message.map_or_else(core::ptr::null, |message| message.as_cstr().as_ptr()),
			);
		}

		if ret != 0 {
			return Err(JniError::Unknown);
		}

		Ok(())
	}
	// TODO: ExceptionOccurred

	/// Prints an exception and a backtrace of the stack to a system error-reporting channel, such as stderr.
	///
	/// This is a convenience routine provided for debugging.
	pub fn exception_describe(&self) {
		unsafe {
			let invoke_interface = self.as_native_interface();
			((*invoke_interface).ExceptionDescribe)(self.0.cast::<jni_sys::JNIEnv>());
		}
	}
	// TODO: ExceptionClear
	// TODO: FatalError

	/// Returns `true` when there is a pending exception
	pub fn exception_check(&self) -> bool {
		let ret;
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).ExceptionCheck)(self.0.cast::<jni_sys::JNIEnv>());
		}

		ret
	}
}
