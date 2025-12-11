mod error;

use crate::classpath::{ClassPathEntry, add_classpath_entry};
use crate::native::jdk::internal::util::SystemProps::Raw::SYSTEM_PROPERTIES;
use crate::options::error::OptionsError;

use std::ffi::{CStr, c_char, c_int, c_void};
use std::mem;

use jni::java_vm::{AbortHookFn, ExitHookFn, VFPrintFHookFn};
use jni::sys::JavaVMInitArgs;

unsafe extern "C" fn vfprintf_default(_stream: *mut c_void, _format: *const c_char, _: ...) {
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

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub enum Verbosity {
	#[default]
	Class,
	Module,
	Gc,
	Jni,
}

#[derive(Default)]
pub struct JvmOptions {
	hooks: Hooks,
	verbosity: Option<Verbosity>,
}

impl JvmOptions {
	pub unsafe fn load(init: &JavaVMInitArgs) -> Result<Self, OptionsError> {
		let mut options = JvmOptions::default();

		let mut system_props_guard = SYSTEM_PROPERTIES.lock().unwrap();
		for pos in 0..init.nOptions as usize {
			let option = unsafe { *init.options.add(pos) };
			let option_string_c = unsafe { CStr::from_ptr(option.optionString) };
			let option_string = option_string_c.to_str()?;

			let mut opt_split = option_string.splitn(2, '=');

			let key = opt_split.next().unwrap();

			// Special cases, no value
			match key {
				"vfprintf" => {
					options.hooks.vfprintf =
						unsafe { mem::transmute::<*mut c_void, VFPrintFHookFn>(option.extraInfo) }
				},
				"exit" => {
					options.hooks.exit =
						unsafe { mem::transmute::<*mut c_void, ExitHookFn>(option.extraInfo) }
				},
				"abort" => {
					options.hooks.abort =
						unsafe { mem::transmute::<*mut c_void, AbortHookFn>(option.extraInfo) }
				},
				_ if let Some(verbosity) = key.strip_prefix("-verbose") => {
					options.verbosity = Some(match verbosity.split_once(':') {
						Some((_, target)) => match target {
							"class" => Verbosity::Class,
							"module" => Verbosity::Module,
							"gc" => Verbosity::Gc,
							"jni" => Verbosity::Jni,
							_ => Verbosity::default(),
						},
						None => Verbosity::default(),
					});
				},
				_ => {},
			}

			let val = opt_split.next().unwrap();

			match key {
				"-Djava.class.path" => {
					for path in val.split(':') {
						add_classpath_entry(ClassPathEntry::new(path));
					}

					system_props_guard.insert(String::from("java.class.path"), String::from(val));
				},
				_ if let Some(system_prop) = key.strip_prefix("-D") => {
					let key = String::from(system_prop);
					let val = String::from(val);
					system_props_guard.insert(key, val);
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
