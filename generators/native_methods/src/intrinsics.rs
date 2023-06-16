use crate::modules::Module;
use crate::parse::Method;

use std::collections::{HashMap, HashSet};

use std::fmt::Write as _;
use std::fs::OpenOptions;
use std::io::Write as _;
use std::path::Path;

macro_rules! generated_file_header {
	() => {
		r#"#[cfg_attr(rustfmt, rustfmt::skip)]

use ::common::int_types::u1;

const NUMBER_OF_INTRINSICS: u8 = {};
"#
	};
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct IntrinsicMethod {
	name: String,
	generated_name: String,
	descriptor: String,
	signature_symbol: String,
	name_symbol: String,
	intrinsic_flags: &'static str,
}

impl From<&Method> for IntrinsicMethod {
	fn from(value: &Method) -> Self {
		Self {
			name: value.name(),
			generated_name: value.generated_name().to_string(),
			descriptor: value.descriptor.to_string(),
			signature_symbol: value.signature_symbol_name(),
			name_symbol: value.name_symbol().to_string(),
			intrinsic_flags: value.intrinsic_flags(),
		}
	}
}

pub(crate) fn generate_intrinsics(generated_directory: &Path, modules: &[Module]) {
	let mut intrinsic_methods = HashMap::new();
	for module in modules {
		module.for_each_class(|class| {
			intrinsic_methods.extend(class.methods().filter_map(|method| {
				method.intrinsic_name(class).map(|id| {
					(
						id,
						(
							module.name_for_class(&class.class_name),
							IntrinsicMethod::from(method),
						),
					)
				})
			}))
		});
	}

	// Generate any additional symbols that we need
	generate_symbols(generated_directory, intrinsic_methods.values());

	let generated_file_path = generated_directory.join("intrinsics_generated.rs");
	let mut generated_file = OpenOptions::new()
		.write(true)
		.truncate(true)
		.create(true)
		.open(generated_file_path)
		.unwrap();

	// + 1 to account for the null ID
	let total_ids = intrinsic_methods.len() + 1;

	writeln!(
		&mut generated_file,
		"{}",
		format_args!(generated_file_header!(), total_ids)
	)
	.unwrap();

	writeln!(
		&mut generated_file,
		"{}",
		// Chain "None", since we need to at least have a "null" intrinsic
		create_intrinsic_name_table(
			std::iter::once("None").chain(intrinsic_methods.keys().map(String::as_str)),
			total_ids
		)
	)
	.unwrap();

	writeln!(
		&mut generated_file,
		"{}",
		create_intrinsic_id_enum(
			std::iter::once("None").chain(intrinsic_methods.keys().map(String::as_str))
		)
	)
	.unwrap();

	writeln!(
		&mut generated_file,
		"{}",
		create_method_mappings(intrinsic_methods.iter())
	)
	.unwrap();
}

/// Generates additional symbols, injecting them into the `vm_symbols::define_symbols!` call
/// in `runtime/src/symbols.rs`
fn generate_symbols<'a>(
	generated_directory: &Path,
	iter: impl Iterator<Item = &'a (String, IntrinsicMethod)> + Clone,
) {
	// ../../symbols/src/lib.rs
	let symbols_project_dir = generated_directory
		.parent()
		.unwrap()
		.parent()
		.unwrap()
		.join("symbols");
	let symbols_file_path = symbols_project_dir.join("src").join("lib.rs");

	let mut symbols_file_contents = std::fs::read_to_string(&symbols_file_path).unwrap();

	generate_class_name_symbols(
		generated_directory,
		&mut symbols_file_contents,
		iter.clone(),
	);
	generate_method_name_symbols(
		generated_directory,
		&mut symbols_file_contents,
		iter.clone(),
	);
	generate_method_signature_symbols(generated_directory, &mut symbols_file_contents, iter);

	let mut symbols_file = OpenOptions::new()
		.truncate(true)
		.write(true)
		.open(&symbols_file_path)
		.unwrap();
	symbols_file
		.write_all(symbols_file_contents.as_bytes())
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
		.contains(&format!("\"{}\"", check_for))
}

