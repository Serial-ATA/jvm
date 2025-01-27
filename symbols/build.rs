fn main() {
	println!("cargo:rerun-if-changed=../generators/vm_symbols");
	println!("cargo:rerun-if-changed=../generators/native_methods");
	println!("cargo:rerun-if-changed=../generated/native/*");
}
