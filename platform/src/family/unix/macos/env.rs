pub fn libjvm_path() -> Option<PathBuf> {
	unimplemented!("macOS libjvm loading");
}

pub fn java_library_path() -> String {
	unimplemented!("macOS java.library.path loading");
}

pub fn boot_library_path(libjvm_path: &Path) -> Option<PathBuf> {
	unimplemented!("macOS sun.boot.library.path loading");
}

pub fn java_home(boot_library_path: &Path) -> Option<PathBuf> {
	unimplemented!("macOS java.home loading");
}

pub fn extensions_dirs(java_home_path: &Path) -> String {
	unimplemented!("macOS java.ext.dirs loading");
}
