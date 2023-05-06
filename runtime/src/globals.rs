use classfile::FieldType;

pub static mut STRING_VALUE_FIELD_OFFSET: usize = 0;
pub static mut STRING_CODER_FIELD_OFFSET: usize = 0;

pub static mut CLASS_NAME_FIELD_OFFSET: usize = 0;

pub const TYPES: &[(&str, FieldType)] = &[
	("boolean", FieldType::Boolean),
	("char", FieldType::Char),
	("float", FieldType::Float),
	("double", FieldType::Double),
	("byte", FieldType::Byte),
	("short", FieldType::Short),
	("int", FieldType::Int),
	("long", FieldType::Long),
	("void", FieldType::Void),
];

pub fn string_value_field_offset() -> usize {
	unsafe { STRING_VALUE_FIELD_OFFSET }
}
pub fn string_coder_field_offset() -> usize {
	unsafe { STRING_CODER_FIELD_OFFSET }
}

pub fn class_name_field_offset() -> usize {
	unsafe { CLASS_NAME_FIELD_OFFSET }
}
