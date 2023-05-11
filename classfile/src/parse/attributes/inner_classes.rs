use super::Location;
use crate::error::Result;
use crate::{AttributeTag, AttributeType, InnerClass};

use std::io::Read;

use common::traits::JavaReadExt;

const VALID_LOCATIONS: &[Location] = &[Location::ClassFile];

pub fn read<R>(reader: &mut R, location: Location) -> Result<AttributeType>
where
	R: Read,
{
	location.verify_valid(AttributeTag::InnerClasses, VALID_LOCATIONS)?;

	let number_of_classes = reader.read_u2()?;
	let mut classes = Vec::with_capacity(number_of_classes as usize);

	for _ in 0..number_of_classes {
		classes.push(InnerClass {
			inner_class_info_index: reader.read_u2()?,
			outer_class_info_index: reader.read_u2()?,
			inner_name_index: reader.read_u2()?,
			inner_class_access_flags: reader.read_u2()?,
		})
	}

	Ok(AttributeType::InnerClasses { classes })
}
