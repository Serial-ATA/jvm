use super::Location;
use crate::AttributeType;

use std::io::Read;

use common::traits::JavaReadExt;

const VALID_LOCATIONS: &[Location] = &[Location::FieldInfo];

pub fn read<R>(reader: &mut R, location: Location) -> AttributeType
where
	R: Read,
{
	location.verify_valid(VALID_LOCATIONS);
	AttributeType::ConstantValue {
		constantvalue_index: reader.read_u2(),
	}
}
