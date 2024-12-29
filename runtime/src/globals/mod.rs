pub mod classes;
pub mod fields;
pub mod modules;
pub mod threads;

use classfile::FieldType;
use symbols::{sym, Symbol};

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

pub const PRIMITIVES: &[Symbol] = &[
	sym!(bool),
	sym!(byte),
	sym!(char),
	sym!(double),
	sym!(float),
	sym!(int),
	sym!(long),
	sym!(short),
];
