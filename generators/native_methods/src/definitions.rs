use crate::parse::{AccessFlags, Class, Member, Type};

use std::fmt::Write as _;
use std::fs::{File, OpenOptions};
use std::io::Write as _;
use std::path::Path;

use bitflags::Flags;

impl Type {
	fn expect_method(&self) -> &str {
		match self {
			Type::Boolean => "expect_int() == 1",
			Type::Byte => "expect_int() as ::jni::sys::jbyte",
			Type::Char => "expect_int() as ::jni::sys::jchar",
			Type::Int => "expect_int()",
			Type::Short => "expect_int() as ::jni::sys::jshort",

			Type::Double => "expect_double()",
			Type::Float => "expect_float()",
			Type::Long => "expect_long()",
			Type::Class(_) | Type::Array(_) => "expect_reference()",
			_ => unreachable!(),
		}
	}
}

pub fn generate_definitions_for_class(def_path: &Path, class: &Class) {
	let definitions_path = def_path.join(format!("{}.definitions.rs", class.class_name));
	let mut definitions_file = OpenOptions::new()
		.write(true)
		.truncate(true)
		.create(true)
		.open(definitions_path)
		.unwrap();

	writeln!(
		definitions_file,
		"#[allow(non_snake_case)]\npub mod definitions {{"
	)
	.unwrap();

	generate_methods_for_class(class, &mut definitions_file);

	for member in &class.members {
		if let Member::Class(c) = member {
			generate_methods_for_class(c, &mut definitions_file);
		}
	}

	writeln!(definitions_file, "}}").unwrap();
}

fn generate_methods_for_class(class: &Class, definitions_file: &mut File) {
	for method in class.methods().filter(|method| {
		method.name.is_some() && method.modifiers.contains(AccessFlags::ACC_NATIVE)
	}) {
		writeln!(
			definitions_file,
			"\tpub fn _{}(env: crate::native::JNIEnv, locals: \
			 crate::stack::local_stack::LocalStack) -> crate::native::NativeReturn {{",
			method.name()
		)
		.unwrap();

		let is_static = method.modifiers.contains(AccessFlags::ACC_STATIC);
		if !is_static {
			writeln!(
				definitions_file,
				"\t\tlet this: crate::heap::reference::Reference = locals[0].expect_reference();"
			)
			.unwrap();
		}

		for (idx, (ty, name)) in method.params.iter().enumerate() {
			writeln!(
				definitions_file,
				"\t\tlet {}_: {} = locals[{}].{};",
				name,
				if let Type::Array(_) = ty {
					String::from("crate::heap::reference::Reference")
				} else {
					ty.map_to_rust_ty()
				},
				if is_static { idx } else { idx + 1 },
				ty.expect_method()
			)
			.unwrap();
		}

		let mut method_call = String::new();
		write!(
			method_call,
			"\t\tsuper::{}(env,{}",
			method.name(),
			if is_static { "" } else { "this," }
		)
		.unwrap();

		for (_, name) in &method.params {
			write!(method_call, "{}_,", name).unwrap();
		}

		write!(method_call, ")").unwrap();

		match method.return_ty {
			Type::Void => {
				// Cannot implement From<()> for NativeReturn, need a special case for void returns
				writeln!(definitions_file, "{};\n\t\tNone\n\t}}", method_call).unwrap();
			},
			Type::Class(_) | Type::Array(_) => {
				writeln!(
					definitions_file,
					"Some(instructions::Operand::Reference({}))\n\t}}",
					method_call
				)
				.unwrap();
			},
			_ => {
				writeln!(definitions_file, "Some({}.into())\n\t}}", method_call).unwrap();
			},
		}
	}
}
