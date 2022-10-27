use super::reference::ClassRef;
use crate::stack::operand_stack::Operand;

use classfile::fieldinfo::ACC_STATIC;
use classfile::{ConstantPool, FieldInfo, FieldType};
use common::traits::PtrType;
use common::types::u2;

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
	) -> Self {
		let access_flags = field_info.access_flags;

		let name_index = field_info.name_index;
		let name = constant_pool.get_class_name(name_index - 1).to_vec();

		let descriptor_index = field_info.descriptor_index;
		let mut descriptor_bytes = constant_pool.get_constant_utf8(descriptor_index);

		let descriptor = FieldType::parse(&mut descriptor_bytes);
		let constant_value_index = field_info.get_constant_value_attribute();

		Self {
			idx,
			class,
			access_flags,
			name,
			descriptor,
			constant_value_index,
		}
	}

	pub fn is_static(&self) -> bool {
		self.access_flags & ACC_STATIC == ACC_STATIC
	}

	pub fn get_static_value(&self) -> Operand {
		assert!(self.is_static());
		self.class.get().static_field_slots[self.idx]
	}
}
