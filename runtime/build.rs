fn main() {
	println!("cargo:rerun-if-changed=generators/native_methods");

	native_methods::generate();
}
