use crate::accessflags::FieldAccessFlags;
use crate::attribute::{Attribute, ConstantValue};
use crate::error::Result;
use std::borrow::Cow;
use std::fmt::Display;
use std::io::Read;

use common::int_types::{u1, u2};
use common::traits::JavaReadExt;

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.5
#[derive(Debug, Clone, PartialEq)]
pub struct FieldInfo {
	pub access_flags: FieldAccessFlags,
	pub name_index: u2,
	pub descriptor_index: u2,
	pub attributes: Box<[Attribute]>,
}

impl FieldInfo {
	pub fn get_constant_value_attribute(&self) -> Option<ConstantValue> {
		self.attributes
			.iter()
			.find_map(|attr| attr.constant_value())
	}
}

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.3.2
#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
	Byte,
	Character,
	Double,
	Float,
	Integer,
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
			b'C' => Ok(Self::Character),
			b'D' => Ok(Self::Double),
			b'F' => Ok(Self::Float),
			b'I' => Ok(Self::Integer),
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
			_ => true,
		}
	}

	pub fn is_byte(&self) -> bool {
		matches!(self, Self::Byte)
	}

	pub fn is_char(&self) -> bool {
		matches!(self, Self::Character)
	}

	pub fn is_double(&self) -> bool {
		matches!(self, Self::Double)
	}

	pub fn is_float(&self) -> bool {
		matches!(self, Self::Float)
	}

	pub fn is_int(&self) -> bool {
		matches!(self, Self::Integer)
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

	pub fn is_void(&self) -> bool {
		matches!(self, Self::Void)
	}

	pub fn is_array(&self) -> bool {
		matches!(self, Self::Array(_))
	}

	pub fn is_array_of_class(&self, name: &[u1]) -> bool {
		let FieldType::Array(component) = self else {
			return false;
		};

		let FieldType::Object(component_class_name) = &**component else {
			return false;
		};

		name == &**component_class_name
	}

	pub fn as_signature(&self) -> Cow<'static, str> {
		match self {
			Self::Byte => "B".into(),
			Self::Character => "C".into(),
			Self::Double => "D".into(),
			Self::Float => "F".into(),
			Self::Integer => "I".into(),
			Self::Long => "J".into(),
			Self::Short => "S".into(),
			Self::Boolean => "Z".into(),
			Self::Void => "V".into(),
			Self::Object(name) => format!("L{};", String::from_utf8_lossy(name)).into(),
			Self::Array(component) => format!("[{}", component.as_signature()).into(),
		}
	}

	pub fn as_java_type(&self) -> Cow<'static, str> {
		match self {
			Self::Byte => "byte".into(),
			Self::Character => "char".into(),
			Self::Double => "double".into(),
			Self::Float => "float".into(),
			Self::Integer => "int".into(),
			Self::Long => "long".into(),
			Self::Short => "short".into(),
			Self::Boolean => "boolean".into(),
			Self::Void => "void".into(),
			Self::Object(name) => {
				format!("{}", String::from_utf8_lossy(name).replace('/', ".")).into()
			},
			Self::Array(component) => format!("[{}", component.as_java_type()).into(),
		}
	}

	/// Get the [array type code] for this field type
	///
	/// This is only applicable for primitive types
	///
	/// [array type code]: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.newarray
	pub fn as_array_type_code(&self) -> Option<u8> {
		match self {
			FieldType::Boolean => Some(4),
			FieldType::Character => Some(5),
			FieldType::Float => Some(6),
			FieldType::Double => Some(7),
			FieldType::Byte => Some(8),
			FieldType::Short => Some(9),
			FieldType::Integer => Some(10),
			FieldType::Long => Some(11),
			_ => None,
		}
	}

	/// The number of slots this type takes up in the [operand stack]
	///
	/// [operand stack]: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.6.2
	pub fn stack_size(&self) -> u1 {
		match self {
			FieldType::Double | FieldType::Long => 2,
			_ => 1,
		}
	}
}

impl Display for FieldType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(&self.as_signature())
	}
}
