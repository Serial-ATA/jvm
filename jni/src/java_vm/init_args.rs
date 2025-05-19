use crate::version::JniVersion;

use core::ffi::{c_char, c_int, c_void};
use std::ffi::CString;

#[derive(Clone)]
pub struct VmInitArgs {
	version: JniVersion,
	options: Vec<jni_sys::JavaVMOption>,
	ignore_unrecognized: bool,

	__strings: Vec<CString>,
}

impl Default for VmInitArgs {
	fn default() -> VmInitArgs {
		VmInitArgs {
			version: JniVersion::LATEST,
			options: Vec::new(),
			ignore_unrecognized: false,
			__strings: Vec::new(),
		}
	}
}

impl core::fmt::Debug for VmInitArgs {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("VmInitArgs")
			.field("version", &self.version)
			.field("options", &self.__strings)
			.field("ignore_unrecognized", &self.ignore_unrecognized)
			.finish()
	}
}

const HOOKS: [&str; 3] = ["vfprintf", "exit", "abort"];

pub type VFPrintFHookFn = unsafe extern "C" fn(stream: *mut c_void, format: *const c_char, ...);
pub type ExitHookFn = extern "C" fn(status: c_int);
pub type AbortHookFn = extern "C" fn();

impl VmInitArgs {
	/// Create a new `VmInitArgs` with the given JNI version
	pub fn new(version: JniVersion) -> Self {
		Self {
			version,
			options: Vec::new(),
			ignore_unrecognized: false,
			__strings: Vec::new(),
		}
	}

	pub fn options(mut self, options: impl IntoIterator<Item = impl Into<String>>) -> Self {
		let mut vm_options = Vec::new();

		for opt in options {
			let mut opt = opt.into();

			if opt.is_empty() || HOOKS.contains(&opt.as_str()) {
				continue;
			}

			match opt.bytes().position(|c| c == b'\0') {
				Some(index) => {
					if index != opt.len() {
						// TODO: Should this error?
						continue;
					}
				},
				None => {
					opt.push('\0');
				},
			}

			// TODO: These strings are supposed  to be in the default platform encoding
			vm_options.push(jni_sys::JavaVMOption {
				optionString: opt.as_ptr() as _,
				extraInfo: core::ptr::null_mut::<c_void>(),
			});

			// SAFETY: Checked that the only null byte present in the string is at the end.
			let s = unsafe { CString::from_vec_with_nul_unchecked(opt.into_bytes()) };
			self.__strings.push(s);
		}

		self.options = vm_options;
		self
	}

	pub unsafe fn vfprintf(mut self, hook: VFPrintFHookFn) -> Self {
		let option_string = unsafe { CString::from_vec_with_nul_unchecked(b"vfprintf\0".to_vec()) };
		let extra_info: *const c_void = unsafe { core::mem::transmute(hook) };

		self.options.push(jni_sys::JavaVMOption {
			optionString: option_string.as_ptr() as _,
			extraInfo: extra_info as _,
		});
		self.__strings.push(option_string);

		self
	}

	pub unsafe fn exit(mut self, hook: ExitHookFn) -> Self {
		let option_string = unsafe { CString::from_vec_with_nul_unchecked(b"exit\0".to_vec()) };
		let extra_info: *const c_void = unsafe { core::mem::transmute(hook) };

		self.options.push(jni_sys::JavaVMOption {
			optionString: option_string.as_ptr() as _,
			extraInfo: extra_info as _,
		});
		self.__strings.push(option_string);

		self
	}

	pub unsafe fn abort(mut self, hook: AbortHookFn) -> Self {
		let option_string = unsafe { CString::from_vec_with_nul_unchecked(b"abort\0".to_vec()) };
		let extra_info: *const c_void = unsafe { core::mem::transmute(hook) };

		self.options.push(jni_sys::JavaVMOption {
			optionString: option_string.as_ptr() as _,
			extraInfo: extra_info as _,
		});
		self.__strings.push(option_string);

		self
	}

	pub fn ignore_unrecognized(mut self, ignore_unrecognized: bool) -> Self {
		self.ignore_unrecognized = ignore_unrecognized;
		self
	}

	pub(super) fn finish(self) -> FinalizedVmInitArgs {
		let VmInitArgs {
			version,
			mut options,
			ignore_unrecognized,
			__strings,
		} = self;

		let nOptions = options.len() as jni_sys::jint;
		let options_ptr = options.as_mut_ptr();

		let java_vm_init_args = jni_sys::JavaVMInitArgs {
			version: version.into(),
			nOptions,
			options: options_ptr,
			ignoreUnrecognized: ignore_unrecognized,
		};

		FinalizedVmInitArgs {
			java_vm_init_args,
			__options: options,
			__strings,
		}
	}
}

/// A wrapper around a built [`jni_sys::JavaVMInitArgs`]
pub(super) struct FinalizedVmInitArgs {
	java_vm_init_args: jni_sys::JavaVMInitArgs,
	__options: Vec<jni_sys::JavaVMOption>,
	__strings: Vec<CString>,
}

impl FinalizedVmInitArgs {
	pub(super) fn raw(&self) -> *const jni_sys::JavaVMInitArgs {
		&self.java_vm_init_args
	}
}
