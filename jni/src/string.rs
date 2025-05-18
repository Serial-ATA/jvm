//! Safe string wrapper that handles Java's modified UTF-8 encoding

use std::borrow::Cow;
use std::ffi::{CStr, CString, c_char};

use common::unicode;

/// A Java modified UTF-8 encoded string
///
/// See [here](https://docs.oracle.com/en/java/javase/23/docs/specs/jni/types.html#modified-utf-8-strings) for
/// information on how this differs from a standard UTF-8 string.
///
/// # Usage
///
/// ```rust
/// use jni::string::JniString;
///
/// let my_text = "Hello, world!";
/// let java_string = JniString::new(my_text);
///
/// // Will always be true, regardless of input
/// assert_eq!(my_text, java_string.as_str());
///
/// // Only true because `my_text` is an ASCII string
/// assert_eq!(my_text.as_bytes(), java_string.as_cstr().to_bytes());
/// ```
pub struct JniString {
	inner: CString,
}

impl JniString {
	/// Create a new `JavaString` from a UTF-8 string
	pub fn new(text: impl AsRef<str>) -> Self {
		Self::from(text)
	}

	/// Create a new `JavaString` from a raw C string
	///
	/// # Safety
	///
	/// * `raw` must be a valid modified UTF-8 string
	/// * `raw` must be a valid **null-terminated** C string
	///
	/// Otherwise, the behavior is undefined.
	pub unsafe fn from_raw(raw: *mut c_char) -> Self {
		Self {
			inner: unsafe { CString::from_raw(raw) },
		}
	}

	/// Get the underlying string as a UTF-8 string
	pub fn as_str(&self) -> Cow<'_, str> {
		unicode::decode(self.inner.as_bytes())
			.expect("somehow created an invalid modified UTF-8 string")
	}

	/// Get the underlying C string
	///
	/// See also: [`JniString::as_str()`]
	pub fn as_cstr(&self) -> &CStr {
		self.inner.as_c_str()
	}
}

impl<T> From<T> for JniString
where
	T: AsRef<str>,
{
	fn from(value: T) -> Self {
		let mut encoded = unicode::encode(value.as_ref()).into_owned();
		encoded.push(b'\0');

		JniString {
			inner: unsafe { CString::from_vec_with_nul_unchecked(encoded) },
		}
	}
}
