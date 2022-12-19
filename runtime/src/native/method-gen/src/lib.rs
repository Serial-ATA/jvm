mod parse;

use std::borrow::Cow;
use std::ffi::OsStr;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Component, PathBuf};

use walkdir::WalkDir;

static CRATE_ROOT: &str = env!("CARGO_MANIFEST_DIR");
static METHOD_DEFINITION_DIR_NAME: &str = "def";
static INIT_FN_FILE_HEADER: &str = "#[allow(trivial_casts)]\nfn init_native_method_table() -> \
                                    HashMap<NativeMethodDef<'static>, NativeMethodPtr> {\nlet mut \
                                    map = HashMap::new();";

pub fn run() {
	let crate_root = PathBuf::from(CRATE_ROOT);
	let native_directory = crate_root.parent().unwrap();

	let dirs_filtered = WalkDir::new(native_directory)
		.into_iter()
		.map(Result::unwrap)
		.filter(|entry| {
			entry.file_type().is_dir() && entry.file_name() == METHOD_DEFINITION_DIR_NAME
		});

	let mut modules = Vec::new();
	for dir in dirs_filtered {
		let mut components = dir
			.path()
			.components()
			.rev()
			.skip(1)
			.map(Component::as_os_str)
			.map(OsStr::to_string_lossy)
			.take_while(|comp| comp != "native")
			.collect::<Vec<Cow<'_, str>>>();

		components.reverse();

		let mut module_name = String::new();
		for comp in components {
			module_name.push_str(&comp);
			module_name.push('/');
		}

		let mut module_classes = Vec::new();
		for entry in std::fs::read_dir(dir.path()).unwrap().map(Result::unwrap) {
			let file_contents = std::fs::read_to_string(entry.path()).unwrap();
			let class = parse::Class::parse(file_contents);
			module_classes.push(class);
		}

		modules.push((module_name, module_classes))
	}

	let init_fn_file_path = native_directory.join("native_init.rs");
	let mut init_fn_file = OpenOptions::new()
		.write(true)
		.truncate(true)
		.create(true)
		.open(init_fn_file_path)
		.unwrap();

	write!(init_fn_file, "{}", INIT_FN_FILE_HEADER).unwrap();

	for (module, classes) in modules {
		for class in classes {
			for method in class.methods {
				write!(
					init_fn_file,
					"map.insert(NativeMethodDef {{ class: &{:?}, name: &{:?}, descriptor: &{:?} \
					 }}, {}{}::{} as NativeMethodPtr);\n",
					format!("{}{}", module, class.class_name).as_bytes(),
					method.name.as_bytes(),
					method.descriptor.as_bytes(),
					module.replace('/', "::"),
					class.class_name,
					method.name
				)
				.unwrap();
			}
		}
	}

	write!(init_fn_file, "map}}").unwrap();
}

#[test]
fn test_parse() {
	run();
}
