use crate::JavaReadExt;
use crate::attribute;

use classfile::FieldInfo;

use std::io::Read;

pub fn read_field_info<R>(reader: &mut R) -> FieldInfo
    where
        R: Read,
{
    let access_flags = reader.read_u2();
    let name_index = reader.read_u2();
    let descriptor_index = reader.read_u2();

    let attributes_count = reader.read_u2();
    let mut attributes = Vec::with_capacity(attributes_count as usize);

    for _ in 0..attributes_count {
        attributes.push(attribute::read_attribute(reader))
    }

    FieldInfo {
        access_flags,
        name_index,
        descriptor_index,
        attributes,
    }
}