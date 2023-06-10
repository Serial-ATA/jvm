use super::Location;
use crate::attribute::PermittedSubclasses;
use crate::error::Result;
use crate::{AttributeTag, AttributeType};

use std::io::Read;

use common::traits::JavaReadExt;

const VALID_LOCATIONS: &[Location] = &[Location::ClassFile];

pub fn read<R>(reader: &mut R, location: Location) -> Result<AttributeType>
where
	R: Read,
{
	location.verify_valid(AttributeTag::PermittedSubclasses, VALID_LOCATIONS)?;

	let number_of_classes = reader.read_u2()?;
	let mut classes = Vec::with_capacity(number_of_classes as usize);

	for _ in 0..number_of_classes {
		classes.push(reader.read_u2()?)
	}

	Ok(AttributeType::PermittedSubclasses(PermittedSubclasses {
		classes,
	}))
}
