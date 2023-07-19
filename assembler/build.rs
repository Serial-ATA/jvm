fn main() {
	println!("cargo:rerun-if-changed=assembler/intel_spec_reader");

	intel_spec_reader::generate();
}
