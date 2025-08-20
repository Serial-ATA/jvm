pub fn java_library_path() -> String {
	unimplemented!("Windows java.library.path loading");
}

fn java_home() -> String {
	unimplemented!("Windows java.home loading");
}

impl SystemPaths {
	pub fn init() -> Option<Self> {
		unimplemented!("Windows `SystemPaths` loading");
	}
}
