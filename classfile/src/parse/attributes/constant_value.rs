use super::Location;
use crate::attribute::{AttributeTag, AttributeType, ConstantValue};
use crate::error::Result;

use std::io::Read;

use common::traits::JavaReadExt;

const VALID_LOCATIONS: &[Location] = &[Location::FieldInfo];

pub fn read<R>(reader: &mut R, location: Location) -> Result<AttributeType>
where
	R: Read,
{
	location.verify_valid(AttributeTag::ConstantValue, VALID_LOCATIONS)?;
	Ok(AttributeType::ConstantValue(ConstantValue {
		constantvalue_index: reader.read_u2()?,
	}))
}
