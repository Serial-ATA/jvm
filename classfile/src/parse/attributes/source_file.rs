use super::Location;
use crate::attribute::{AttributeTag, AttributeType, SourceFile};
use crate::error::Result;

use std::io::Read;

use common::traits::JavaReadExt;

const VALID_LOCATIONS: &[Location] = &[Location::ClassFile];

pub fn read<R>(reader: &mut R, location: Location) -> Result<AttributeType>
where
	R: Read,
{
	location.verify_valid(AttributeTag::SourceFile, VALID_LOCATIONS)?;

	Ok(AttributeType::SourceFile(SourceFile {
		sourcefile_index: reader.read_u2()?,
	}))
}
