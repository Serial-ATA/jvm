use std::fs::read_dir;
use std::path::Path;

const ARCHITECTURES: &[&str] = &["x86"];

fn panic() {
	panic!(
		"Could not find generated files! Be sure to do `just asm` to generate instruction \
		 definitions prior to building."
	)
}

fn main() {
	let asm_specs_dir = Path::new("../generated/asm_specs");
	if !asm_specs_dir.exists() {
		panic()
	}

	for arch in ARCHITECTURES {
		let arch_dir = asm_specs_dir.join(arch);
		if !arch_dir.exists() || !arch_dir.is_dir() || read_dir(arch_dir).unwrap().count() == 0 {
			panic()
		}
	}
}
