use crate::logging::info;
use crate::thread::JavaThread;
use crate::thread::exceptions::throw_with_ret;

use std::ffi::{CStr, OsStr, c_char, c_void};

use jni::sys::jboolean;
use native_macros::jni_call;

#[jni_call(no_env, no_strict_types)]
pub extern "C" fn JVM_LoadZipLibrary() -> *mut c_void {
	todo!()
}

#[jni_call(no_env, no_strict_types)]
pub extern "C" fn JVM_LoadLibrary(name: *const c_char, throw_exception: jboolean) -> *mut c_void {
	let thread = JavaThread::current();

	let name_c = unsafe { CStr::from_ptr(name) };
	let name_os = unsafe { OsStr::from_encoded_bytes_unchecked(name_c.to_bytes()) };

	match platform::libs::Library::load(name_os) {
		Ok(lib) => {
			info!(
				TARGETS: (Library), "Loaded library {}, handle {:016p}",
				name_c.to_string_lossy(),
				lib.raw()
			);

			lib.raw()
		},
		Err(e) => {
			info!(
				TARGETS: (Library), "Failed to load library {}",
				name_c.to_string_lossy(),
			);

			if throw_exception {
				throw_with_ret!(
					std::ptr::null_mut(),
					thread,
					UnsatisfiedLinkError,
					"{}: {e}",
					name_os.as_encoded_bytes().escape_ascii()
				);
			}

			std::ptr::null_mut()
		},
	}
}

#[jni_call(no_env, no_strict_types)]
pub extern "C" fn JVM_UnloadLibrary(handle: *mut c_void) {
	let lib = unsafe { platform::libs::Library::from_raw(handle) };
	let ptr = lib.raw();
	let _ = lib.close();

	info!(
		TARGETS: (Library), "Unloaded library with handle {ptr:016p}",
	);
}

#[jni_call(no_env, no_strict_types)]
pub extern "C" fn JVM_FindLibraryEntry(handle: *mut c_void, name: *const c_char) -> *mut c_void {
	let lib = unsafe { platform::libs::Library::from_raw(handle) };
	let name_c = unsafe { CStr::from_ptr(name) };

	let ret = match unsafe { lib.symbol::<c_void>(&name_c) } {
		Ok(sym) => sym.raw(),
		Err(_) => std::ptr::null_mut(),
	};

	info!(
		TARGETS: (Library), "{} {} in library with handle {:016p}",
		if ret.is_null() { "Failed to find" } else { "Found" },
		name_c.to_string_lossy(),
		lib.raw()
	);

	ret
}
