use super::Location;
use crate::error::Result;
use crate::{AttributeTag, AttributeType};

use std::io::Read;

use common::traits::JavaReadExt;

const VALID_LOCATIONS: &[Location] = &[Location::MethodInfo];

pub fn read<R>(reader: &mut R, location: Location) -> Result<AttributeType>
where
	R: Read,
{
	location.verify_valid(AttributeTag::Exceptions, VALID_LOCATIONS)?;

	let number_of_exceptions = reader.read_u2()?;
	let mut exception_index_table = Vec::with_capacity(number_of_exceptions as usize);

	for _ in 0..number_of_exceptions {
		exception_index_table.push(reader.read_u2()?);
	}

	Ok(AttributeType::Exceptions {
		exception_index_table,
	})
}
