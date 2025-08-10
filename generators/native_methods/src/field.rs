use crate::LINTS;
use crate::parse::{Class, Member};

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

/// Whether the class contains any fields
fn class_contains_fields(class: &Class) -> bool {
	for member in &class.members {
		match member {
			Member::Field(_) => return true,
			Member::Class(c) => {
				if class_contains_fields(c) {
					return true;
				}
			},
			Member::Method(_) => {},
		}
	}

	false
}

pub(crate) fn generate_native_constant_fields(class: &Class, def_path: &Path) {
	if !class_contains_fields(class) {
		return;
	}

	let mut file = create_constant_field_file_for_class(class, def_path);
	write_entries_for_class(file.as_mut(), def_path, class);

	if let Some(mut file) = file {
		writeln!(
			file,
			"}}\npub use __{}_field_constants::*;",
			class.sanitized_class_name()
		)
		.unwrap();
	}
}

fn create_constant_field_file_for_class(class: &Class, def_path: &Path) -> Option<File> {
	if !class
		.members
		.iter()
		.any(|member| matches!(member, Member::Field(_)))
	{
		return None;
	}

	let constant_fields_path = def_path.join(format!("{}.constants.rs", class.class_name));
	let mut constant_fields_file = OpenOptions::new()
		.write(true)
		.truncate(true)
		.create(true)
		.open(constant_fields_path)
		.unwrap();

	writeln!(
		constant_fields_file,
		r"mod __{}_field_constants {{
    {LINTS}",
		class.sanitized_class_name()
	)
	.unwrap();

	Some(constant_fields_file)
}

fn write_entries_for_class(mut file: Option<&mut File>, def_path: &Path, class: &Class) {
	for member in &class.members {
		match member {
			Member::Field(field) => {
				writeln!(
					file.as_mut().expect("file should exist"),
					"#[allow(non_upper_case_globals, dead_code)]\npub const {}: {} = {};",
					field.name,
					field.ty.map_to_rust_ty(),
					field.expr
				)
				.unwrap();
			},
			Member::Class(c) => {
				generate_native_constant_fields(c, def_path);
			},
			_ => {},
		}
	}
}
