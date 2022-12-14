fn main() {
	println!("cargo:rerun-if-changed=runtime/src/native");

	method_gen::run();
}
