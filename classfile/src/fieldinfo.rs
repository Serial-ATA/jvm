use crate::attribute::{Attribute, AttributeType};

use std::io::Read;

use common::int_types::{u1, u2};
use common::traits::JavaReadExt;

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.5
#[derive(Debug, Clone, PartialEq)]
pub struct FieldInfo {
	pub access_flags: u2,
	pub name_index: u2,
	pub descriptor_index: u2,
	pub attributes: Vec<Attribute>,
}

#[rustfmt::skip]
impl FieldInfo {
	// Access flags
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.5-200-A.1

	pub const ACC_PUBLIC   : u2	= 0x0001; /* Declared public; may be accessed from outside its package. */
	pub const ACC_PRIVATE  : u2	= 0x0002; /* Declared private; accessible only within the defining class and other classes belonging to the same nest (§5.4.4). */
	pub const ACC_PROTECTED: u2 = 0x0004; /* Declared protected; may be accessed within subclasses. */
	pub const ACC_STATIC   : u2	= 0x0008; /* Declared static. */
	pub const ACC_FINAL    : u2	= 0x0010; /* Declared final; never directly assigned to after object construction (JLS §17.5). */
	pub const ACC_VOLATILE : u2	= 0x0040; /* Declared volatile; cannot be cached. */
	pub const ACC_TRANSIENT: u2 = 0x0080; /* Declared transient; not written or read by a persistent object manager. */
	pub const ACC_SYNTHETIC: u2 = 0x1000; /* Declared synthetic; not present in the source code. */
	pub const ACC_ENUM 	   : u2 = 0x4000; /* Declared as an element of an enum class. */
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
		self.access_flags & Self::ACC_STATIC == Self::ACC_STATIC
	}

	pub fn is_final(&self) -> bool {
		self.access_flags & Self::ACC_FINAL == Self::ACC_FINAL
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
	pub fn parse(bytes: &mut &[u1]) -> Self {
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
