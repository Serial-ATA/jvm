fn main() {
	println!("cargo:rerun-if-changed=runtime/src/native");

	native_methods::generate();
}
