use common::int_types::u1;
use jimage::JImage;

use std::cell::UnsafeCell;
use std::fs::File;
use std::path::PathBuf;

static mut JIMAGE_FILE: UnsafeCell<Option<JImage>> = UnsafeCell::new(None);

pub fn initialized() -> bool {
	unsafe { (*JIMAGE_FILE.get()).is_some() }
}

pub fn lookup_vm_resource(path: &str) -> Option<Vec<u1>> {
	if let Some(file) = unsafe { &*JIMAGE_FILE.get() } {
		let mut size = 0;

		if let Some(location_offset) = file.find_resource("java.base", path, &mut size) {
			let mut uncompressed_data = vec![0; size as usize];
			file.get_resource(location_offset, &mut uncompressed_data);

			return Some(uncompressed_data);
		}
	}

	None
}

pub fn lookup_vm_options() -> Option<Vec<u1>> {
	assert!(
		unsafe { (*JIMAGE_FILE.get()).is_none() },
		"Attempt to lookup vm options twice!"
	);

	let java_home = env!("JAVA_HOME");

	let modules_path_len = java_home.len()
		+ 1 // Separator
		+ 3 // "lib"
		+ 1 // Separator
		+ 7; // "modules"
	let mut modules_path = PathBuf::with_capacity(modules_path_len);
	modules_path.push(java_home);
	modules_path.push("lib");
	modules_path.push("modules");

	if !modules_path.exists() {
		return None;
	}

	let mut jimage_file = File::open(modules_path).unwrap();
	let jimage = jimage_parser::parse(&mut jimage_file);

	unsafe {
		*JIMAGE_FILE.get_mut() = Some(jimage);
	}

	lookup_vm_resource("jdk/internal/vm/options")
}
