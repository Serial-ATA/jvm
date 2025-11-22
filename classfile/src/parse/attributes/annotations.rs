use super::Location;
use crate::attribute::{
	Annotation, AttributeTag, AttributeType, RuntimeInvisibleAnnotations, RuntimeVisibleAnnotations,
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
		AttributeTag, AttributeType, RuntimeInvisibleTypeAnnotations,
		RuntimeVisibleTypeAnnotations, TypeAnnotation,
	};
	use crate::constant_pool::ConstantPool;
	use crate::error::Result;

	use std::io::Read;

	use common::traits::JavaReadExt;

	const VALID_LOCATIONS: &[Location] = &[
		Location::ClassFile,
		Location::FieldInfo,
		Location::MethodInfo,
		Location::Code,
		Location::RecordComponentInfo,
	];

	fn read_attribute_runtime_type_annotations<R>(
		reader: &mut R,
		constant_pool: &ConstantPool,
	) -> Result<Vec<TypeAnnotation>>
	where
		R: Read,
	{
		let num_annotations = reader.read_u2()?;
		let mut annotations = Vec::with_capacity(num_annotations as usize);

		for _ in 0..num_annotations {
			annotations.push(TypeAnnotation::parse(reader, constant_pool)?);
		}

		Ok(annotations)
	}

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
				annotations: read_attribute_runtime_type_annotations(reader, constant_pool)?,
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
				annotations: read_attribute_runtime_type_annotations(reader, constant_pool)?,
			},
		))
	}
}

pub mod parameter_annotations {
	use super::Location;
	use crate::attribute::{
		Annotation, AttributeTag, AttributeType, ParameterAnnotations,
		RuntimeInvisibleParameterAnnotations, RuntimeVisibleParameterAnnotations,
	};
	use crate::constant_pool::ConstantPool;
	use crate::error::Result;

	use std::io::Read;

	use common::traits::JavaReadExt;

	const VALID_LOCATIONS: &[Location] = &[Location::MethodInfo];

	fn read_attribute_runtime_parameter_annotations<R>(
		reader: &mut R,
		constant_pool: &ConstantPool,
	) -> Result<ParameterAnnotations>
	where
		R: Read,
	{
		let num_parameters = reader.read_u1()?;
		let mut parameter_annotations = Vec::with_capacity(num_parameters as usize);

		for _ in 0..num_parameters {
			let num_annotations = reader.read_u2()?;
			let mut annotations = Vec::with_capacity(num_annotations as usize);

			for _ in 0..num_annotations {
				annotations.push(Annotation::parse(reader, constant_pool)?);
			}

			parameter_annotations.push(annotations.into_boxed_slice());
		}

		Ok(ParameterAnnotations {
			annotations: parameter_annotations.into_boxed_slice(),
		})
	}

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
				annotations: read_attribute_runtime_parameter_annotations(reader, constant_pool)?,
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
				annotations: read_attribute_runtime_parameter_annotations(reader, constant_pool)?,
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
		annotations.push(Annotation::parse(reader, constant_pool)?);
	}

	Ok(annotations)
}
