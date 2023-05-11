use super::Location;
use crate::error::Result;
use crate::{AttributeTag, AttributeType};

use std::io::Read;

use common::box_slice;
use common::int_types::u4;

const VALID_LOCATIONS: &[Location] = &[Location::ClassFile];

pub fn read<R>(reader: &mut R, attribute_length: u4, location: Location) -> Result<AttributeType>
where
	R: Read,
{
	location.verify_valid(AttributeTag::SourceDebugExtension, VALID_LOCATIONS)?;

	Ok(AttributeType::SourceDebugExtension {
		debug_extension: {
			let mut debug_extension = box_slice![0; attribute_length as usize];
			reader.read_exact(&mut debug_extension)?;

			debug_extension
		},
	})
}
