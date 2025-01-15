fn main() {
	println!("cargo:rerun-if-changed=../generators/native_methods");
	println!("cargo:rerun-if-changed=src/native/mod.rs");

	if let Err(e) = native_methods::generate() {
		panic!("Failed to generate native methods: {}", e);
	}
}
