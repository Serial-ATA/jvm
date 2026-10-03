use std::ffi::{CStr, OsStr};
use std::mem;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

use libc::{Dl_info, dladdr};

pub const SYS_EXTENSIONS_DIR: &str = "/usr/java/packages";
pub const EXTENSIONS_DIR: &str = "/lib/ext";
pub const DEFAULT_LIBPATH: &str = "/usr/lib64:/lib64:/lib:/usr/lib";

// In the Hotspot-style JAVA_HOME, the libjvm.so will be located at $JAVA_HOME/lib/<vm_variant>/libjvm.so
pub fn libjvm_path() -> Option<PathBuf> {
	let mut dlinfo;
	let dladdr_ret;
	unsafe {
		dlinfo = mem::zeroed::<Dl_info>();
		dladdr_ret = dladdr(libjvm_path as *const _, &raw mut dlinfo)
	}

	if dladdr_ret == 0 {
		return None;
	}

	let lib_path_raw = unsafe { CStr::from_ptr(dlinfo.dli_fname) };
	let lib_path_osstr = OsStr::from_bytes(lib_path_raw.to_bytes());

	Some(PathBuf::from(lib_path_osstr))
}

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

pub fn boot_library_path(libjvm_path: &Path) -> Option<PathBuf> {
	let vm_variant_dir = libjvm_path.parent()?;
	let libs_dir = vm_variant_dir.parent()?;

	if libs_dir.file_name().is_none_or(|f| f != "lib") {
		return None;
	}

	Some(libs_dir.to_path_buf())
}

pub fn java_home(boot_library_path: &Path) -> Option<PathBuf> {
	if let Ok(path) = std::env::var("JAVA_HOME") {
		return Some(PathBuf::from(path));
	}

	// If any of the expected components aren't found, there isn't much we can do other than hope the
	// user set -Djava.home, which will overwrite anything returned here anyway.
	let java_home = boot_library_path.parent()?;

	Some(java_home.to_path_buf())
}

pub fn extensions_dirs(java_home_path: &Path) -> String {
	format!(
		"{}{EXTENSIONS_DIR}:{SYS_EXTENSIONS_DIR}{EXTENSIONS_DIR}",
		java_home_path.display()
	)
}
