fn main() {
	println!("cargo::rerun-if-changed=../generators/native_methods/");
	build_deps::rerun_if_changed_paths("src/native/**/*.def").unwrap();

	if let Err(e) = native_methods::generate() {
		println!("cargo::error=Failed to generate native methods: {e}");
		std::process::exit(1);
	}
}
