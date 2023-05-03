use super::attribute;
use crate::constant_pool::ConstantPool;
use crate::fieldinfo::FieldInfo;

use std::io::Read;

use common::traits::JavaReadExt;

pub fn read_field_info<R>(reader: &mut R, constant_pool: &ConstantPool) -> FieldInfo
where
	R: Read,
{
	let access_flags = reader.read_u2();
	let name_index = reader.read_u2();
	let descriptor_index = reader.read_u2();

	let attributes_count = reader.read_u2();
	let mut attributes = Vec::with_capacity(attributes_count as usize);

	for _ in 0..attributes_count {
		attributes.push(attribute::read_attribute(reader, constant_pool))
	}

	FieldInfo {
		access_flags,
		name_index,
		descriptor_index,
		attributes,
	}
}