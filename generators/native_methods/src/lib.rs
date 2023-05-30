#![feature(drain_filter)]

mod parse;

use crate::parse::{AccessFlags, Class, Member, Method};

use std::borrow::Cow;
use std::ffi::OsStr;
use std::fmt::Write as _;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Component, Path, PathBuf};

use walkdir::WalkDir;

static CRATE_ROOT: &str = env!("CARGO_MANIFEST_DIR");
static METHOD_DEFINITION_DIR_NAME: &str = "def";
static INIT_FN_FILE_HEADER: &str = "#[allow(trivial_casts)]\nfn init_native_method_table() -> \
                                    HashMap<NativeMethodDef<'static>, NativeMethodPtr> {\nlet mut \
                                    map = HashMap::new();\n";

static MODULE_MARKER_START_COMMENT: &str = "// Module marker, do not remove";

pub fn generate() {
	// Do a bunch of path work to get to ../../runtime/src/native
	let crate_root = PathBuf::from(CRATE_ROOT);
	let project_root = crate_root.parent().unwrap().parent().unwrap();

	let native_directory = project_root.join("runtime").join("src").join("native");

	let dirs_filtered = WalkDir::new(&native_directory)
		.into_iter()
		.map(Result::unwrap)
		.filter(|entry| {
			entry.file_type().is_dir() && entry.file_name() == METHOD_DEFINITION_DIR_NAME
		});

	let mut modules = Vec::new();
	for dir in dirs_filtered {
		let components = create_relative_path_components(dir.path(), true);

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
			generate_native_constant_fields(&mut class, dir.path());
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

	let generated_modules = generate_modules(&native_directory);

	let root_module_path = native_directory.join("mod.rs");
	let root_mod_file_content = std::fs::read_to_string(&root_module_path).unwrap();

	let marker_comment_start_pos = root_mod_file_content
		.rfind(MODULE_MARKER_START_COMMENT)
		.expect("Can't find module marker comment");

	// Remove anything trailing the comment
	let mut root_mod_file_content_bytes = root_mod_file_content.into_bytes();
	root_mod_file_content_bytes
		.drain(marker_comment_start_pos + MODULE_MARKER_START_COMMENT.len()..);

	write!(
		&mut root_mod_file_content_bytes,
		"\n{}\n",
		&generated_modules
	)
	.unwrap();
	std::fs::write(&root_module_path, &root_mod_file_content_bytes)
		.expect("Failed to write modules to native/mod.rs");
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
			_ => {},
		}
	}
}

fn class_contains_fields(class: &Class) -> bool {
	for member in &class.members {
		match member {
			Member::Field(_) => return true,
			Member::Class(c) => return class_contains_fields(c),
			Member::Method(_) => {},
		}
	}

	false
}

fn generate_native_constant_fields(class: &mut Class, def_path: &Path) {
	if !class_contains_fields(class) {
		return;
	}

	let file = create_constant_field_file_for_class(class, def_path);
	write_entries_for_class(file, def_path, class);
}

fn create_constant_field_file_for_class(class: &Class, def_path: &Path) -> Option<File> {
	if !class
		.members
		.iter()
		.any(|member| matches!(member, Member::Field(_)))
	{
		return None;
	}

	let constant_fields_path = def_path.join(format!("{}.constants", class.class_name));
	let constant_fields_file = OpenOptions::new()
		.write(true)
		.truncate(true)
		.create(true)
		.open(constant_fields_path)
		.unwrap();

	Some(constant_fields_file)
}

fn write_entries_for_class(mut file: Option<File>, def_path: &Path, class: &Class) {
	for member in &class.members {
		match member {
			Member::Field(field) => {
				writeln!(
					file.as_mut().expect("file should exist"),
					"#[allow(non_upper_case_globals)]\npub const {}: {} = {};",
					field.name,
					field.ty.map_to_rust_ty(),
					field.expr
				)
				.unwrap();
			},
			Member::Class(c) => {
				let file = create_constant_field_file_for_class(c, def_path);
				write_entries_for_class(file, def_path, c);
			},
			_ => {},
		}
	}
}

