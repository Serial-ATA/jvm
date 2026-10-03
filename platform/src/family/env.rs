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

#[non_exhaustive]
pub struct SystemPaths {
	pub libjvm_path: PathBuf,
	pub boot_library_path: PathBuf,
	pub boot_class_path: PathBuf,
	pub java_home: PathBuf,
	pub extensions_dirs: String,
}

impl SystemPaths {
	/// Fetch the [`SystemPaths`] for the current environment
	///
	/// The paths are fetched once and cached for future calls
	#[cfg(not(feature = "test"))]
	pub fn init() -> Option<&'static Self> {
		SYSTEM_PATHS
			.get_or_init(|| {
				let libjvm_path = libjvm_path()?;
				let boot_library_path = boot_library_path(&libjvm_path)?;
				let java_home = java_home(&libjvm_path)?;

				let boot_class_path = boot_class_path(&java_home)?;
				let extensions_dirs = extensions_dirs(&java_home);

				Some(SystemPaths {
					libjvm_path,
					boot_library_path,
					boot_class_path,
					java_home,
					extensions_dirs,
				})
			})
			.as_ref()
	}

	/// Stripped down `init` for tests
	///
	/// Much less platform-specific code, since we're work with a well-known dist in `build/`
	#[cfg(feature = "test")]
	pub fn init() -> Option<&'static Self> {
		use crate::{JNI_LIB_PREFIX, JNI_LIB_SUFFIX};

		SYSTEM_PATHS
			.get_or_init(|| {
				let java_home =
					std::env::var("TEST_JAVA_HOME").expect("`TEST_JAVA_HOME` should be set!");

				let java_home = Path::new(&java_home);
				let boot_library_path = java_home.join("lib");
				let libjvm_path =
					boot_library_path.join(format!("{JNI_LIB_PREFIX}jvm{JNI_LIB_SUFFIX}"));

				let boot_class_path =
					boot_class_path(java_home).expect("incomplete `TEST_JAVA_HOME`");
				let extensions_dirs = extensions_dirs(java_home);

				Some(SystemPaths {
					libjvm_path,
					boot_library_path,
					boot_class_path,
					java_home: java_home.to_path_buf(),
					extensions_dirs,
				})
			})
			.as_ref()
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
