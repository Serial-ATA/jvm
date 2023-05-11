mod annotations;
mod bootstrap_methods;
mod code;
mod constant_value;
mod deprecated;
mod enclosing_method;
mod exceptions;
mod inner_classes;
mod line_number_table;
mod local_variable_table;
mod local_variable_type_table;
mod method_parameters;
mod module;
mod nest;
mod permitted_subclasses;
mod record;
mod signature;
mod source_debug_extension;
mod source_file;
mod stack_map_table;
mod synthetic;

use super::error::{ClassFileParseError, Result};
use crate::attribute::{Attribute, AttributeTag};
use crate::constant_pool::ConstantPool;

use std::io::Read;

use common::traits::JavaReadExt;

/// The location in which the attribute occurs
///
/// Valid locations are defined here: <https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.7-320>
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Location {
	ClassFile,
	FieldInfo,
	MethodInfo,
	RecordComponentInfo,
	Code,
}

impl Location {
	fn verify_valid(self, tag: AttributeTag, locations: &[Location]) -> Result<()> {
		if locations.contains(&self) {
			return Ok(());
		}

		Err(ClassFileParseError::InvalidLocation(tag, self))
	}
}

#[rustfmt::skip]
pub fn read_attribute<R>(
	reader: &mut R,
	constant_pool: &ConstantPool,
	location: Location,
) -> Result<Attribute>
where
	R: Read,
{
	let attribute_name_index = reader.read_u2()?;
	let attribute_name = constant_pool.get_constant_utf8(attribute_name_index);

	let attribute_length = reader.read_u4()?;

	let info = match AttributeTag::from(attribute_name) {
		AttributeTag::ConstantValue => constant_value::read(reader, location),
		AttributeTag::Code => code::read(reader, constant_pool, location),
		AttributeTag::StackMapTable => stack_map_table::read(reader, location),
		AttributeTag::Exceptions => exceptions::read(reader, location),
		AttributeTag::InnerClasses => inner_classes::read(reader, location),
		AttributeTag::EnclosingMethod => enclosing_method::read(reader, location),
		AttributeTag::Synthetic => synthetic::read(location),
		AttributeTag::Signature => signature::read(reader, location),
		AttributeTag::SourceFile => source_file::read(reader, location),
		AttributeTag::SourceDebugExtension => source_debug_extension::read(reader, attribute_length, location),
		AttributeTag::LineNumberTable => line_number_table::read(reader, location),
		AttributeTag::LocalVariableTable => local_variable_table::read(reader, location),
		AttributeTag::LocalVariableTypeTable => local_variable_type_table::read(reader, location),
		AttributeTag::Deprecated => deprecated::read(location),
		AttributeTag::RuntimeVisibleAnnotations => annotations::read_visible(reader, constant_pool, location),
		AttributeTag::RuntimeInvisibleAnnotations => annotations::read_invisible(reader, constant_pool, location),
		AttributeTag::RuntimeVisibleParameterAnnotations => annotations::parameter_annotations::read_visible(reader, constant_pool, location),
		AttributeTag::RuntimeInvisibleParameterAnnotations => annotations::parameter_annotations::read_invisible(reader, constant_pool, location),
		AttributeTag::RuntimeVisibleTypeAnnotations => annotations::type_annotations::read_visible(reader, constant_pool, location),
		AttributeTag::RuntimeInvisibleTypeAnnotations => annotations::type_annotations::read_invisible(reader, constant_pool, location),
		AttributeTag::AnnotationDefault => annotations::read_default(reader, constant_pool, location),
		AttributeTag::BootstrapMethods => bootstrap_methods::read(reader, location),
		AttributeTag::MethodParameters => method_parameters::read(reader, location),
		AttributeTag::Module => module::read(reader, location),
		AttributeTag::ModulePackages => module::read_packages(reader, location),
		AttributeTag::ModuleMainClass => module::read_main_class(reader, location),
		AttributeTag::NestHost => nest::read_host(reader, location),
		AttributeTag::NestMembers => nest::read_members(reader, location),
		AttributeTag::Record => record::read(reader, constant_pool, location),
		AttributeTag::PermittedSubclasses => permitted_subclasses::read(reader, location),
	}?;

	Ok(Attribute {
		attribute_name_index,
		info,
	})
}