macro_rules! native_method_table_file_header {
	() => {
		r#"use crate::native::{{NativeMethodDef, NativeMethodPtr}};

use std::sync::atomic::{{AtomicBool, Ordering}};

static NATIVES_REGISTERED: AtomicBool = AtomicBool::new(false);

#[allow(trivial_casts)]
pub fn registerNatives(_: JNIEnv, _: crate::stack::local_stack::LocalStack) -> NativeReturn {{
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
			_ => unreachable!()
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

fn generate_modules(native_directory: &Path) -> String {
	let mut modules_str = String::new();

	let dirs_filtered = WalkDir::new(native_directory)
		.into_iter()
		.map(Result::unwrap)
		.filter(|entry| {
			entry.file_type().is_dir()
				&& entry.file_name() != METHOD_DEFINITION_DIR_NAME
				&& entry.path().join(METHOD_DEFINITION_DIR_NAME).exists()
		});

	let mut modules = Vec::new();
	for dir in dirs_filtered {
		let components = create_relative_path_components(dir.path(), false);
		if !components.is_empty() {
			let mut files = std::fs::read_dir(dir.path())
				.unwrap()
				.filter_map(|entry| {
					let entry = entry.unwrap();
					if entry.file_type().unwrap().is_file() {
						Some(entry.path().file_stem().unwrap().to_os_string())
					} else {
						None
					}
				})
				.collect::<Vec<_>>();

			files.sort();

			modules.push((components, files));
		}
	}

	let mut current_root = None;
	let mut current_module;
	let mut current_depth = 0;
	let mut index = 0;
	loop {
		if index == modules.len() {
			while current_depth > 0 {
				writeln!(&mut modules_str, "{}}}", "\t".repeat(current_depth)).unwrap();
				current_depth -= 1;
			}
			writeln!(&mut modules_str, "{}}}", "\t".repeat(current_depth)).unwrap();
			break;
		}

		let (module, classes) = &modules[index];
		if current_root != Some(&module[0]) {
			if index != 0 {
				while current_depth > 0 {
					writeln!(&mut modules_str, "{}}}", "\t".repeat(current_depth)).unwrap();
					current_depth -= 1;
				}
				writeln!(&mut modules_str, "}}\n").unwrap();
			}

			current_root = Some(&module[0]);

			let (components, classes) = &modules[index];
			for component in components {
				current_module = Some(component);

				writeln!(
					&mut modules_str,
					"{}pub(crate) mod {} {{",
					"\t".repeat(current_depth),
					current_module.unwrap()
				)
				.unwrap();
				current_depth += 1;
			}

			for item in classes {
				writeln!(
					&mut modules_str,
					"{}pub(crate) mod {};",
					"\t".repeat(current_depth),
					item.to_str().unwrap()
				)
				.unwrap();
			}

			current_depth = components.len() - 1;
			index += 1;
			continue;
		}

		current_module = Some(&module[current_depth]);
		if (module.len() - 1) == current_depth {
			writeln!(&mut modules_str, "{}}}", "\t".repeat(current_depth)).unwrap();
		}

		writeln!(
			&mut modules_str,
			"{}pub(crate) mod {} {{",
			"\t".repeat(current_depth),
			current_module.unwrap()
		)
		.unwrap();

		if current_depth != module.len() - 1 {
			current_depth += 1;
		}

		for item in classes {
			writeln!(
				&mut modules_str,
				"{}pub(crate) mod {};",
				"\t".repeat(current_depth),
				item.to_str().unwrap()
			)
			.unwrap();
		}

		index += 1;
	}

	modules_str
}

fn create_relative_path_components(path: &Path, skip_first: bool) -> Vec<String> {
	let mut components = path
		.components()
		.rev()
		.skip(if skip_first { 1 } else { 0 })
		.map(Component::as_os_str)
		.map(OsStr::to_string_lossy)
		.take_while(|comp| comp != "native")
		.map(Cow::into_owned)
		.collect::<Vec<String>>();

	components.reverse();
	components
}

#[test]
fn test_parse() {
	generate();
}
