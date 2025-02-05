pub mod classes;
pub mod fields;
pub mod mirrors;
pub mod modules;
pub mod threads;

use crate::symbols::{sym, Symbol};

use classfile::FieldType;

/// Primitive type names, as they would appear in Java source code
///
/// <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.3>
pub const PRIMITIVE_TYPE_NAMES_TO_FIELD_TYPES: &[(&str, FieldType)] = &[
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

/// A `BaseType`, as in a primitive type as it would appear in a descriptor string
///
/// <https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-BaseType>
pub const BASE_TYPES_TO_FIELD_TYPES: &[(&str, FieldType)] = &[
	("Z", FieldType::Boolean),
	("C", FieldType::Char),
	("F", FieldType::Float),
	("D", FieldType::Double),
	("B", FieldType::Byte),
	("S", FieldType::Short),
	("I", FieldType::Int),
	("J", FieldType::Long),
	("V", FieldType::Void),
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
