use crate::attribute::Attribute;
use crate::types::u2;

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.5
#[derive(Debug, Clone, PartialEq)]
pub struct FieldInfo {
	pub access_flags: u2,
	pub name_index: u2,
	pub descriptor_index: u2,
	pub attributes: Vec<Attribute>,
}
