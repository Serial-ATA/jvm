pub mod classloader;
pub mod jar;

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, RwLock};

use common::int_types::u1;
use jimage::JImage;
use once_cell::sync::Lazy;
use zip::ZipArchive;

static CLASSPATH: Lazy<RwLock<ClassPath>> = Lazy::new(|| RwLock::new(ClassPath::default()));

pub fn add_classpath_entry(entry: ClassPathEntry) {
	CLASSPATH.write().unwrap().entries.push(entry);
}

pub fn find_classpath_entry(name: &[u1]) -> Vec<u1> {
	let name = std::str::from_utf8(name).unwrap();
	let mut name = name.replace('.', "/");
	name.push_str(".class");

	for entry in &CLASSPATH.read().unwrap().entries {
		match entry {
			ClassPathEntry::Dir(path) => {
				let class_path = path.join(&name);
				if class_path.exists() {
					return std::fs::read(class_path).unwrap();
				}
			},
			ClassPathEntry::Zip(archive) => {
				let mut archive = archive.lock().unwrap();
				if let Ok(mut file) = archive.by_name(&name) {
					let mut file_contents = Vec::with_capacity(file.size() as usize);
					file.read_to_end(&mut file_contents).unwrap();
					return file_contents;
				};
			},
			ClassPathEntry::Image(_image) => {
				unimplemented!("JImage classpath search")
			},
		}
	}

	panic!("Class `{}` not found", name);
}

pub enum ClassPathEntry {
	Dir(PathBuf),
	Zip(Mutex<ZipArchive<File>>),
	Image(JImage),
}

impl ClassPathEntry {
	pub fn new<P: AsRef<Path>>(path: P) -> Self {
		let path = path.as_ref();
		assert!(path.exists());

		if path.is_dir() {
			return Self::Dir(path.to_path_buf());
		}

		let extension = path.extension().map(|ext| ext.to_str().unwrap());
		if extension == Some("jar") || extension == Some("zip") {
			return Self::Zip(Mutex::new(
				ZipArchive::new(File::open(path).unwrap()).unwrap(),
			));
		}

		panic!("")
	}
}

#[derive(Default)]
struct ClassPath {
	entries: Vec<ClassPathEntry>,
}
