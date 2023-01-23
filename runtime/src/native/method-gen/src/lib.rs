#![feature(drain_filter)]

mod parse;

use crate::parse::{AccessFlags, Class, Member, Method};

use std::borrow::Cow;
use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
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
			build_map_inserts(&mut init_fn_file, module, class);
		}
	}

	write!(init_fn_file, "map\n}}").unwrap();
}

fn build_map_inserts(file: &mut File, module: &str, class: &Class) {
	for member in &class.members {
		match member {
			Member::Method(method) => {
				writeln!(
					file,
					"map.insert({});",
					method_table_entry(module, &class.class_name, method)
				)
				.unwrap();
			},
			Member::Class(class_) => build_map_inserts(file, module, class_),
		}
	}
}

macro_rules! native_method_table_file_header {
	() => {
		r#"use crate::native::{{NativeMethodDef, NativeMethodPtr}};

use std::sync::atomic::{{AtomicBool, Ordering}};

static NATIVES_REGISTERED: AtomicBool = AtomicBool::new(false);

#[allow(trivial_casts)]
pub fn registerNatives(_: crate::stack::local_stack::LocalStack) -> NativeReturn {{
	if NATIVES_REGISTERED.compare_exchange(false, true, Ordering::SeqCst, Ordering::Acquire) != Ok(false) {{
		return None;
	}}
	
	let natives: [(NativeMethodDef<'static>, NativeMethodPtr); {}] = [
"#
	};
}

fn generate_register_natives_table(module: &str, class: &mut Class, def_path: &Path) {
	if !class
		.members
		.iter()
		.any(|member| matches!(member, Member::Method(method) if method.name == "registerNatives"))
	{
		return;
	}

	let native_method_table_path = def_path.join(format!("{}.registerNatives", class.class_name));
	let mut native_method_table_file = OpenOptions::new()
		.write(true)
		.truncate(true)
		.create(true)
		.open(native_method_table_path)
		.unwrap();

	write!(
		native_method_table_file,
		"{}",
		format_args!(
			native_method_table_file_header!(),
			class
				.members
				.iter()
				.filter(|member| matches!(member, Member::Method(method) if !method.modifiers.contains(AccessFlags::ACC_STATIC)))
				.count()
		)
	)
	.unwrap();

	for ref member in class.members.drain_filter(|member| {
		matches!(member, Member::Method(method) if method.name != "registerNatives" && !method.modifiers.contains(AccessFlags::ACC_STATIC))
	}) {
		match member {
			Member::Method(method) => {
				writeln!(
					native_method_table_file,
					"\t\t({}),",
					method_table_entry(module, &class.class_name, method)
				)
					.unwrap();
			}
			Member::Class(_) => unreachable!()
		}
	}

	write!(
		native_method_table_file,
		"\t];\n\n\tfor method in natives \
		 {{\n\t\tcrate::native::insert_method(method);\n\t}}\nNone\n}}"
	)
	.unwrap();
}

fn method_table_entry(module: &str, class_name: &str, method: &Method) -> String {
	format!(
		"NativeMethodDef {{ class: &{:?}, name: &{:?}, descriptor: &{:?} }}, \
		 crate::native::{}{}::{} as NativeMethodPtr",
		format!("{}{}", module, class_name).as_bytes(),
		method.name.as_bytes(),
		method.descriptor.as_bytes(),
		module.replace('/', "::"),
		class_name.replace('$', "::"),
		method.name
	)
}

#[test]
fn test_parse() {
	run();
}
