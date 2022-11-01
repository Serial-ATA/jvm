use super::reference::{ClassRef, FieldRef};
use crate::stack::operand_stack::Operand;

use classfile::fieldinfo::ACC_STATIC;
use classfile::traits::PtrType;
use classfile::types::u2;
use classfile::{ConstantPool, FieldInfo, FieldType};

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
	idx: usize, // Used for the `static_field_slots` field on `Class`
	pub class: ClassRef,
	pub access_flags: u2,
	pub name: Vec<u8>,
	pub descriptor: FieldType,
	pub constant_value_index: Option<u2>,
	// TODO
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
		self.access_flags & ACC_STATIC == ACC_STATIC
	}

	pub fn get_static_value(&self) -> Operand {
		assert!(self.is_static());
		self.class.get().static_field_slots[self.idx]
	}
}
