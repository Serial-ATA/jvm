use super::Location;
use crate::attribute::{AttributeTag, AttributeType, Signature};
use crate::error::Result;

use std::io::Read;

use common::traits::JavaReadExt;

const VALID_LOCATIONS: &[Location] = &[
	Location::ClassFile,
	Location::FieldInfo,
	Location::MethodInfo,
	Location::RecordComponentInfo,
];

pub fn read<R>(reader: &mut R, location: Location) -> Result<AttributeType>
where
	R: Read,
{
	location.verify_valid(AttributeTag::Signature, VALID_LOCATIONS)?;
	Ok(AttributeType::Signature(Signature {
		signature_index: reader.read_u2()?,
	}))
}
