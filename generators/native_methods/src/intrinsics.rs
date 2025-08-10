use crate::error::Result;
use crate::modules::Module;
use crate::parse::{AccessFlags, Method};
use crate::{LINTS, SymbolCollector};

use std::collections::HashMap;
use std::fmt::Write as _;
use std::fs::OpenOptions;
use std::io::Write as _;
use std::path::Path;

macro_rules! generated_file_header {
	() => {
		r"#![cfg_attr(rustfmt, rustfmt::skip)]

use crate::native::intrinsics::IntrinsicFlags;

use classfile::accessflags::MethodAccessFlags;
use ::common::int_types::u1;

const NUMBER_OF_INTRINSICS: u8 = {};
"
	};
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct IntrinsicMethodDefinition {
	name: String,
	generated_name: String,
	descriptor: String,
	signature_symbol: String,
	name_symbol: String,
	intrinsic_flags: &'static str,
	is_intrinsic: bool,
	is_native: bool,
}

impl From<&Method> for IntrinsicMethodDefinition {
	fn from(value: &Method) -> Self {
		Self {
			name: value.name(),
			generated_name: value.generated_name().to_string(),
			descriptor: value.descriptor.clone(),
			signature_symbol: value.signature_symbol_name(),
			name_symbol: value.name_symbol().clone(),
			intrinsic_flags: value.intrinsic_flags(),
			is_intrinsic: value.is_intrinsic_candidate,
			is_native: value.modifiers.contains(AccessFlags::ACC_NATIVE),
		}
	}
}

pub(crate) fn generate_intrinsics(
	generated_directory: &Path,
	modules: &[Module],
	symbol_collector: &mut SymbolCollector,
) -> Result<()> {
	let mut intrinsic_methods = HashMap::new();
	for module in modules {
		module.for_each_class(|class| {
			intrinsic_methods.extend(
				class
					.methods()
					.filter(|&method| method.is_intrinsic_candidate)
					.map(|method| {
						symbol_collector.add_class_name(module.name_for_class(&class.class_name));
						symbol_collector.add_method(method);

						(
							method.full_name_symbol(class),
							(
								module.name_for_class(&class.class_name),
								IntrinsicMethodDefinition::from(method),
							),
						)
					}),
			)
		});
	}

	let generated_file_path = generated_directory.join("intrinsics_generated.rs");
	let mut generated_file = OpenOptions::new()
		.write(true)
		.truncate(true)
		.create(true)
		.open(generated_file_path)?;

	// + 1 to account for the null ID
	let total_ids = intrinsic_methods.len() + 1;

	let mut intrinsic_methods_sorted = intrinsic_methods.into_iter().collect::<Vec<_>>();
	intrinsic_methods_sorted.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));

	writeln!(
		&mut generated_file,
		r"mod __generated_intrinsics {{
    {LINTS}{}",
		format_args!(generated_file_header!(), total_ids)
	)?;

	writeln!(
		&mut generated_file,
		"{}",
		// Chain "None", since we need to at least have a "null" intrinsic
		create_intrinsic_name_table(
			std::iter::once("None").chain(intrinsic_methods_sorted.iter().map(|(k, _)| k.as_str())),
			total_ids
		)?
	)?;

	writeln!(
		&mut generated_file,
		"{}",
		create_intrinsic_id_enum(
			std::iter::once("None").chain(intrinsic_methods_sorted.iter().map(|(k, _)| k.as_str()))
		)?
	)?;

	writeln!(
		&mut generated_file,
		"{}",
		create_method_mappings(intrinsic_methods_sorted.iter())?
	)?;

	writeln!(
		&mut generated_file,
		r"}}
pub use __generated_intrinsics::*;",
	)?;

	Ok(())
}

fn create_intrinsic_name_table<'a>(
	intrinsic_ids: impl Iterator<Item = &'a str>,
	total_ids: usize,
) -> Result<String> {
	let mut intrinsic_name_table = format!(
		"pub(in crate::native) static INTRINSIC_NAME_TABLE: [&[u1]; {}] = [\n",
		total_ids
	);
	for id in intrinsic_ids {
		writeln!(intrinsic_name_table, "\t&{:?},", id.as_bytes())?;
	}

	writeln!(intrinsic_name_table, "];")?;
	Ok(intrinsic_name_table)
}

/// Creates the `IntrinsicId` enum
fn create_intrinsic_id_enum<'a>(intrinsic_ids: impl Iterator<Item = &'a str>) -> Result<String> {
	let mut intrinsic_name_enum = String::from(
		r"#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum IntrinsicId {
",
	);
	for id in intrinsic_ids {
		writeln!(intrinsic_name_enum, "\t{},", id)?;
	}

	intrinsic_name_enum.push('}');
	Ok(intrinsic_name_enum)
}

/// Creates the `IntrinsicId::for_method` method
fn create_method_mappings<'a>(
	intrinsic_ids: impl Iterator<Item = &'a (String, (String, IntrinsicMethodDefinition))>,
) -> Result<String> {
	let mut intrinsic_id_method_mapping = String::from(
		r"
impl IntrinsicId {
	/// Attempt to map the method to an `IntrinsicId`
	pub fn for_method(class: crate::symbols::Symbol, method_name: crate::symbols::Symbol, signature: crate::symbols::Symbol, flags: MethodAccessFlags) -> Self {
		use crate::symbols::sym;

		// Creates a unique ID for a method using its class, name, and signature
		macro_rules! intrinsics_id3 {
			($class:expr, $method_name:expr, $method_signature:expr) => {
				(($method_signature.as_u32() as u64) +
					(($method_name.as_u32()  as u64) <<    crate::symbols::Symbol::PRE_INTERNED_LIMIT_LOG2) +
					(($class .as_u32()       as u64) << (2*crate::symbols::Symbol::PRE_INTERNED_LIMIT_LOG2)))
			};
		}

		match intrinsics_id3!(class, method_name, signature) {
",
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
		)?;
	}

	writeln!(
		intrinsic_id_method_mapping,
		"\t\t\t_ => return IntrinsicId::None,\n\t\t}}\n\n\t\treturn IntrinsicId::None;\n\t}}\n}}"
	)?;
	Ok(intrinsic_id_method_mapping)
}
