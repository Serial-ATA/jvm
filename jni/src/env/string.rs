use crate::env::JniEnv;
use crate::error::{JniError, Result};
use crate::objects::JString;
use crate::string::JniString;

use std::ffi::c_char;

use jni_sys::{jchar, jsize, jstring};

/// A handle to the modified UTF-8 characters of a [`JString`]
///
/// This is created with [`JniEnv::get_string_utf_chars()`]
///
/// When dropped, this will release the memory back to the VM
pub struct StringCharsHandle<'a> {
	env: &'a JniEnv,
	source: jstring,
	ptr: *const c_char,
	len: usize,
}

impl AsRef<[u8]> for StringCharsHandle<'_> {
	fn as_ref(&self) -> &[u8] {
		unsafe { std::slice::from_raw_parts(self.ptr.cast::<u8>(), self.len) }
	}
}

impl Drop for StringCharsHandle<'_> {
	fn drop(&mut self) {
		let _ret: ();
		unsafe {
			let invoke_interface = self.env.as_native_interface();
			_ret = ((*invoke_interface).ReleaseStringUTFChars)(
				self.env.0.cast::<jni_sys::JNIEnv>(),
				self.source,
				self.ptr,
			);
		}
	}
}

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
			ret = ((*invoke_interface).NewStringUTF)(
				self.0.cast::<jni_sys::JNIEnv>(),
				utf.as_cstr().as_ptr(),
			);
		}

		if self.exception_check() {
			return Err(JniError::ExceptionThrown);
		}

		if ret.is_null() {
			return Err(JniError::Unknown);
		}

		Ok(unsafe { JString::from_raw(ret) })
	}

	/// Returns the length in bytes of the modified UTF-8 representation of a string.
	///
	/// ## PARAMETERS
	///
	/// `string`: a Java string object.
	///
	/// ## RETURNS
	///
	/// Returns the UTF-8 length of the string.
	pub fn get_string_utf_length(&self, string: JString) -> Result<jsize> {
		let ret;
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).GetStringUTFLength)(
				self.0.cast::<jni_sys::JNIEnv>(),
				string.raw(),
			);
		}

		if self.exception_check() {
			return Err(JniError::ExceptionThrown);
		}

		Ok(ret)
	}

	/// Returns the bytes representing the string in modified UTF-8 encoding.
	///
	/// ## PARAMETERS
	///
	/// `string`: a Java string object.
	///
	/// ## RETURNS
	///
	/// Returns the modified UTF-8 string.
	pub fn get_string_utf_chars(&self, string: JString) -> Result<StringCharsHandle<'_>> {
		let ret;
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).GetStringUTFChars)(
				self.0.cast::<jni_sys::JNIEnv>(),
				string.raw(),
				std::ptr::null_mut(),
			);
		}

		if self.exception_check() {
			return Err(JniError::ExceptionThrown);
		}

		let len = unsafe { libc::strlen(ret) };
		Ok(StringCharsHandle {
			env: self,
			source: string.raw(),
			ptr: ret,
			len,
		})
	}

	/// Copies `buf.len()` number of Unicode characters beginning at offset `start` into the given `buf`.
	///
	/// # Errors
	///
	/// This will error if an exception is thrown.
	///
	/// Possible exceptions:
	///
	/// * `StringIndexOutOfBoundsException`: On index overflow.
	pub fn get_string_region(&self, str: JString, start: jsize, buf: &mut [jchar]) -> Result<()> {
		let _ret: ();
		unsafe {
			let invoke_interface = self.as_native_interface();
			_ret = ((*invoke_interface).GetStringRegion)(
				self.0.cast::<jni_sys::JNIEnv>(),
				str.raw(),
				start,
				buf.len() as jsize,
				buf.as_mut_ptr(),
			);
		}

		if self.exception_check() {
			return Err(JniError::ExceptionThrown);
		}

		Ok(())
	}

	/// Translates `buf.len()` number of Unicode characters beginning at offset `start` into modified UTF-8
	/// encoding and place the result in the given buffer `buf`.
	///
	/// # Errors
	///
	/// This will error if an exception is thrown.
	///
	/// Possible exceptions:
	///
	/// * `StringIndexOutOfBoundsException`: On index overflow.
	pub fn get_string_utf_region(
		&self,
		str: JString,
		start: jsize,
		buf: &mut [c_char],
	) -> Result<()> {
		let _ret: ();
		unsafe {
			let invoke_interface = self.as_native_interface();
			_ret = ((*invoke_interface).GetStringUTFRegion)(
				self.0.cast::<jni_sys::JNIEnv>(),
				str.raw(),
				start,
				buf.len() as jsize,
				buf.as_mut_ptr(),
			);
		}

		if self.exception_check() {
			return Err(JniError::ExceptionThrown);
		}

		Ok(())
	}
	// TODO: GetStringCritical
	// TODO: ReleaseStringCritical
}
