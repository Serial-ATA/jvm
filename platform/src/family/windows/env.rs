pub fn libjvm_path() -> Option<PathBuf> {
	unimplemented!("Windows libjvm loading");
}

pub fn java_library_path() -> String {
	unimplemented!("Windows java.library.path loading");
}

pub fn boot_library_path(libjvm_path: &Path) -> Option<PathBuf> {
	unimplemented!("Windows sun.boot.library.path loading");
}

pub fn java_home(boot_library_path: &Path) -> Option<PathBuf> {
	unimplemented!("Windows java.home loading");
}

pub fn extensions_dirs(java_home_path: &Path) -> String {
	unimplemented!("Windows java.ext.dirs loading");
}
