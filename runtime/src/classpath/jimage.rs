use crate::native::jdk::internal::util::SystemProps::Raw::SYSTEM_PROPERTIES;

use common::int_types::u1;
use jimage::JImage;

use std::cell::SyncUnsafeCell;
use std::fs::File;
use std::path::PathBuf;

static JIMAGE_FILE: SyncUnsafeCell<Option<JImage>> = SyncUnsafeCell::new(None);

pub fn initialized() -> bool {
	unsafe { (*JIMAGE_FILE.get()).is_some() }
}

pub fn lookup_vm_resource(path: &str) -> Option<Box<[u1]>> {
	if let Some(file) = unsafe { &*JIMAGE_FILE.get() }
		&& let Some((location_offset, size)) = file.find_resource("java.base", path)
	{
		let uncompressed_data = file.get_resource(location_offset).unwrap(); // TODO: Error handling
		return Some(uncompressed_data);
	}

	None
}

pub fn lookup_vm_options() -> Option<Box<[u1]>> {
	assert!(!initialized(), "Attempt to lookup vm options twice!");

	// CLI/JNI options are already parsed at this point
	let mut modules_path;
	{
		let guard = SYSTEM_PROPERTIES.lock().unwrap();

		let java_home = guard
			.get("java.home")
			.expect("JAVA_HOME should be set at this point");

		let modules_path_len = java_home.len()
			+ 1 // Separator
			+ 3 // "lib"
			+ 1 // Separator
			+ 7; // "modules"
		modules_path = PathBuf::with_capacity(modules_path_len);
		modules_path.push(java_home);
		modules_path.push("lib");
		modules_path.push("modules");
	}

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
