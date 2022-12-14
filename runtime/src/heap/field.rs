use super::reference::{ClassRef, FieldRef, Reference};

use std::fmt::{Debug, Formatter};

use classfile::{ConstantPool, FieldInfo, FieldType};
use common::int_types::{u1, u2};
use instructions::Operand;

#[derive(Clone, PartialEq)]
pub struct Field {
	pub idx: usize, // Used to set the value on `ClassInstance`s
	pub class: ClassRef,
	pub access_flags: u2,
	pub name: Vec<u1>,
	pub descriptor: FieldType,
	pub constant_value_index: Option<u2>,
	// TODO
}

#[rustfmt::skip]
impl Field {
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

impl Field {
	pub fn new(
		idx: usize,
		class: ClassRef,
		field_info: &FieldInfo,
		constant_pool: &ConstantPool,
	) -> FieldRef {
		let access_flags = field_info.access_flags;

		let name_index = field_info.name_index;
		let name = constant_pool.get_constant_utf8(name_index).to_vec();

		let descriptor_index = field_info.descriptor_index;
		let mut descriptor_bytes = constant_pool.get_constant_utf8(descriptor_index);

		let descriptor = FieldType::parse(&mut descriptor_bytes);
		let constant_value_index = field_info.get_constant_value_attribute();

		FieldRef::new(Self {
			idx,
			class,
			access_flags,
			name,
			descriptor,
			constant_value_index,
		})
	}

	pub fn is_static(&self) -> bool {
		self.access_flags & Field::ACC_STATIC != 0
	}

	pub fn is_final(&self) -> bool {
		self.access_flags & Field::ACC_FINAL != 0
	}

	pub fn get_static_value(&self) -> Operand<Reference> {
		assert!(self.is_static());
		self.class.unwrap_class_instance().static_field_slots[self.idx].clone()
	}

	pub fn set_static_value(&self, value: Operand<Reference>) {
		assert!(self.is_static());
		self.class.unwrap_class_instance_mut().static_field_slots[self.idx] = value;
	}

	pub fn default_value_for_ty(ty: &FieldType) -> Operand<Reference> {
		match ty {
			FieldType::Byte
			| FieldType::Boolean
			| FieldType::Short
			| FieldType::Int
			| FieldType::Char => Operand::Int(0),
			FieldType::Long => Operand::Long(0),
			FieldType::Float => Operand::Float(0.),
			FieldType::Double => Operand::Double(0.),
			FieldType::Void | FieldType::Object(_) | FieldType::Array(_) => {
				Operand::Reference(Reference::Null)
			},
		}
	}
}

impl Debug for Field {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Field")
			.field("idx", &self.idx)
			.field("class", &self.class)
			.field("access_flags", &self.access_flags)
			.field("name", &unsafe {
				std::str::from_utf8_unchecked(&self.name)
			})
			.field("descriptor", &self.descriptor)
			.field("constant_value_index", &self.constant_value_index)
			.finish()
	}
}
