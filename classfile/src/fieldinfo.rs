use crate::accessflags::FieldAccessFlags;
use crate::attribute::{Attribute, ConstantValue};
use crate::error::Result;

use std::io::Read;

use common::int_types::{u1, u2};
use common::traits::JavaReadExt;

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.5
#[derive(Debug, Clone, PartialEq)]
pub struct FieldInfo {
	pub access_flags: FieldAccessFlags,
	pub name_index: u2,
	pub descriptor_index: u2,
	pub attributes: Vec<Attribute>,
}

impl FieldInfo {
	pub fn get_constant_value_attribute(&self) -> Option<ConstantValue> {
		self.attributes
			.iter()
			.find_map(|attr| attr.constant_value())
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
	Object(Box<[u1]>),
	Array(Box<FieldType>),
}

impl FieldType {
	pub fn parse(bytes: &mut &[u1]) -> Result<Self> {
		// FieldDescriptor:
		// 	FieldType
		//
		// FieldType:
		// 	BaseType
		// 	ObjectType
		// 	ArrayType

		match bytes.read_u1()? {
			// BaseType:
			// 	(one of)
			// 	B C D F I J S Z
			b'B' => Ok(Self::Byte),
			b'C' => Ok(Self::Char),
			b'D' => Ok(Self::Double),
			b'F' => Ok(Self::Float),
			b'I' => Ok(Self::Int),
			b'J' => Ok(Self::Long),
			b'S' => Ok(Self::Short),
			b'Z' => Ok(Self::Boolean),
			// ObjectType:
			//  L ClassName ;
			b'L' => {
				let class_name = bytes
					.bytes()
					.flatten()
					.take_while(|b| *b != b';')
					.collect::<Box<[u1]>>();

				Ok(Self::Object(class_name))
			},
			// ArrayType:
			//  [ ComponentType
			// ComponentType:
			//  FieldType
			b'[' => {
				let component_type = Self::parse(bytes)?;

				Ok(Self::Array(Box::new(component_type)))
			},
			b'V' => Ok(Self::Void),
			_ => {
				// TODO: Error handling
				panic!("Invalid field type descriptor")
			},
		}
	}

	pub fn is_primitive(&self) -> bool {
		match self {
			Self::Object(_) | Self::Array(_) => false,
			Self::Void => unreachable!(),
			_ => true,
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

	pub fn is_class(&self, name: &[u1]) -> bool {
		matches!(self, Self::Object(obj) if &**obj == name)
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
