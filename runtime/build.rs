fn main() {
	println!("cargo:rerun-if-changed=generators/native_methods");

	if let Err(e) = native_methods::generate() {
		panic!("Failed to generate native methods: {}", e);
	}
}
