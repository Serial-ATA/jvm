use crate::types::{u1, u2};

// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7
#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub attribute_name_index: u2,
    pub info: Vec<u1>,
}