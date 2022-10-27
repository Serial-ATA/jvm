use crate::attribute::{Attribute, AttributeType};

use std::io::Read;

use common::traits::JavaReadExt;
use common::types::u2;

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.5
pub const ACC_STATIC: u2 = 0x0008;

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.5
#[derive(Debug, Clone, PartialEq)]
pub struct FieldInfo {
	pub access_flags: u2,
	pub name_index: u2,
	pub descriptor_index: u2,
	pub attributes: Vec<Attribute>,
}

impl FieldInfo {
	pub fn get_constant_value_attribute(&self) -> Option<u2> {
		for attr in &self.attributes {
			if let AttributeType::ConstantValue {
				constantvalue_index,
			} = attr.info
			{
				return Some(constantvalue_index);
			}
		}

		None
	}

	pub fn is_static(&self) -> bool {
		self.access_flags & ACC_STATIC == ACC_STATIC
	}
}

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.3.2
#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
	Byte,
	Char,
	Double,
	Float,
	Int,
	Long,
	Short,
	Boolean,
	Void, // Only valid for method return types
	Object(String),
	Array(Box<FieldType>),
}

impl FieldType {
	pub fn parse(bytes: &mut &[u8]) -> Self {
		// FieldDescriptor:
		// 	FieldType
		//
		// FieldType:
		// 	BaseType
		// 	ObjectType
		// 	ArrayType

		match bytes.read_u1() {
			// BaseType:
			// 	(one of)
			// 	B C D F I J S Z
			b'B' => Self::Byte,
			b'C' => Self::Char,
			b'D' => Self::Double,
			b'F' => Self::Float,
			b'I' => Self::Int,
			b'J' => Self::Long,
			b'S' => Self::Short,
			b'Z' => Self::Boolean,
			// ObjectType:
			//  L ClassName ;
			b'L' => {
				let class_name = bytes
					.bytes()
					.flatten()
					.take_while(|b| *b != b';')
					.map(char::from)
					.collect::<String>();

				Self::Object(class_name)
			},
			// ArrayType:
			//  [ ComponentType
			// ComponentType:
			//  FieldType
			b'[' => {
				let component_type = Self::parse(bytes);

				Self::Array(Box::new(component_type))
			},
			b'V' => Self::Void,
			_ => panic!("Invalid field type descriptor"),
		}
	}

	pub fn is_byte(&self) -> bool {
		matches!(self, Self::Byte)
	}

	pub fn is_char(&self) -> bool {
		matches!(self, Self::Char)
	}

	pub fn is_double(&self) -> bool {
		matches!(self, Self::Double)
	}

	pub fn is_float(&self) -> bool {
		matches!(self, Self::Float)
	}

	pub fn is_int(&self) -> bool {
		matches!(self, Self::Int)
	}

	pub fn is_long(&self) -> bool {
		matches!(self, Self::Long)
	}

	pub fn is_class(&self) -> bool {
		matches!(self, Self::Object(_))
	}

	pub fn is_short(&self) -> bool {
		matches!(self, Self::Short)
	}

	pub fn is_boolean(&self) -> bool {
		matches!(self, Self::Boolean)
	}

	pub fn is_array(&self) -> bool {
		matches!(self, Self::Array(_))
	}
}
