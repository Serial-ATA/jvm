use crate::objects::JObject;
use crate::sys::jint;
use crate::version::JniVersion;

use std::borrow::Cow;
use std::ffi::{c_char, c_void};
use std::ptr;

use common::unicode;

pub struct VmAttachArgs {
	version: JniVersion,
	name: Option<String>,
	group: Option<JObject>,
}

impl VmAttachArgs {
	pub fn new(version: JniVersion) -> Self {
		Self {
			version,
			name: None,
			group: None,
		}
	}

	pub fn name(mut self, name: String) -> Self {
		self.name = Some(name);
		self
	}

	pub fn group(mut self, group: JObject) -> Self {
		self.group = Some(group);
		self
	}

	pub(super) fn finish(self) -> FinalizedJavaVMAttachArgs {
		let VmAttachArgs {
			version,
			name,
			group,
		} = self;

		let mut __name = None;

		let mut name_ptr = ptr::null();
		if let Some(name) = name {
			match unicode::encode(name.as_str()) {
				// If owned, we need to replace `__name` with the new allocation
				Cow::Owned(value) => {
					name_ptr = value.as_ptr();
					__name = Some(value);
				},
				// If borrowed, `__name` needs to point to the original allocation
				Cow::Borrowed(_) => {
					name_ptr = name.as_ptr();
					__name = Some(name.into_bytes());
				},
			}
		}

		let group = match group {
			None => ptr::null_mut(),
			Some(group) => group.raw(),
		};

		FinalizedJavaVMAttachArgs {
			version: version.into(),
			name: name_ptr as _,
			group,
			__name,
		}
	}
}

/// A wrapper around a built [`VmAttachArgs`]
///
/// Structure:
///
/// ```c
/// typedef struct JavaVMAttachArgs {
///     jint version;
///     char *name;    /* the name of the thread as a modified UTF-8 string, or NULL */
///     jobject group; /* global ref of a ThreadGroup object, or NULL */
/// } JavaVMAttachArgs
/// ```
#[expect(dead_code)]
pub(super) struct FinalizedJavaVMAttachArgs {
	version: jint,
	name: *const c_char,
	group: jni_sys::jobject,

	__name: Option<Vec<u8>>,
}

impl FinalizedJavaVMAttachArgs {
	#[allow(trivial_casts)]
	pub(super) fn raw(&self) -> *const c_void {
		self as *const Self as *const c_void
	}
}
