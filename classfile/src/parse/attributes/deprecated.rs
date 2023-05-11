use crate::parse::attributes::Location;
use crate::AttributeType;

const VALID_LOCATIONS: &[Location] = &[
	Location::ClassFile,
	Location::FieldInfo,
	Location::MethodInfo,
];

pub fn read(location: Location) -> AttributeType {
	location.verify_valid(VALID_LOCATIONS);
	AttributeType::Deprecated
}
