pub mod classloader;

use std::path::{Path, PathBuf};
use std::sync::RwLock;

use once_cell::sync::Lazy;
use common::types::u1;

static CLASSPATH: Lazy<RwLock<ClassPath>> = Lazy::new(|| RwLock::new(ClassPath::default()));

pub fn add_classpath_entry(entry: ClassPathEntry) {
    CLASSPATH.write().unwrap().entries.push(entry);
}

pub fn find_classpath_entry(name: &[u1]) -> Vec<u8> {
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
            }
            ClassPathEntry::Zip => unimplemented!("Zip classpath entries")
        }
    }

    panic!("Class `{}` not found", name);
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