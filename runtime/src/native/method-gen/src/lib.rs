mod parse;

use crate::parse::{Class, Method};

use std::borrow::Cow;
use std::ffi::OsStr;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Component, Path, PathBuf};

use walkdir::WalkDir;

static CRATE_ROOT: &str = env!("CARGO_MANIFEST_DIR");
static METHOD_DEFINITION_DIR_NAME: &str = "def";
static INIT_FN_FILE_HEADER: &str = "#[allow(trivial_casts)]\nfn init_native_method_table() -> \
                                    HashMap<NativeMethodDef<'static>, NativeMethodPtr> {\nlet mut \
                                    map = HashMap::new();\n";

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
			let path = entry.path();
			if path.extension() != Some(METHOD_DEFINITION_DIR_NAME.as_ref()) {
				continue;
			}

			let file_contents = std::fs::read_to_string(&path).unwrap();
			let mut class = parse::Class::parse(
				file_contents,
				path.file_stem().unwrap().to_str().unwrap(),
				&module_name,
			);
			generate_register_natives_table(&module_name, &mut class, dir.path());

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

	for (module, classes) in &modules {
		for class in classes {
			for method in &class.methods {
				writeln!(
					init_fn_file,
					"map.insert({});",
					method_table_entry(module, class, method)
				)
				.unwrap();
			}
		}
	}

	write!(init_fn_file, "map\n}}").unwrap();
}

macro_rules! native_method_table_file_header {
	() => {
		r#"use crate::native::{{NativeMethodDef, NativeMethodPtr}};

use std::sync::atomic::{{AtomicBool, Ordering}};

static NATIVES_REGISTERED: AtomicBool = AtomicBool::new(false);

#[allow(trivial_casts)]
pub fn registerNatives(_: crate::stack::local_stack::LocalStack) {{
	if NATIVES_REGISTERED.compare_exchange(false, true, Ordering::SeqCst, Ordering::Acquire) != Ok(false) {{
		return;
	}}
	
	let natives: [(NativeMethodDef<'static>, NativeMethodPtr); {}] = [
"#
	};
}

fn generate_register_natives_table(module: &str, class: &mut Class, def_path: &Path) {
	if let Some(pos) = class
		.methods
		.iter()
		.position(|method| method.name == "registerNatives")
	{
		class.methods.swap(pos, 0);

		let native_method_table_path =
			def_path.join(format!("{}.registerNatives", class.class_name));
		let mut native_method_table_file = OpenOptions::new()
			.write(true)
			.truncate(true)
			.create(true)
			.open(native_method_table_path)
			.unwrap();

		write!(
			native_method_table_file,
			"{}",
			format_args!(native_method_table_file_header!(), class.methods.len() - 1)
		)
		.unwrap();

		for ref method in class.methods.drain(1..).collect::<Vec<Method>>() {
			writeln!(
				native_method_table_file,
				"\t\t({}),",
				method_table_entry(module, class, method)
			)
			.unwrap();
		}

		write!(
			native_method_table_file,
			"\t];\n\n\tfor method in natives \
			 {{\n\t\tcrate::native::insert_method(method);\n\t}}\n}}"
		)
		.unwrap();
	}
}

fn method_table_entry(module: &str, class: &Class, method: &Method) -> String {
	format!(
		"NativeMethodDef {{ class: &{:?}, name: &{:?}, descriptor: &{:?} }}, \
		 crate::native::{}{}::{} as NativeMethodPtr",
		format!("{}{}", module, class.class_name).as_bytes(),
		method.name.as_bytes(),
		method.descriptor.as_bytes(),
		module.replace('/', "::"),
		class.class_name,
		method.name
	)
}

#[test]
fn test_parse() {
	run();
}
