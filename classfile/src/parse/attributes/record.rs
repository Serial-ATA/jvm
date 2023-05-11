use super::{read_attribute, Location};
use crate::{AttributeType, ConstantPool, RecordComponentInfo};

use std::io::Read;

use common::traits::JavaReadExt;

const VALID_LOCATIONS: &[Location] = &[Location::ClassFile];

pub fn read<R>(reader: &mut R, constant_pool: &ConstantPool, location: Location) -> AttributeType
where
	R: Read,
{
	location.verify_valid(VALID_LOCATIONS);

	let components_count = reader.read_u2();
	let mut components = Vec::with_capacity(components_count as usize);
	for _ in 0..components_count {
		let name_index = reader.read_u2();
		let descriptor_index = reader.read_u2();

		let attributes_count = reader.read_u2();
		let mut attributes = Vec::with_capacity(attributes_count as usize);
		for _ in 0..attributes_count {
			attributes.push(read_attribute(
				reader,
				constant_pool,
				Location::RecordComponentInfo,
			))
		}

		components.push(RecordComponentInfo {
			name_index,
			descriptor_index,
			attributes,
		})
	}

	AttributeType::Record { components }
}
