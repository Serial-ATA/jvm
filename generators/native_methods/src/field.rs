use crate::parse::{Class, Member};

use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

/// Whether the class contains any fields
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

pub(crate) fn generate_native_constant_fields(class: &mut Class, def_path: &Path) {
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
