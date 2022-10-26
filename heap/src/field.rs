use classfile::FieldType;
use common::types::u2;

pub struct Field {
	pub access_flags: u2,
	pub name: Vec<u8>,
	pub descriptor: FieldType,
	// TODO
}
