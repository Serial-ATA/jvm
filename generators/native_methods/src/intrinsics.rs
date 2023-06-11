use crate::modules::Module;

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

pub(crate) fn generate_intrinsics(native_directory: &Path, modules: &[Module]) {
	// We need to at least have a "null" intrinsic
	let mut intrinsic_ids = vec![String::from("None")];

	let mut intrinsic_methods = Vec::new();
	for module in modules {
		module.for_each_class(|class| {
			intrinsic_methods.extend(
				class
					.methods()
					.filter_map(|method| method.intrinsic_name(class)),
			)
		});
	}

	intrinsic_ids.append(&mut intrinsic_methods);

	let generated_file_path = native_directory.join("intrinsics_generated.rs");
	let mut generated_file = OpenOptions::new()
		.write(true)
		.truncate(true)
		.create(true)
		.open(generated_file_path)
		.unwrap();

	writeln!(
		&mut generated_file,
		"{}",
		format_args!(generated_file_header!(), intrinsic_ids.len())
	)
	.unwrap();

	writeln!(
		&mut generated_file,
		"{}",
		create_intrinsic_name_table(&intrinsic_ids)
	)
	.unwrap();

	writeln!(
		&mut generated_file,
		"{}",
		create_intrinsic_id_enum(&intrinsic_ids)
	)
	.unwrap();

	// TODO: intrinsic flags
	//       method.intrinsic_flags()
}

fn create_intrinsic_name_table(intrinsic_ids: &[String]) -> String {
	let mut intrinsic_name_table = format!(
		"pub(in crate::native) static INTRINSIC_NAME_TABLE: [&[u1]; {}] = [\n",
		intrinsic_ids.len()
	);
	for id in intrinsic_ids {
		writeln!(intrinsic_name_table, "\t&{:?},", id.as_bytes()).unwrap();
	}

	writeln!(intrinsic_name_table, "];").unwrap();
	intrinsic_name_table
}

fn create_intrinsic_id_enum(intrinsic_ids: &[String]) -> String {
	let mut intrinsic_name_enum =
		String::from("#[allow(non_camel_case_types)]\n#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]\npub enum IntrinsicId {\n");
	for id in intrinsic_ids {
		writeln!(intrinsic_name_enum, "\t{},", id).unwrap();
	}

	intrinsic_name_enum.push('}');
	intrinsic_name_enum
}
