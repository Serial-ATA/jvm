use super::Location;
use crate::attribute::{AttributeTag, AttributeType, MethodParameter, MethodParameters};
use crate::error::Result;

use std::io::Read;

use common::traits::JavaReadExt;

const VALID_LOCATIONS: &[Location] = &[Location::MethodInfo];

pub fn read<R>(reader: &mut R, location: Location) -> Result<AttributeType>
where
	R: Read,
{
	location.verify_valid(AttributeTag::MethodParameters, VALID_LOCATIONS)?;

	let parameters_count = reader.read_u1()?;
	let mut parameters = Vec::with_capacity(parameters_count as usize);

	for _ in 0..parameters_count {
		parameters.push(MethodParameter {
			name_index: reader.read_u2()?,
			access_flags: reader.read_u2()?,
		})
	}

	Ok(AttributeType::MethodParameters(MethodParameters {
		parameters,
	}))
}
