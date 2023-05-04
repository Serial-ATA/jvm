use crate::attribute::{Attribute, AttributeType, Code};
use crate::fieldinfo::FieldType;
use crate::LineNumber;

use common::int_types::{u1, u2};
use common::traits::JavaReadExt;

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.6
#[derive(Debug, Clone, PartialEq)]
pub struct MethodInfo {
	pub access_flags: u2,
	pub name_index: u2,
	pub descriptor_index: u2,
	pub attributes: Vec<Attribute>,
}

impl MethodInfo {
	pub fn get_line_number_table_attribute(&self) -> Option<Vec<LineNumber>> {
		for attr in &self.attributes {
			if let AttributeType::LineNumberTable {
				ref line_number_table,
			} = attr.info
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
}

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.3.3
#[derive(Debug, Clone, PartialEq)]
pub struct MethodDescriptor {
	pub parameters: Box<[FieldType]>,
	pub return_type: FieldType,
}

impl MethodDescriptor {
	pub fn parse(bytes: &mut &[u1]) -> Self {
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

		assert_eq!(
			bytes.read_u1(),
			b'(',
			"Unexpected character in method descriptor"
		);

		let mut parameters = Vec::new();
		while bytes[0] != b')' {
			parameters.push(FieldType::parse(bytes));
		}

		let _end_paren = bytes.read_u1();

		let return_type = FieldType::parse(bytes);
		assert!(bytes.is_empty(), "Method descriptor is too long!");

		Self {
			parameters: parameters.into_boxed_slice(),
			return_type,
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::fieldinfo::FieldType;
	use crate::methodinfo::MethodDescriptor;

	#[test]
	fn descriptor_no_parameters_void_return() {
		let descriptor = "()V";
		let method_descriptor = MethodDescriptor::parse(&mut descriptor.as_bytes());

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
					parameters: Box::new([FieldType::Char]),
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
					parameters: Box::new([FieldType::Int]),
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
			assert_eq!(MethodDescriptor::parse(&mut raw.as_bytes()), expected);
		}
	}

	#[test]
	fn descriptor_multiple_primitive_parameters() {
		let descriptor = "(BBCD)V";
		let method_descriptor = MethodDescriptor::parse(&mut descriptor.as_bytes());

		assert_eq!(
			&*method_descriptor.parameters,
			[
				FieldType::Byte,
				FieldType::Byte,
				FieldType::Char,
				FieldType::Double
			]
		);
		assert_eq!(method_descriptor.return_type, FieldType::Void);
	}

	#[test]
	fn descriptor_object_parameter() {
		let descriptor = "(Ljava/lang/Object;)V";
		let method_descriptor = MethodDescriptor::parse(&mut descriptor.as_bytes());

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
		let method_descriptor = MethodDescriptor::parse(&mut descriptor.as_bytes());

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
