use classfile::{ConstantPool, FieldInfo, FieldType};
use common::types::u2;

pub struct Field {
	pub access_flags: u2,
	pub name: Vec<u8>,
	pub descriptor: FieldType,
	// TODO
}

impl Field {
	pub fn new(field_info: &FieldInfo, constant_pool: &ConstantPool) -> Self {
		let access_flags = field_info.access_flags;

		let name_index = field_info.name_index;
		let name = constant_pool.get_class_name(name_index).to_vec();

		let descriptor_index = field_info.descriptor_index;
		let mut descriptor_bytes = constant_pool.get_constant_utf8(descriptor_index);

		let descriptor = FieldType::parse(&mut descriptor_bytes);

		Self {
			access_flags,
			name,
			descriptor,
		}
	}
}