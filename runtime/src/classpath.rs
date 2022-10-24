use once_cell::sync::Lazy;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

static CLASSPATH: Lazy<RwLock<ClassPath>> = Lazy::new(|| RwLock::new(ClassPath::default()));

pub fn add_classpath_entry(entry: ClassPathEntry) {
	CLASSPATH.write().unwrap().entries.push(entry);
}

pub enum ClassPathEntry {
	Dir(PathBuf),
	Zip,
}

impl ClassPathEntry {
	pub fn new<P: AsRef<Path>>(path: P) -> Self {
		let path = path.as_ref();
		assert!(path.exists());

		if path.is_dir() {
			Self::Dir(path.to_path_buf())
		} else {
			// TODO
			Self::Zip
		}
	}
}

#[derive(Default)]
struct ClassPath {
	entries: Vec<ClassPathEntry>,
}
