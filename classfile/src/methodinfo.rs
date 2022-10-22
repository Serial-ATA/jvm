use crate::types::u2;
use crate::attribute::Attribute;

// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.6
#[derive(Debug, Clone, PartialEq)]
pub struct MethodInfo {
    pub access_flags: u2,
    pub name_index: u2,
    pub descriptor_index: u2,
    pub attributes: Vec<Attribute>
}