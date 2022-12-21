use crate::classpath::ClassPathEntry;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::sync::Mutex;

use zip::ZipArchive;

pub fn main_class_from_jar_manifest(jar_path: &Path) -> Option<String> {
	let mut main_class = None;

	let jar_file = File::open(jar_path).unwrap();
	let mut zip_archive = ZipArchive::new(jar_file).unwrap();

	if let Ok(mut manifest) = zip_archive.by_name("META-INF/MANIFEST.MF") {
		let mut file_contents = String::new();
		manifest.read_to_string(&mut file_contents).unwrap();

		if let Some(main_class_line) = file_contents
			.lines()
			.find(|line| line.starts_with("Main-Class"))
		{
			if let Some((_, class_name)) = main_class_line.split_once(':') {
				main_class = Some(class_name.trim().to_string());
			}
		}
	}

	if main_class.is_some() {
		// Valid JAR file, add it to the classpath
		super::add_classpath_entry(ClassPathEntry::Zip(Mutex::new(zip_archive)));
	}

	main_class
}
