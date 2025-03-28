fn main() {
	println!("cargo::rerun-if-changed=../generators/native_methods/");
	println!("cargo::rerun-if-changed=src/native/mod.rs");
	println!("cargo::rerun-if-changed=src/native/java");
	println!("cargo::rerun-if-changed=src/native/jdk");

	if let Err(e) = native_methods::generate() {
		println!("cargo::error=Failed to generate native methods: {e}");
		std::process::exit(1);
	}
}
