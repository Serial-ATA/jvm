use crate::properties::FILE_SEPARATOR;

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

// Export family specific impls

#[cfg(target_family = "unix")]
pub use super::unix::env::*;
#[cfg(target_family = "windows")]
pub use super::windows::env::*;

// We can just share the paths, they'll never change during runtime
static SYSTEM_PATHS: OnceLock<Option<SystemPaths>> = OnceLock::new();

pub struct SystemPaths {
	pub libjvm_path: PathBuf,
	pub boot_library_path: PathBuf,
	pub boot_class_path: PathBuf,
	pub java_home: PathBuf,
	pub extensions_dirs: String,
	pub(crate) _priv: (),
}

impl SystemPaths {
	pub fn init() -> Option<&'static Self> {
		SYSTEM_PATHS.get_or_init(Self::init_impl).as_ref()
	}
}

pub(crate) fn boot_class_path(java_home: &Path) -> Option<PathBuf> {
	let jimage_path = format!(
		"{}{FILE_SEPARATOR}lib{FILE_SEPARATOR}modules",
		java_home.display()
	);
	if Path::new(&jimage_path).exists() {
		return Some(PathBuf::from(jimage_path));
	}

	let exploded_modules_path =
		format!("{}{FILE_SEPARATOR}/modules/java.base", java_home.display());
	if Path::new(&exploded_modules_path).exists() {
		return Some(PathBuf::from(exploded_modules_path));
	}

	None
}
