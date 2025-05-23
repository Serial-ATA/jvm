use crate::accessflags::MethodAccessFlags;
use crate::attribute::resolved::ResolvedAnnotation;
use crate::attribute::{Attribute, AttributeType, Code, LineNumber, LineNumberTable};
use crate::constant_pool::ConstantPool;
use crate::error::ClassFileParseError;
use crate::fieldinfo::FieldType;
use crate::parse::error::Result;

use common::int_types::{u1, u2};
use common::traits::JavaReadExt;

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.6
#[derive(Debug, Clone, PartialEq)]
pub struct MethodInfo {
	pub access_flags: MethodAccessFlags,
	pub name_index: u2,
	pub descriptor_index: u2,
	pub attributes: Box<[Attribute]>,
}

impl MethodInfo {
	pub fn get_line_number_table_attribute(&self) -> Option<Vec<LineNumber>> {
		for attr in &self.attributes {
			if let AttributeType::LineNumberTable(LineNumberTable {
				ref line_number_table,
			}) = attr.info
			{
				return Some(line_number_table.clone());
			}
		}

		None
	}

	pub fn get_code_attribute(&self) -> Option<Code> {
		for attr in &self.attributes {
			if let AttributeType::Code(ref code) = attr.info {
				return Some(code.clone());
			}
		}

		None
	}

	pub fn runtime_visible_annotations<'a>(
		&'a self,
		constant_pool: &'a ConstantPool,
	) -> Option<impl Iterator<Item = ResolvedAnnotation> + 'a> {
		for attr in &self.attributes {
			let Some(raw_annotations) = attr.runtime_visible_annotations() else {
				continue;
			};

			let iter = raw_annotations
				.annotations
				.iter()
				.map(move |anno| ResolvedAnnotation::resolve_from(anno, constant_pool));

			return Some(iter);
		}

		None
	}
}

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.3.3
#[derive(Debug, Clone, PartialEq)]
pub struct MethodDescriptor {
	pub parameters: Box<[FieldType]>,
	pub return_type: FieldType,
}

impl MethodDescriptor {
	pub fn parse(bytes: &mut &[u1]) -> Result<Self> {
		// MethodDescriptor:
		// 	( {ParameterDescriptor} ) ReturnDescriptor
		//
		// ParameterDescriptor:
		// 	FieldType
		//
		// ReturnDescriptor:
		// 	FieldType
		// 	VoidDescriptor
		//
		// VoidDescriptor:
		// 	V

		let start_paren = bytes.read_u1()?;
		if start_paren != b'(' {
			return Err(ClassFileParseError::InvalidMethodDescriptor(
				"Descriptor does not start with '('",
			));
		}

		let mut parameters = Vec::new();
		while bytes[0] != b')' {
			parameters.push(FieldType::parse(bytes)?);
		}

		let _end_paren = bytes.read_u1()?;

		let return_type = FieldType::parse(bytes)?;
		assert!(bytes.is_empty(), "Method descriptor is too long!");

		Ok(Self {
			parameters: parameters.into_boxed_slice(),
			return_type,
		})
	}
}

#[cfg(test)]
mod tests {
	use crate::fieldinfo::FieldType;
	use crate::methodinfo::MethodDescriptor;

	#[test]
	fn descriptor_no_parameters_void_return() {
		let descriptor = "()V";
		let method_descriptor = MethodDescriptor::parse(&mut descriptor.as_bytes()).unwrap();

		assert!(method_descriptor.parameters.is_empty());
		assert_eq!(method_descriptor.return_type, FieldType::Void);
	}

	#[test]
	fn descriptor_primitive_parameters_void_return() {
		let descriptors = vec![
			(
				"(B)V",
				MethodDescriptor {
					parameters: Box::new([FieldType::Byte]),
					return_type: FieldType::Void,
				},
			),
			(
				"(C)V",
				MethodDescriptor {
					parameters: Box::new([FieldType::Character]),
					return_type: FieldType::Void,
				},
			),
			(
				"(D)V",
				MethodDescriptor {
					parameters: Box::new([FieldType::Double]),
					return_type: FieldType::Void,
				},
			),
			(
				"(F)V",
				MethodDescriptor {
					parameters: Box::new([FieldType::Float]),
					return_type: FieldType::Void,
				},
			),
			(
				"(I)V",
				MethodDescriptor {
					parameters: Box::new([FieldType::Integer]),
					return_type: FieldType::Void,
				},
			),
			(
				"(J)V",
				MethodDescriptor {
					parameters: Box::new([FieldType::Long]),
					return_type: FieldType::Void,
				},
			),
			(
				"(S)V",
				MethodDescriptor {
					parameters: Box::new([FieldType::Short]),
					return_type: FieldType::Void,
				},
			),
			(
				"(Z)V",
				MethodDescriptor {
					parameters: Box::new([FieldType::Boolean]),
					return_type: FieldType::Void,
				},
			),
		];

		for (raw, expected) in descriptors {
			assert_eq!(
				MethodDescriptor::parse(&mut raw.as_bytes()).unwrap(),
				expected
			);
		}
	}

	#[test]
	fn descriptor_multiple_primitive_parameters() {
		let descriptor = "(BBCD)V";
		let method_descriptor = MethodDescriptor::parse(&mut descriptor.as_bytes()).unwrap();

		assert_eq!(
			&*method_descriptor.parameters,
			[
				FieldType::Byte,
				FieldType::Byte,
				FieldType::Character,
				FieldType::Double
			]
		);
		assert_eq!(method_descriptor.return_type, FieldType::Void);
	}

	#[test]
	fn descriptor_object_parameter() {
		let descriptor = "(Ljava/lang/Object;)V";
		let method_descriptor = MethodDescriptor::parse(&mut descriptor.as_bytes()).unwrap();

		assert_eq!(
			&*method_descriptor.parameters,
			&[FieldType::Object(
				b"java/lang/Object".to_vec().into_boxed_slice()
			)]
		);
		assert_eq!(method_descriptor.return_type, FieldType::Void);
	}

	#[test]
	fn descriptor_mutliple_object_parameters() {
		let descriptor = "(Ljava/lang/Object;Ljava/lang/Integer;Ljava/lang/String;)V";
		let method_descriptor = MethodDescriptor::parse(&mut descriptor.as_bytes()).unwrap();

		assert_eq!(
			&*method_descriptor.parameters,
			[
				FieldType::Object(b"java/lang/Object".to_vec().into_boxed_slice()),
				FieldType::Object(b"java/lang/Integer".to_vec().into_boxed_slice()),
				FieldType::Object(b"java/lang/String".to_vec().into_boxed_slice()),
			]
		);
		assert_eq!(method_descriptor.return_type, FieldType::Void);
	}
}