/// Generates `Symbol`s for intrinsic class names
///
/// For example, `java.lang.StringBuilder` will have a symbol created like so:
///
/// ```
/// vm_symbols::define_symbols! {
/// 	// ...
/// 	java_lang_StringBuilder: "java/lang/StringBuilder",
/// 	// ...
/// }
/// ```
fn generate_class_name_symbols<'a>(
	generated_directory: &Path,
	symbols_file_contents: &str,
	iter: impl Iterator<Item = &'a (String, IntrinsicMethod)>,
) {
	const SECTION_HEADER: &str = "// Classes";
	const MARKER_COMMENT: &str = "// -- GENERATED CLASS NAME MARKER, DO NOT DELETE --";

	let section_header_position =
		get_position_of_marker_comment(symbols_file_contents, SECTION_HEADER);
	let marker_comment_position =
		get_position_of_marker_comment(symbols_file_contents, MARKER_COMMENT);

	let mut class_names_to_add = HashSet::new();
	for (class_name, _) in iter {
		class_names_to_add.insert(class_name);
	}

	let mut class_name_symbols_file = OpenOptions::new()
		.write(true)
		.truncate(true)
		.create(true)
		.open(generated_directory.join("class_names.symbols"))
		.unwrap();

	for class_name in class_names_to_add {
		if !check_in_section(
			symbols_file_contents,
			section_header_position,
			marker_comment_position,
			class_name,
		) {
			writeln!(
				class_name_symbols_file,
				"{}: {:?},",
				class_name.replace('/', "_"),
				class_name
			)
			.unwrap();
		}
	}
}

/// Generates `Symbol`s for intrinsic method names
///
/// For example, `java.lang.Object#hashCode` will have a symbol created like so:
///
/// ```
/// vm_symbols::define_symbols! {
/// 	// ...
/// 	object_hashcode_name: "hashCode",
/// 	// ...
/// }
/// ```
fn generate_method_name_symbols<'a>(
	generated_directory: &Path,
	symbols_file_contents: &str,
	iter: impl Iterator<Item = &'a (String, IntrinsicMethod)>,
) {
	const SECTION_HEADER: &str = "// Methods";
	const MARKER_COMMENT: &str = "// -- GENERATED METHOD NAME MARKER, DO NOT DELETE --";

	let section_header_position =
		get_position_of_marker_comment(symbols_file_contents, SECTION_HEADER);
	let marker_comment_position =
		get_position_of_marker_comment(symbols_file_contents, MARKER_COMMENT);

	let mut methods_to_add = Vec::new();
	for (_, method) in iter {
		methods_to_add.push((&method.name_symbol, &method.generated_name));
	}

	methods_to_add.sort();
	methods_to_add.dedup();

	let mut method_name_symbols_file = OpenOptions::new()
		.write(true)
		.truncate(true)
		.create(true)
		.open(generated_directory.join("method_names.symbols"))
		.unwrap();

	for (name_symbol, generated_name) in methods_to_add {
		if !check_in_section(
			symbols_file_contents,
			section_header_position,
			marker_comment_position,
			generated_name,
		) {
			writeln!(
				method_name_symbols_file,
				"{}: {:?},",
				name_symbol, generated_name
			)
			.unwrap();
		}
	}
}

