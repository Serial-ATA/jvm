use std::ffi::c_int;
use std::path::Path;

const JVMTI_VERSION_BASE: c_int = 0x30000000;

fn main() {
	println!("cargo:rerun-if-env-changed=JAVA_VERSION");

	let Ok(java_version) = std::env::var("JAVA_VERSION") else {
		panic!("`JAVA_VERSION` environment variable must be set")
	};

	let version_path = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap())
		.join("src")
		.join("version.rs");
	let version_fn = format!(
		"pub(crate) const fn jvmti_version() -> std::ffi::c_int {{ {JVMTI_VERSION_BASE:#X} + \
		 ({java_version} * 0x10000) }}\n"
	);

	std::fs::write(version_path, version_fn).unwrap();
}
