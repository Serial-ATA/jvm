#![feature(drain_filter)]

mod field;
mod intrinsics;
mod modules;
mod parse;
mod registernatives;
mod util;

use crate::intrinsics::generate_intrinsics;
use crate::modules::Module;
use crate::parse::{AccessFlags, Class, Member, Method};

use std::collections::HashMap;
use std::fmt::Write as _;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

static CRATE_ROOT: &str = env!("CARGO_MANIFEST_DIR");
static INIT_FN_FILE_HEADER: &str = "#[allow(trivial_casts)]\nfn init_native_method_table() -> \
                                    HashMap<NativeMethodDef, NativeMethodPtr> {\n\tuse \
                                    symbols::sym;\n\tlet mut map = HashMap::new();\n";

static MODULE_MARKER_START_COMMENT: &str = "// Module marker, do not remove";

#[derive(Default)]
struct SymbolCollector {
	class_name_symbols_to_add: HashMap<String, String>,
	method_name_symbols_to_add: HashMap<String, String>,
	method_signature_symbols_to_add: HashMap<String, String>,
}

impl SymbolCollector {
	pub fn add_class_name(&mut self, value: String) {
		let symbol_name = value.replace('/', "_").replace('$', "_");
		self.class_name_symbols_to_add.insert(symbol_name, value);
	}

	pub fn add_method(&mut self, method: &Method, class: &Class) {
		self.method_name_symbols_to_add
			.insert(method.name_symbol(), method.generated_name().to_string());
		self.method_signature_symbols_to_add.insert(
			method.signature_symbol_name(),
			method.descriptor.to_string(),
		);
	}

	/// Generates additional symbols, injecting them into the `vm_symbols::define_symbols!` call
	/// in `runtime/src/symbols.rs`
	fn generate_symbols<'a>(&self, generated_directory: &Path) {
		// ../../symbols/src/lib.rs
		let symbols_project_dir = generated_directory
			.parent()
			.unwrap()
			.parent()
			.unwrap()
			.join("symbols");
		let symbols_file_path = symbols_project_dir.join("src").join("lib.rs");

		let symbols_file_contents = std::fs::read_to_string(&symbols_file_path).unwrap();

		const CLASS_NAME_SECTION_HEADER: &str = "// Classes";
		const CLASS_NAME_MARKER_COMMENT: &str =
			"// -- GENERATED CLASS NAME MARKER, DO NOT DELETE --";

		Self::generate_symbols_inner(
			&self.class_name_symbols_to_add,
			CLASS_NAME_SECTION_HEADER,
			CLASS_NAME_MARKER_COMMENT,
			generated_directory,
			"class_names.symbols",
			&symbols_file_contents,
		);

		const METHOD_NAME_SECTION_HEADER: &str = "// Methods";
		const METHOD_NAME_MARKER_COMMENT: &str =
			"// -- GENERATED METHOD NAME MARKER, DO NOT DELETE --";

		Self::generate_symbols_inner(
			&self.method_name_symbols_to_add,
			METHOD_NAME_SECTION_HEADER,
			METHOD_NAME_MARKER_COMMENT,
			generated_directory,
			"method_names.symbols",
			&symbols_file_contents,
		);

		const METHOD_SIGNATURE_SECTION_HEADER: &str = "// Signatures";
		const METHOD_SIGNATURE_MARKER_COMMENT: &str =
			"// -- GENERATED METHOD SIGNATURE MARKER, DO NOT DELETE --";

		Self::generate_symbols_inner(
			&self.method_signature_symbols_to_add,
			METHOD_SIGNATURE_SECTION_HEADER,
			METHOD_SIGNATURE_MARKER_COMMENT,
			generated_directory,
			"signature_names.symbols",
			&symbols_file_contents,
		);
	}

	/// Gets the position of a marker comment (eg. "// Classes")
	fn get_position_of_marker_comment(contents: &str, section_marker_comment: &str) -> usize {
		contents.rfind(section_marker_comment).expect(&format!(
			"Failed to find marker comment: {}",
			section_marker_comment
		)) + section_marker_comment.len()
	}

	fn check_in_section(
		contents: &str,
		section_header_position: usize,
		marker_comment_position: usize,
		check_for: &str,
	) -> bool {
		contents[section_header_position..marker_comment_position]
			.contains(&format!("{}", check_for))
	}

	fn generate_symbols_inner<'a>(
		symbol_iter: &HashMap<String, String>,
		section_header: &str,
		marker_comment: &str,
		generated_directory: &Path,
		file_name: &str,
		symbols_file_contents: &str,
	) {
		let section_header_position =
			Self::get_position_of_marker_comment(symbols_file_contents, section_header);
		let marker_comment_position =
			Self::get_position_of_marker_comment(symbols_file_contents, marker_comment);

		let mut symbols_file = OpenOptions::new()
			.write(true)
			.truncate(true)
			.create(true)
			.open(generated_directory.join(file_name))
			.unwrap();

		for (symbol, value) in symbol_iter {
			if !Self::check_in_section(
				symbols_file_contents,
				section_header_position,
				marker_comment_position,
				symbol,
			) {
				writeln!(symbols_file, "{}: {:?},", symbol.replace('/', "_"), value).unwrap();
			}
		}
	}
}

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

/// Generate native and intrinsic method definitions
pub fn generate() {
	let mut symbol_collector = SymbolCollector::default();

	let native_directory = get_runtime_native_directory();
	let generated_directory = get_generated_directory();
	let modules = modules::get_modules_from(
		&generated_directory,
		&native_directory,
		&mut symbol_collector,
	);

	generate_intrinsics(&generated_directory, &modules, &mut symbol_collector);
	create_native_method_table(&generated_directory, &modules, &mut symbol_collector);

	// Generate any new symbols we found that are not defined in `../../symbols/src/lib.rs`
	symbol_collector.generate_symbols(&generated_directory);

	generate_modules(&native_directory, &modules);
}

fn create_native_method_table(
	generated_directory: &Path,
	modules: &[Module],
	symbol_collector: &mut SymbolCollector,
) {
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
			build_map_inserts(&mut init_fn_file, module, class, symbol_collector);
		}
	}

	write!(init_fn_file, "\tmap\n}}").unwrap();
}

fn build_map_inserts(
	file: &mut File,
	module: &Module,
	class: &Class,
	symbol_collector: &mut SymbolCollector,
) {
	symbol_collector.add_class_name(module.name_for_class(&class.class_name));

	for member in &class.members {
		match member {
			Member::Method(method) => {
				if !method.modifiers.contains(AccessFlags::ACC_NATIVE) {
					continue;
				}

				symbol_collector.add_method(method, class);

				writeln!(
					file,
					"\tmap.insert({});",
					util::method_table_entry(&module.name, &class, method)
				)
				.unwrap();
			},
			Member::Class(class_) => build_map_inserts(file, module, class_, symbol_collector),
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
