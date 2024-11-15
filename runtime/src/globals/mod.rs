pub mod classes;
pub mod field_offsets;
pub mod modules;
pub mod threads;

use classfile::FieldType;

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
