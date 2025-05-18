mod error;

use crate::classpath::{ClassPathEntry, add_classpath_entry};
use crate::options::error::OptionsError;

use jni::java_vm::{AbortHookFn, ExitHookFn, VFPrintFHookFn};
use jni::sys::JavaVMInitArgs;
use std::ffi::{CStr, c_char, c_int, c_void};
use std::mem;

unsafe extern "C" fn vfprintf_default(_stream: *mut c_void, _format: *const c_char, ...) {
	todo!("vfprintf")
}

extern "C" fn exit_default(_status: c_int) {
	todo!("exit_default")
}

extern "C" fn abort_default() {
	todo!("abort_default")
}

pub struct Hooks {
	vfprintf: VFPrintFHookFn,
	exit: ExitHookFn,
	abort: AbortHookFn,
}

impl Default for Hooks {
	fn default() -> Self {
		Self {
			vfprintf: vfprintf_default,
			exit: exit_default,
			abort: abort_default,
		}
	}
}

pub struct JvmOptions {
	hooks: Hooks,
}

impl Default for JvmOptions {
	fn default() -> Self {
		Self {
			hooks: Hooks::default(),
		}
	}
}

impl JvmOptions {
	pub unsafe fn load(init: &JavaVMInitArgs) -> Result<Self, OptionsError> {
		let mut options = JvmOptions::default();

		for pos in 0..init.nOptions as usize {
			let option = unsafe { *init.options.add(pos) };
			let option_string = unsafe { CStr::from_ptr(option.optionString) };

			let mut opt_split = option_string.to_str().unwrap().splitn(1, "=");

			let key = opt_split.next().unwrap();

			// Special cases, no value
			match key {
				"vfprintf" => options.hooks.vfprintf = unsafe { mem::transmute(option.extraInfo) },
				"exit" => options.hooks.exit = unsafe { mem::transmute(option.extraInfo) },
				"abort" => options.hooks.abort = unsafe { mem::transmute(option.extraInfo) },
				_ => {},
			}

			let val = opt_split.next().unwrap();

			match key {
				"-Djava.class.path" => {
					for path in val.split(':') {
						add_classpath_entry(ClassPathEntry::new(path));
					}
				},
				_ => {
					if !init.ignoreUnrecognized {
						return Err(OptionsError::UnrecognizedOption(key.to_string()));
					}
				},
			}
		}

		Ok(options)
	}
}
