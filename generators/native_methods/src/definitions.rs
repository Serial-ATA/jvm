use crate::parse::{AccessFlags, Class, Type};

use std::fmt::Write as _;
use std::fs::OpenOptions;
use std::io::Write as _;
use std::path::Path;

use bitflags::Flags;

impl Type {
	fn expect_method(&self) -> &str {
		match self {
			Type::Boolean => "expect_int() == 1",

			Type::Byte | Type::Char | Type::Int | Type::Short => "expect_int()",
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

	for method in class.methods().filter(|method| {
		method.name.is_some()
			&& method.name.as_deref() != Some("registerNatives")
			&& method.modifiers.contains(AccessFlags::ACC_NATIVE)
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
				ty.map_to_rust_ty(),
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

		if method.return_ty == Type::Void {
			// Cannot implement From<()> for NativeReturn, need a special case for void returns
			writeln!(definitions_file, "{};\n\t\tNone\n\t}}", method_call).unwrap();
		} else {
			writeln!(definitions_file, "Some({}.into())\n\t}}", method_call).unwrap();
		}
	}

	writeln!(definitions_file, "}}").unwrap();
}
