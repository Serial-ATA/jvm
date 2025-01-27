use crate::attribute::{AttributeTag, AttributeType};
use crate::error::Result;
use crate::parse::attributes::Location;

const VALID_LOCATIONS: &[Location] = &[
	Location::ClassFile,
	Location::FieldInfo,
	Location::MethodInfo,
];

pub fn read(location: Location) -> Result<AttributeType> {
	location.verify_valid(AttributeTag::Synthetic, VALID_LOCATIONS)?;
	Ok(AttributeType::Synthetic)
}
