use super::Location;
use crate::AttributeType;

use std::io::Read;

use common::int_types::u4;

const VALID_LOCATIONS: &[Location] = &[Location::ClassFile];

pub fn read<R>(reader: &mut R, attribute_length: u4, location: Location) -> AttributeType
where
	R: Read,
{
	location.verify_valid(VALID_LOCATIONS);

	AttributeType::SourceDebugExtension {
		debug_extension: {
			let mut debug_extension = vec![0; attribute_length as usize];
			reader.read_exact(&mut debug_extension).unwrap();

			debug_extension
		},
	}
}
