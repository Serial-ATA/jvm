//! Safe string wrapper that handles Java's modified UTF-8 encoding

use std::borrow::Cow;
use std::ffi::{c_char, CStr, CString};

pub struct JString {
	inner: CString,
}

impl JString {
	pub unsafe fn from_raw(raw: *mut c_char) -> Self {
		Self {
			inner: unsafe { CString::from_raw(raw) },
		}
	}

	pub fn as_str(&self) -> Cow<'_, str> {
		cesu8::from_java_cesu8(self.inner.as_bytes()).expect("TODO: handle invalid encoding")
	}

	pub fn as_cstr(&self) -> &CStr {
		self.inner.as_c_str()
	}
}

impl<T> From<T> for JString
where
	T: AsRef<str>,
{
	fn from(value: T) -> Self {
		let encoded = cesu8::to_java_cesu8(value.as_ref()).into_owned();
		JString {
			inner: unsafe { CString::from_vec_with_nul_unchecked(encoded) },
		}
	}
}
