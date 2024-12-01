use super::attributes::{self, Location};
use crate::accessflags::MethodAccessFlags;
use crate::constant_pool::ConstantPool;
use crate::methodinfo::MethodInfo;
use crate::parse::error::Result;

use std::io::Read;

use common::traits::JavaReadExt;

pub fn read_method_info<R>(reader: &mut R, constant_pool: &ConstantPool) -> Result<MethodInfo>
where
	R: Read,
{
	let access_flags = reader.read_u2()?;
	let name_index = reader.read_u2()?;
	let descriptor_index = reader.read_u2()?;

	let attributes_count = reader.read_u2()?;
	let mut attributes = Vec::with_capacity(attributes_count as usize);

	for _ in 0..attributes_count {
		attributes.push(attributes::read_attribute(
			reader,
			constant_pool,
			Location::MethodInfo,
		)?);
	}

	Ok(MethodInfo {
		access_flags: MethodAccessFlags::from(access_flags),
		name_index,
		descriptor_index,
		attributes: attributes.into_boxed_slice(),
	})
}
