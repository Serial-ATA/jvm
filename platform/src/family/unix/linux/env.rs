use libc::{Dl_info, dladdr};
use std::ffi::{CStr, OsStr};
use std::mem;
use std::os::unix::ffi::OsStrExt;
use std::path::PathBuf;

pub const SYS_EXTENSIONS_DIR: &str = "/usr/java/packages";
pub const DEFAULT_LIBPATH: &str = "/usr/lib64:/lib64:/lib:/usr/lib";

pub fn java_library_path() -> String {
	let ld_library_path = std::env::var("LD_LIBRARY_PATH").map_or_else(
		|_| String::new(),
		|mut ld| {
			ld.push(':');
			ld
		},
	);
	format!("{ld_library_path}{SYS_EXTENSIONS_DIR}/lib:{DEFAULT_LIBPATH}")
}

#[expect(
	clippy::missing_panics_doc,
	reason = "unable to locate = panic currently"
)]
pub fn java_home() -> String {
	if let Ok(path) = std::env::var("JAVA_HOME") {
		return path;
	}

	let mut dlinfo;
	let dladdr_ret;
	unsafe {
		dlinfo = mem::zeroed::<Dl_info>();
		dladdr_ret = dladdr(java_home as *const _, &raw mut dlinfo)
	}

	// TODO: Error, not panic
	assert_ne!(dladdr_ret, 0, "Cannot locate libjvm_runtime.so");

	let lib_path_raw = unsafe { CStr::from_ptr(dlinfo.dli_fname) };
	let lib_path_osstr = OsStr::from_bytes(lib_path_raw.to_bytes());

	// In the Hotspot-style JAVA_HOME, the libjvm_runtime.so will be located at $JAVA_HOME/lib/<vm_variant>/libjvm_runtime.so
	//
	// If any of those components aren't found, there isn't much we can do other than hope the user
	// set -Djava.home, which will overwrite anything return here anyway.
	let lib_path = PathBuf::from(lib_path_osstr);

	let Some(vm_variant_dir) = lib_path.parent() else {
		return String::new();
	};

	let Some(libs_dir) = vm_variant_dir.parent() else {
		return String::new();
	};

	if libs_dir.file_name().is_none_or(|f| f != "lib") {
		return String::new();
	}

	let Some(java_home) = libs_dir.parent() else {
		return String::new();
	};

	java_home.to_string_lossy().into()
}
