use crate::error::Result;
use crate::parse::attributes::Location;
use crate::{AttributeTag, AttributeType};

const VALID_LOCATIONS: &[Location] = &[
	Location::ClassFile,
	Location::FieldInfo,
	Location::MethodInfo,
];

pub fn read(location: Location) -> Result<AttributeType> {
	location.verify_valid(AttributeTag::Deprecated, VALID_LOCATIONS)?;
	Ok(AttributeType::Deprecated)
}