/// Generates `Symbol`s for intrinsic method signatures
///
/// For example, a method with a signatures of `(ZZ)V` will have a symbol created like so:
///
/// ```
/// vm_symbols::define_symbols! {
/// 	// ...
/// 	bool_bool_void_signature: "(ZZ)V",
/// 	// ...
/// }
/// ```
fn generate_method_signature_symbols<'a>(
	generated_directory: &Path,
	symbols_file_contents: &str,
	iter: impl Iterator<Item = &'a (String, IntrinsicMethod)>,
) {
	const SECTION_HEADER: &str = "// Signatures";
	const MARKER_COMMENT: &str = "// -- GENERATED METHOD SIGNATURE MARKER, DO NOT DELETE --";

	let section_header_position =
		get_position_of_marker_comment(symbols_file_contents, SECTION_HEADER);
	let marker_comment_position =
		get_position_of_marker_comment(symbols_file_contents, MARKER_COMMENT);

	let mut signatures_to_add = Vec::new();
	for (_, method) in iter {
		signatures_to_add.push((&method.signature_symbol, &method.descriptor));
	}

	signatures_to_add.sort();
	signatures_to_add.dedup();

	let mut signature_symbols_file = OpenOptions::new()
		.write(true)
		.truncate(true)
		.create(true)
		.open(generated_directory.join("signature_names.symbols"))
		.unwrap();

	for (signature_symbol, descriptor) in signatures_to_add {
		if !check_in_section(
			symbols_file_contents,
			section_header_position,
			marker_comment_position,
			descriptor,
		) {
			writeln!(
				signature_symbols_file,
				"{}: {:?},",
				signature_symbol, descriptor
			)
			.unwrap();
		}
	}
}

fn create_intrinsic_name_table<'a>(
	intrinsic_ids: impl Iterator<Item = &'a str>,
	total_ids: usize,
) -> String {
	let mut intrinsic_name_table = format!(
		"pub(in crate::native) static INTRINSIC_NAME_TABLE: [&[u1]; {}] = [\n",
		total_ids
	);
	for id in intrinsic_ids {
		writeln!(intrinsic_name_table, "\t&{:?},", id.as_bytes()).unwrap();
	}

	writeln!(intrinsic_name_table, "];").unwrap();
	intrinsic_name_table
}

/// Creates the `IntrinsicId` enum
fn create_intrinsic_id_enum<'a>(intrinsic_ids: impl Iterator<Item = &'a str>) -> String {
	let mut intrinsic_name_enum = String::from(
		"#[allow(non_camel_case_types)]\n#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, \
		 Debug)]\npub enum IntrinsicId {\n",
	);
	for id in intrinsic_ids {
		writeln!(intrinsic_name_enum, "\t{},", id).unwrap();
	}

	intrinsic_name_enum.push('}');
	intrinsic_name_enum
}

/// Creates the `IntrinsicId::for_method` method
fn create_method_mappings<'a>(
	intrinsic_ids: impl Iterator<Item = (&'a String, &'a (String, IntrinsicMethod))>,
) -> String {
	let mut intrinsic_id_method_mapping = String::from(
		r#"
impl IntrinsicId {
	/// Attempt to map the method to an `IntrinsicId`
	pub fn for_method(class: Symbol, method_name: Symbol, signature: Symbol, flags: MethodAccessFlags) -> Self {
		use symbols::sym;

		// Creates a unique ID for a method using its class, name, and signature
		macro_rules! intrinsics_id3 {
			($class:expr, $method_name:expr, $method_signature:expr) => {
				(($method_signature.as_u32() as u64) +
					(($method_name.as_u32()  as u64) <<    Symbol::LOG2_LIMIT) +
					(($class .as_u32()       as u64) << (2*Symbol::LOG2_LIMIT)))
			};
		}

		match intrinsics_id3!(class, method_name, signature) {
"#,
	);

	for (id, (class_name, method)) in intrinsic_ids {
		writeln!(
			intrinsic_id_method_mapping,
			"\t\t\tid3 if id3 == intrinsics_id3!(sym!({}), sym!({}), sym!({})) => {{ if \
			 IntrinsicFlags::from(flags) == {} {{ return IntrinsicId::{} }} }}",
			class_name.replace('/', "_"),
			method.name_symbol,
			method.signature_symbol,
			method.intrinsic_flags,
			id,
		)
		.unwrap();
	}

	writeln!(
		intrinsic_id_method_mapping,
		"\t\t\t_ => return IntrinsicId::None,\n\t\t}}\n\n\t\treturn IntrinsicId::None;\n\t}}\n}}"
	)
	.unwrap();
	intrinsic_id_method_mapping
}
