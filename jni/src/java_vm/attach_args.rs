use crate::version::JniVersion;
use jni_sys::jint;
use std::borrow::Cow;
use std::ffi::{c_char, c_void};

pub struct VmAttachArgs {
	version: JniVersion,
	name: Option<String>,
	group: Option<u32>,
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

	pub fn group(mut self, group: u32) -> Self {
		self.group = Some(group);
		self
	}

	pub(super) fn finish(self) -> FinalizedJavaVMAttachArgs {
		let VmAttachArgs {
			version,
			name,
			group,
		} = self;

		let mut __name: Option<Cow<'_, [u8]>> = None;

		let mut name_ptr = core::ptr::null_mut();
		if let Some(mut name) = name {
			match cesu8::to_java_cesu8(&*name) {
				// If owned, we need to replace `__name` with the new allocation
				c @ Cow::Owned(_) => {
					name_ptr = c.as_ptr() as _;
					__name = Some(c);
				},
				// If borrowed, `__name` needs to point to the original allocation
				Cow::Borrowed(_) => {
					name_ptr = name.as_mut_ptr();
					__name = Some(Cow::Owned(name.into_bytes()));
				},
			}
		}

		FinalizedJavaVMAttachArgs {
			version: version.into(),
			name: name_ptr as _,
			group: todo!(),
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
pub(super) struct FinalizedJavaVMAttachArgs {
	version: jint,
	name: *const c_char,
	group: jni_sys::jobject,

	__name: Option<Cow<'static, [u8]>>,
}

impl FinalizedJavaVMAttachArgs {
	#[allow(trivial_casts)]
	pub(super) fn raw(&self) -> *const c_void {
		self as *const Self as *const c_void
	}
}
