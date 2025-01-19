use common::int_types::u1;
use jimage::JImage;

use std::cell::SyncUnsafeCell;
use std::fs::File;
use std::path::PathBuf;

static JIMAGE_FILE: SyncUnsafeCell<Option<JImage>> = SyncUnsafeCell::new(None);

pub fn initialized() -> bool {
	unsafe { (*JIMAGE_FILE.get()).is_some() }
}

pub fn lookup_vm_resource(path: &str) -> Option<Vec<u1>> {
	if let Some(file) = unsafe { &*JIMAGE_FILE.get() } {
		if let Some((location_offset, size)) = file.find_resource("java.base", path) {
			let mut uncompressed_data = vec![0; size as usize];
			file.get_resource(location_offset, &mut uncompressed_data)
				.unwrap(); // TODO: Error handling

			return Some(uncompressed_data);
		}
	}

	None
}

pub fn lookup_vm_options() -> Option<Vec<u1>> {
	assert!(!initialized(), "Attempt to lookup vm options twice!");

	let java_home = std::env::var("JAVA_HOME").expect("JAVA_HOME not set");

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
	let jimage = JImage::read_from(&mut jimage_file).unwrap(); // TODO: Error handling

	unsafe {
		*JIMAGE_FILE.get() = Some(jimage);
	}

	lookup_vm_resource("jdk/internal/vm/options")
}
