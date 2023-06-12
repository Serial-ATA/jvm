#![feature(drain_filter)]

mod field;
mod intrinsics;
mod modules;
mod parse;
mod registernatives;
mod util;

use crate::intrinsics::generate_intrinsics;
use crate::modules::Module;
use crate::parse::{Class, Member};

use std::fmt::Write as _;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

static CRATE_ROOT: &str = env!("CARGO_MANIFEST_DIR");
static INIT_FN_FILE_HEADER: &str = "#[allow(trivial_casts)]\nfn init_native_method_table() -> \
                                    HashMap<NativeMethodDef<'static>, NativeMethodPtr> {\nlet mut \
                                    map = HashMap::new();\n";

static MODULE_MARKER_START_COMMENT: &str = "// Module marker, do not remove";

fn get_runtime_native_directory() -> PathBuf {
	// Do a bunch of path work to get to ../../runtime/src/native
	let crate_root = PathBuf::from(CRATE_ROOT);
	let project_root = crate_root.parent().unwrap().parent().unwrap();

	project_root.join("runtime").join("src").join("native")
}

fn get_generated_directory() -> PathBuf {
	let crate_root = PathBuf::from(CRATE_ROOT);
	let project_root = crate_root.parent().unwrap().parent().unwrap();

	project_root.join("generated").join("native")
}

pub fn generate() {
	let native_directory = get_runtime_native_directory();
	let generated_directory = get_generated_directory();
	let modules = modules::get_modules_from(&generated_directory, &native_directory);

	generate_intrinsics(&native_directory, &modules);
	create_native_method_table(&generated_directory, &modules);
	generate_modules(&native_directory, &modules);
}

fn create_native_method_table(generated_directory: &Path, modules: &[Module]) {
	let init_fn_file_path = generated_directory.join("native_init.rs");
	let mut init_fn_file = OpenOptions::new()
		.write(true)
		.truncate(true)
		.create(true)
		.open(init_fn_file_path)
		.unwrap();

	write!(init_fn_file, "{}", INIT_FN_FILE_HEADER).unwrap();

	for module in modules {
		for class in &module.classes {
			build_map_inserts(&mut init_fn_file, &module.name, class);
		}
	}

	write!(init_fn_file, "map\n}}").unwrap();
}

fn build_map_inserts(file: &mut File, module: &str, class: &Class) {
	for member in &class.members {
		match member {
			Member::Method(method) => {
				if method.is_intrinsic {
					continue; // TODO
				}

				writeln!(
					file,
					"map.insert({});",
					util::method_table_entry(module, &class.class_name, method)
				)
				.unwrap();
			},
			Member::Class(class_) => build_map_inserts(file, module, class_),
			_ => {},
		}
	}
}

/// Create module definitions to be placed in `runtime/src/native/mod.rs`
///
/// This goes through all of the directories in `runtime/src/native` to create a module structure.
///
/// For example, `runtime/src/native/java/lang/Object.rs` would be converted into:
///
/// ```rs
/// mod java {
///     mod lang {
///         mod Object;
///     }
/// }
/// ```
fn generate_modules(native_directory: &Path, modules: &[Module]) {
	let root_module_path = native_directory.join("mod.rs");
	let root_mod_file_content = std::fs::read_to_string(&root_module_path).unwrap();

	let marker_comment_start_pos = root_mod_file_content
		.rfind(MODULE_MARKER_START_COMMENT)
		.expect("Can't find module marker comment");

	// Remove anything trailing the comment
	let mut root_mod_file_content_bytes = root_mod_file_content.into_bytes();
	root_mod_file_content_bytes
		.drain(marker_comment_start_pos + MODULE_MARKER_START_COMMENT.len()..);

	let generated_modules = create_modules_string(modules);

	write!(
		&mut root_mod_file_content_bytes,
		"\n{}\n",
		&generated_modules
	)
	.unwrap();
	std::fs::write(&root_module_path, &root_mod_file_content_bytes)
		.expect("Failed to write modules to native/mod.rs");
}

fn create_modules_string(modules: &[Module]) -> String {
	let mut modules_str = String::new();

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

		let module = &modules[index];
		if current_root != module.components.first() {
			if index != 0 {
				while current_depth > 0 {
					writeln!(&mut modules_str, "{}}}", "\t".repeat(current_depth)).unwrap();
					current_depth -= 1;
				}
				writeln!(&mut modules_str, "}}\n").unwrap();
			}

			current_root = module.components.first();

			let module = &modules[index];
			for component in &module.components {
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

			for class in &module.classes {
				writeln!(
					&mut modules_str,
					"{}pub(crate) mod {};",
					"\t".repeat(current_depth),
					&class.class_name
				)
				.unwrap();
			}

			current_depth = module.components.len() - 1;
			index += 1;
			continue;
		}

		current_module = Some(&module.components[current_depth]);
		if (module.components.len() - 1) == current_depth {
			writeln!(&mut modules_str, "{}}}", "\t".repeat(current_depth)).unwrap();
		}

		writeln!(
			&mut modules_str,
			"{}pub(crate) mod {} {{",
			"\t".repeat(current_depth),
			current_module.unwrap()
		)
		.unwrap();

		if current_depth != module.components.len() - 1 {
			current_depth += 1;
		}

		for class in &module.classes {
			writeln!(
				&mut modules_str,
				"{}pub(crate) mod {};",
				"\t".repeat(current_depth),
				&class.class_name
			)
			.unwrap();
		}

		index += 1;
	}

	modules_str
}

#[test]
fn test_parse() {
	generate();
}
