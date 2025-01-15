#![feature(extract_if)]

mod definitions;
mod error;
mod field;
mod intrinsics;
mod modules;
mod parse;
mod registernatives;
mod util;

use crate::intrinsics::generate_intrinsics;
use crate::modules::Module;
use crate::parse::{AccessFlags, Class, Member, Method};
use error::Result;

use std::collections::HashMap;
use std::fmt::Write as _;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

static CRATE_ROOT: &str = env!("CARGO_MANIFEST_DIR");
static INIT_FN_FILE_HEADER: &str = r#"#[allow(trivial_casts)]
fn init_native_method_table() -> HashMap<NativeMethodDef, NativeMethodPtr> {
	use symbols::sym;
	
	fn insert(map: &mut HashMap<NativeMethodDef, NativeMethodPtr>, key: NativeMethodDef, value: NativeMethodPtr) {
		let existing = map.insert(key, value);
		assert!(existing.is_none(), "Double registration of native method: {:?}", key);
	}
	
	let mut map = HashMap::new();
"#;

static MODULE_MARKER_START_COMMENT: &str = "// Module marker, do not remove";

#[derive(Default)]
struct SymbolCollector {
	class_name_symbols_to_add: HashMap<String, String>,
	method_name_symbols_to_add: HashMap<String, String>,
	method_signature_symbols_to_add: HashMap<String, String>,
}

// TODO: If a duplicate symbol value is found under another name, the generated files should use the defined name
impl SymbolCollector {
	pub fn add_class_name(&mut self, value: String) {
		let symbol_name = value.replace('/', "_").replace('$', "_");
		self.class_name_symbols_to_add.insert(symbol_name, value);
	}

	pub fn add_method(&mut self, method: &Method) {
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
		)
		.unwrap();

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
		)
		.unwrap();

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
		)
		.unwrap();
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
			.lines()
			.any(|line| line.starts_with(&format!("\t{check_for}:")))
	}

	fn generate_symbols_inner<'a>(
		symbol_iter: &HashMap<String, String>,
		section_header: &str,
		marker_comment: &str,
		generated_directory: &Path,
		file_name: &str,
		symbols_file_contents: &str,
	) -> Result<()> {
		let section_header_position =
			Self::get_position_of_marker_comment(symbols_file_contents, section_header);
		let marker_comment_position =
			Self::get_position_of_marker_comment(symbols_file_contents, marker_comment);

		let mut symbols_file = OpenOptions::new()
			.write(true)
			.truncate(true)
			.create(true)
			.open(generated_directory.join(file_name))?;

		for (symbol, value) in symbol_iter {
			if !Self::check_in_section(
				symbols_file_contents,
				section_header_position,
				marker_comment_position,
				symbol,
			) {
				writeln!(symbols_file, "{}: {:?},", symbol.replace('/', "_"), value)?;
			}
		}

		Ok(())
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
pub fn generate() -> Result<()> {
	let mut symbol_collector = SymbolCollector::default();

	let native_directory = get_runtime_native_directory();
	let generated_directory = get_generated_directory();
	let modules = modules::get_modules_from(
		&generated_directory,
		&native_directory,
		&mut symbol_collector,
	);

	generate_intrinsics(&generated_directory, &modules, &mut symbol_collector)?;
	create_native_method_table(&generated_directory, &modules, &mut symbol_collector)?;

	// Generate any new symbols we found that are not defined in `../../symbols/src/lib.rs`
	symbol_collector.generate_symbols(&generated_directory);

	generate_modules(&native_directory, &modules)?;

	Ok(())
}

fn create_native_method_table(
	generated_directory: &Path,
	modules: &[Module],
	symbol_collector: &mut SymbolCollector,
) -> Result<()> {
	let init_fn_file_path = generated_directory.join("native_init.rs");
	let mut init_fn_file = OpenOptions::new()
		.write(true)
		.truncate(true)
		.create(true)
		.open(init_fn_file_path)?;

	write!(init_fn_file, "{}", INIT_FN_FILE_HEADER)?;

	for module in modules {
		module.for_each_class(|class| {
			build_map_inserts(&mut init_fn_file, module, class, symbol_collector).unwrap()
		});
	}

	write!(init_fn_file, "\tmap\n}}")?;

	Ok(())
}

fn build_map_inserts(
	file: &mut File,
	module: &Module,
	class: &Class,
	symbol_collector: &mut SymbolCollector,
) -> Result<()> {
	symbol_collector.add_class_name(module.name_for_class(&class.class_name));

	for member in &class.members {
		let Member::Method(method) = member else {
			// Don't need to check for `Member::Class`, this was called inside `for_each_class`, which
			// already walks through members.
			continue;
		};

		if !method.modifiers.contains(AccessFlags::ACC_NATIVE) {
			continue;
		}

		symbol_collector.add_method(method);

		writeln!(
			file,
			"\tinsert(&mut map, {});",
			util::method_table_entry(&module.name, &class, method)
		)?;
	}

	Ok(())
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
fn generate_modules(native_directory: &Path, modules: &[Module]) -> Result<()> {
	let root_module_path = native_directory.join("mod.rs");
	let root_mod_file_content = std::fs::read_to_string(&root_module_path)?;

	let marker_comment_start_pos = root_mod_file_content
		.rfind(MODULE_MARKER_START_COMMENT)
		.expect("Can't find module marker comment");

	// Remove anything trailing the comment
	let mut root_mod_file_content_bytes = root_mod_file_content;

	let existing_modules = root_mod_file_content_bytes
		.split_off(marker_comment_start_pos + MODULE_MARKER_START_COMMENT.len());
	let generated_modules = create_modules_string(modules)?;

	// Don't update the modules so the build script won't run again
	if existing_modules.trim() == generated_modules.trim() {
		return Ok(());
	}

	write!(
		&mut root_mod_file_content_bytes,
		"\n{}\n",
		&generated_modules
	)?;
	std::fs::write(&root_module_path, &root_mod_file_content_bytes)
		.expect("Failed to write modules to native/mod.rs");

	Ok(())
}

fn create_modules_string(modules: &[Module]) -> Result<String> {
	let mut modules_str = String::new();

	let mut current_root = None;
	let mut previous_module = &modules[0];
	let mut current_depth = 0;
	for module in modules {
		let root = &module.components[0].name;

		// Onto a new root. The module list is sorted by roots, so we can just close to current
		// root, knowing there will be no more entries for it.
		let is_new_root = Some(root) != current_root;
		if is_new_root {
			current_root = Some(root);
			for depth in (0..current_depth).rev() {
				writeln!(modules_str, "{}}}", "\t".repeat(depth)).unwrap();
			}
			writeln!(modules_str).unwrap();
			current_depth = 0;
		}

		let mut common_root_depth = module.common_root_depth(previous_module) as usize;
		let needs_module_definition = common_root_depth != module.components.len() || is_new_root;
		if common_root_depth == module.components.len() {
			common_root_depth = 0;
		}

		for component in &module.components[common_root_depth..] {
			if needs_module_definition {
				writeln!(
					modules_str,
					"{}pub(crate) mod {} {{",
					"\t".repeat(current_depth),
					component.rust_name()
				)
				.unwrap();

				current_depth += 1;
			}

			for class in &component.classes {
				writeln!(
					modules_str,
					"{}pub(crate) mod {};",
					"\t".repeat(current_depth),
					&class.class_name
				)
				.unwrap();
			}
		}

		previous_module = module;
		current_depth = current_depth.saturating_sub(1);
		writeln!(modules_str, "{}}}", "\t".repeat(current_depth)).unwrap();
	}

	for depth in (0..current_depth).rev() {
		writeln!(modules_str, "{}}}", "\t".repeat(depth)).unwrap();
	}

	Ok(modules_str)
}
