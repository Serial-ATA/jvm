pub mod jar;
pub mod jimage;
pub mod loader;

use crate::classpath::loader::ClassLoader;
use crate::modules::with_module_lock;
use crate::symbols::{Symbol, sym};

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex, RwLock};

use common::int_types::u1;
use zip::ZipArchive;

static CLASSPATH: LazyLock<RwLock<ClassPath>> = LazyLock::new(|| RwLock::new(ClassPath::default()));

pub fn add_classpath_entry(entry: ClassPathEntry) {
	CLASSPATH.write().unwrap().entries.push(entry);
}

pub fn find_classpath_entry(loader: &ClassLoader, name: Symbol) -> Option<Vec<u1>> {
	let mut name = name.as_str().replace('.', "/");
	name.push_str(".class");

	let package_name = name.rfind('/').map(|i| &name[..i])?;
	let package =
		with_module_lock(|guard| loader.lookup_package(guard, Symbol::intern(package_name)));
	let module_name = package
		.and_then(|pkg| pkg.module().name())
		.unwrap_or(sym!(java_base));

	if jimage::initialized()
		&& let Some(resource) = jimage::lookup_vm_resource(module_name.as_str(), &name)
	{
		return Some(resource.into_vec());
	}

	for entry in &CLASSPATH.read().unwrap().entries {
		let ret = entry.find(&name);
		if ret.is_some() {
			return ret;
		}
	}

	None
}

/// A zip (.zip/.jar) classpath entry
enum ZipEntry {
	Unopened(PathBuf),
	Open(ZipArchive<File>),
}

pub enum ClassPathEntry {
	Dir(PathBuf),
	Zip(Mutex<ZipEntry>),
}

impl ClassPathEntry {
	pub fn new<P: AsRef<Path>>(path: P) -> Self {
		let path = path.as_ref();
		match path.extension().and_then(|ext| ext.to_str()) {
			Some("jar") | Some("zip") => {
				Self::Zip(Mutex::new(ZipEntry::Unopened(path.to_path_buf())))
			},
			_ => Self::Dir(path.to_path_buf()),
		}
	}

	/// Attempt to find an entry by `name`
	fn find(&self, name: &str) -> Option<Vec<u1>> {
		// TODO: Zero error handling... not even a little bit
		match self {
			ClassPathEntry::Dir(path) => {
				let class_path = path.join(&name);
				if class_path.exists() {
					return Some(std::fs::read(class_path).unwrap());
				}

				None
			},
			ClassPathEntry::Zip(entry) => {
				let mut zip_entry = entry.lock().unwrap();
				if let ZipEntry::Unopened(path) = &*zip_entry {
					let Ok(file) = File::open(path) else {
						return None;
					};

					let archive = ZipArchive::new(file).unwrap();
					*zip_entry = ZipEntry::Open(archive);
				}

				let ZipEntry::Open(archive) = &mut *zip_entry else {
					unreachable!();
				};

				if let Ok(mut file) = archive.by_name(name) {
					let mut file_contents = Vec::with_capacity(file.size() as usize);
					file.read_to_end(&mut file_contents).unwrap();
					return Some(file_contents);
				}

				None
			},
		}
	}
}

#[derive(Default)]
struct ClassPath {
	entries: Vec<ClassPathEntry>,
}
