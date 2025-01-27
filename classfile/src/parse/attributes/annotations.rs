use super::Location;
use crate::attribute::{
	Annotation, AttributeTag, AttributeType, ElementValue, ElementValuePair, ElementValueTag,
	ElementValueType, RuntimeInvisibleAnnotations, RuntimeVisibleAnnotations,
};
use crate::constant_pool::ConstantPool;
use crate::error::Result;

use std::io::Read;

use common::traits::JavaReadExt;

const VALID_LOCATIONS: &[Location] = &[
	Location::ClassFile,
	Location::FieldInfo,
	Location::MethodInfo,
	Location::RecordComponentInfo,
];

/// Read `AnnotationDefault` attribute
pub fn read_default<R>(
	reader: &mut R,
	constant_pool: &ConstantPool,
	location: Location,
) -> Result<AttributeType>
where
	R: Read,
{
	location.verify_valid(AttributeTag::AnnotationDefault, VALID_LOCATIONS)?;
	Ok(AttributeType::RuntimeVisibleAnnotations(
		RuntimeVisibleAnnotations {
			annotations: read_attribute_runtime_annotations(reader, constant_pool)?,
		},
	))
}

/// Read `RuntimeVisibleAnnotations` attribute
pub fn read_visible<R>(
	reader: &mut R,
	constant_pool: &ConstantPool,
	location: Location,
) -> Result<AttributeType>
where
	R: Read,
{
	location.verify_valid(AttributeTag::RuntimeVisibleAnnotations, VALID_LOCATIONS)?;
	Ok(AttributeType::RuntimeVisibleAnnotations(
		RuntimeVisibleAnnotations {
			annotations: read_attribute_runtime_annotations(reader, constant_pool)?,
		},
	))
}

/// Read `RuntimeInvisibleAnnotations` attribute
pub fn read_invisible<R>(
	reader: &mut R,
	constant_pool: &ConstantPool,
	location: Location,
) -> Result<AttributeType>
where
	R: Read,
{
	location.verify_valid(AttributeTag::RuntimeInvisibleAnnotations, VALID_LOCATIONS)?;
	Ok(AttributeType::RuntimeInvisibleAnnotations(
		RuntimeInvisibleAnnotations {
			annotations: read_attribute_runtime_annotations(reader, constant_pool)?,
		},
	))
}

pub mod type_annotations {
	use super::Location;
	use crate::attribute::{
		AttributeTag, AttributeType, RuntimeInvisibleTypeAnnotations, RuntimeVisibleTypeAnnotations,
	};
	use crate::constant_pool::ConstantPool;
	use crate::error::Result;

	use std::io::Read;

	const VALID_LOCATIONS: &[Location] = &[
		Location::ClassFile,
		Location::FieldInfo,
		Location::MethodInfo,
		Location::Code,
		Location::RecordComponentInfo,
	];

	/// Read `RuntimeVisibleTypeAnnotations` attribute
	pub fn read_visible<R>(
		reader: &mut R,
		constant_pool: &ConstantPool,
		location: Location,
	) -> Result<AttributeType>
	where
		R: Read,
	{
		location.verify_valid(AttributeTag::RuntimeVisibleTypeAnnotations, VALID_LOCATIONS)?;
		Ok(AttributeType::RuntimeVisibleTypeAnnotations(
			RuntimeVisibleTypeAnnotations {
				annotations: super::read_attribute_runtime_annotations(reader, constant_pool)?,
			},
		))
	}

	/// Read `RuntimeInvisibleTypeAnnotations` attribute
	pub fn read_invisible<R>(
		reader: &mut R,
		constant_pool: &ConstantPool,
		location: Location,
	) -> Result<AttributeType>
	where
		R: Read,
	{
		location.verify_valid(
			AttributeTag::RuntimeInvisibleTypeAnnotations,
			VALID_LOCATIONS,
		)?;
		Ok(AttributeType::RuntimeInvisibleTypeAnnotations(
			RuntimeInvisibleTypeAnnotations {
				annotations: super::read_attribute_runtime_annotations(reader, constant_pool)?,
			},
		))
	}
}

pub mod parameter_annotations {
	use super::Location;
	use crate::attribute::{
		AttributeTag, AttributeType, RuntimeInvisibleParameterAnnotations,
		RuntimeVisibleParameterAnnotations,
	};
	use crate::constant_pool::ConstantPool;
	use crate::error::Result;

	use std::io::Read;

	const VALID_LOCATIONS: &[Location] = &[Location::MethodInfo];

