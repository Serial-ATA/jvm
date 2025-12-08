use crate::env::SystemPaths;
use libc::{Dl_info, dladdr};
use std::ffi::{CStr, OsStr};
use std::mem;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

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

fn boot_library_path(libjvm_path: &Path) -> Option<PathBuf> {
	let vm_variant_dir = libjvm_path.parent()?;
	let libs_dir = vm_variant_dir.parent()?;

	if libs_dir.file_name().is_none_or(|f| f != "lib") {
		return None;
	}

	Some(libs_dir.to_path_buf())
}

fn java_home(boot_library_path: &Path) -> Option<PathBuf> {
	if let Ok(path) = std::env::var("JAVA_HOME") {
		return Some(PathBuf::from(path));
	}

	// If any of the expected components aren't found, there isn't much we can do other than hope the
	// user set -Djava.home, which will overwrite anything returned here anyway.
	let java_home = boot_library_path.parent()?;

	Some(java_home.to_path_buf())
}

impl SystemPaths {
	pub(in crate::family) fn init_impl() -> Option<Self> {
		let libjvm_path = libjvm_path()?;
		let boot_library_path = boot_library_path(&libjvm_path)?;
		let java_home = java_home(&libjvm_path)?;

		let extensions_dirs = format!(
			"{}{EXTENSIONS_DIR}:{SYS_EXTENSIONS_DIR}{EXTENSIONS_DIR}",
			java_home.display()
		);

		Some(Self {
			libjvm_path,
			boot_library_path,
			boot_class_path: crate::env::boot_class_path(&java_home)?,
			java_home,
			extensions_dirs,
			_priv: (),
		})
	}
}
