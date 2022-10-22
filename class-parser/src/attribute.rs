use crate::JavaReadExt;

use std::io::Read;

use classfile::Attribute;

pub fn read_attribute<R>(reader: &mut R) -> Attribute
    where
        R: Read,
{
    let attribute_name_index = reader.read_u2();

    let attribute_length = reader.read_u4();
    let mut info = vec![0u8; attribute_length as usize];
    reader.read_exact(&mut info).unwrap();

    Attribute {
        attribute_name_index,
        info
    }
}