	/// Read `RuntimeVisibleParameterAnnotations` attribute
	pub fn read_visible<R>(
		reader: &mut R,
		constant_pool: &ConstantPool,
		location: Location,
	) -> Result<AttributeType>
	where
		R: Read,
	{
		location.verify_valid(
			AttributeTag::RuntimeVisibleParameterAnnotations,
			VALID_LOCATIONS,
		)?;
		Ok(AttributeType::RuntimeVisibleParameterAnnotations(
			RuntimeVisibleParameterAnnotations {
				annotations: super::read_attribute_runtime_annotations(reader, constant_pool)?,
			},
		))
	}

	/// Read `RuntimeInvisibleParameterAnnotations` attribute
	pub fn read_invisible<R>(
		reader: &mut R,
		constant_pool: &ConstantPool,
		location: Location,
	) -> Result<AttributeType>
	where
		R: Read,
	{
		location.verify_valid(
			AttributeTag::RuntimeInvisibleParameterAnnotations,
			VALID_LOCATIONS,
		)?;
		Ok(AttributeType::RuntimeInvisibleParameterAnnotations(
			RuntimeInvisibleParameterAnnotations {
				annotations: super::read_attribute_runtime_annotations(reader, constant_pool)?,
			},
		))
	}
}

fn read_attribute_runtime_annotations<R>(
	reader: &mut R,
	constant_pool: &ConstantPool,
) -> Result<Vec<Annotation>>
where
	R: Read,
{
	let num_annotations = reader.read_u2()?;
	let mut annotations = Vec::with_capacity(num_annotations as usize);

	for _ in 0..num_annotations {
		annotations.push(read_annotation(reader, constant_pool)?);
	}

	Ok(annotations)
}

fn read_elementvalue<R>(reader: &mut R, constant_pool: &ConstantPool) -> Result<ElementValue>
where
	R: Read,
{
	let tag = ElementValueTag::from(reader.read_u1()?);
	let ty = read_element_value_type(reader, tag, constant_pool)?;

	Ok(ElementValue { tag, ty })
}

#[rustfmt::skip]
fn read_element_value_type<R>(reader: &mut R, tag: ElementValueTag, constant_pool: &ConstantPool) -> Result<ElementValueType>
    where
        R: Read,
{
    match tag {
        // The const_value_index item is used if the tag item is one of B, C, D, F, I, J, S, Z, or s.
        ElementValueTag::Byte    => Ok(ElementValueType::Byte    { const_value_index: reader.read_u2()? }),
        ElementValueTag::Char    => Ok(ElementValueType::Char    { const_value_index: reader.read_u2()? }),
        ElementValueTag::Double  => Ok(ElementValueType::Double  { const_value_index: reader.read_u2()? }),
        ElementValueTag::Float   => Ok(ElementValueType::Float   { const_value_index: reader.read_u2()? }),
        ElementValueTag::Int     => Ok(ElementValueType::Int     { const_value_index: reader.read_u2()? }),
        ElementValueTag::Long    => Ok(ElementValueType::Long    { const_value_index: reader.read_u2()? }),
        ElementValueTag::Short   => Ok(ElementValueType::Short   { const_value_index: reader.read_u2()? }),
        ElementValueTag::Boolean => Ok(ElementValueType::Boolean { const_value_index: reader.read_u2()? }),
        ElementValueTag::String  => Ok(ElementValueType::String  { const_value_index: reader.read_u2()? }),

        // The enum_const_value item is used if the tag item is e.
        ElementValueTag::Enum => Ok(ElementValueType::Enum {
            type_name_index: reader.read_u2()?,
            const_value_index: reader.read_u2()?,
        }),

        // The class_info_index item is used if the tag item is c.
        ElementValueTag::Class => Ok(ElementValueType::Class {
            class_info_index: reader.read_u2()?,
        }),

        // The annotation_value item is used if the tag item is @.
        ElementValueTag::Annotation => Ok(ElementValueType::Annotation {
            annotation: read_annotation(reader, constant_pool)?,
        }),

        // The array_value item is used if the tag item is [.
        ElementValueTag::Array => {
            let num_values = reader.read_u2()?;
            let mut values = Vec::with_capacity(num_values as usize);

            for _ in 0..num_values {
                values.push(read_elementvalue(reader, constant_pool)?);
            }

            Ok(ElementValueType::Array { values })
        },
    }
}

fn read_annotation<R>(reader: &mut R, constant_pool: &ConstantPool) -> Result<Annotation>
where
	R: Read,
{
	let type_index = reader.read_u2()?;

	let num_element_value_pairs = reader.read_u2()?;
	let mut element_value_pairs = Vec::with_capacity(num_element_value_pairs as usize);

	for _ in 0..num_element_value_pairs {
		let element_name_index = reader.read_u2()?;

		let value = read_elementvalue(reader, constant_pool)?;

		element_value_pairs.push(ElementValuePair {
			element_name_index,
			value,
		})
	}

	Ok(Annotation {
		type_index,
		element_value_pairs,
	})
}
