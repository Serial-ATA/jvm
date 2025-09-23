//! https://github.com/openjdk/jdk/blob/030b071db1fb6197a2633a04b20aa95432a903bc/test/jdk/jdk/internal/jimage/JImageReadTest.java#L53

use crate::JImage;

use std::fs::File;
use std::path::Path;

const CLASSES: &[&str] = &[
	"/java.base/java/lang/String.class",
	"/java.base/java/lang/Object.class",
	"/java.base/sun/reflect/generics/tree/TypeArgument.class",
	"/java.base/sun/net/www/content-types.properties",
	"/java.logging/java/util/logging/Logger.class",
	"/java.base/java/NOSUCHCLASS/yyy.class", // non-existent
	"/NOSUCHMODULE/java/lang/Class.class",   // non-existent
];

fn image_file() -> Option<File> {
	let java_home = std::env::var("JAVA_HOME").expect("JAVA_HOME not set");
	File::open(Path::new(&java_home).join("lib").join("modules")).ok()
}

#[test]
#[allow(clippy::case_sensitive_file_extension_comparisons)]
fn read_classes() {
	const CLASS_MAGIC: u32 = 0xCAFE_BABE;

	let Some(mut image_file) = image_file() else {
		println!("Test skipped; no jimage file");
		return;
	};

	let file = JImage::read_from(&mut image_file).unwrap();

	for class in CLASSES {
		let location = file.find_location(class);

		let size;
		if let Some(location) = &location {
			size = location.uncompressed_size();
		} else {
			size = 0;
		}

		println!("reading: path: {}, size: {}", class, size);
		if class.contains("NOSUCH") {
			assert!(
				location.is_none(),
				"location found for non-existing class: {}",
				class
			);
			return;
		}

		assert!(location.is_some(), "location not found: {}", class);
		assert!(size > 0, "size of {} should be > 0: ", class);

		let buffer = file
			.get_resource_from_location(location.as_ref().unwrap())
			.unwrap();
		if class.ends_with(".class") {
			let magic = u32::from_be_bytes(buffer[..4].try_into().unwrap());
			assert_eq!(magic, CLASS_MAGIC, "Classfile has bad magic number");
		}
	}
}

#[test]
fn image_resources() {
	let Some(mut image_file) = image_file() else {
		println!("Test skipped; no jimage file");
		return;
	};

	let file = JImage::read_from(&mut image_file).unwrap();

	let names = file.get_entry_names().unwrap();

	// Repeat with count available
	let count = names.len();

	println!(" count: {}, a class: {}", count, names[0]);

	let min_entry_count = 16000;
	assert!(
		min_entry_count < count,
		"unexpected count of entries, count: {}, min: {}",
		count,
		min_entry_count
	);
	for name in &names {
		check_full_name(name);
	}
}

#[allow(clippy::manual_strip, unused)]
fn check_full_name(path: &str) {
	if path.starts_with("/packages") || path.starts_with("/modules") {
		return;
	}

	let mut next = 0;
	let mut module = None;
	let mut parent = None;
	let mut base = None;
	let mut extension = None;
	if path.starts_with('/') {
		next = path[1..].find('/').unwrap();
		module = Some(&path[1..next]);
		next += 1;
	}
	let last_slash = path.rfind('/').unwrap();
	if last_slash > next {
		// has a parent
		parent = Some(&path[next..last_slash]);
		next = last_slash + 1;
	}
	let period = path[next..].find('.');
	if let Some(period) = period {
		base = Some(&path[next..next + period]);
		extension = Some(&path[period + 1..]);
	} else {
		base = Some(&path[next + 1..]);
	}
	assert!(module.is_some(), "module must be non-empty");
	assert!(base.is_some(), "base name must be non-empty");
}
