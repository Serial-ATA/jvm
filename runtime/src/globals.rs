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

/// Various offsets for fields of frequently accessed classes
pub mod field_offsets {
	pub static mut STRING_VALUE_FIELD_OFFSET: usize = 0;
	pub static mut STRING_CODER_FIELD_OFFSET: usize = 0;

	pub static mut CLASS_NAME_FIELD_OFFSET: usize = 0;

	pub fn string_value_field_offset() -> usize {
		unsafe { STRING_VALUE_FIELD_OFFSET }
	}
	pub fn string_coder_field_offset() -> usize {
		unsafe { STRING_CODER_FIELD_OFFSET }
	}

	pub unsafe fn set_string_field_offsets(value: usize, coder: usize) {
		STRING_VALUE_FIELD_OFFSET = value;
		STRING_CODER_FIELD_OFFSET = coder;
	}

	pub fn class_name_field_offset() -> usize {
		unsafe { CLASS_NAME_FIELD_OFFSET }
	}
}

/// Globals related to the module system
pub mod modules {
	pub static mut MODULE_SYSTEM_INITIALIZED: bool = false;

	pub fn module_system_initialized() -> bool {
		unsafe { MODULE_SYSTEM_INITIALIZED }
	}

	pub unsafe fn set_module_system_initialized() {
		MODULE_SYSTEM_INITIALIZED = true;
	}
}
