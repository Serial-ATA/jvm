//! Various offsets for fields of frequently accessed classes

static mut STRING_VALUE_FIELD_OFFSET: usize = 0;
static mut STRING_CODER_FIELD_OFFSET: usize = 0;

/// `java.lang.String#value` field offset
///
/// This will not change for the lifetime of the program.
///
/// Expected type: `jByteArray`
pub fn string_value_field_offset() -> usize {
	unsafe { STRING_VALUE_FIELD_OFFSET }
}
/// `java.lang.String#coder` field offset
///
/// This will not change for the lifetime of the program.
///
/// Expected type: `jint`
pub fn string_coder_field_offset() -> usize {
	unsafe { STRING_CODER_FIELD_OFFSET }
}

pub unsafe fn set_string_field_offsets(value: usize, coder: usize) {
	STRING_VALUE_FIELD_OFFSET = value;
	STRING_CODER_FIELD_OFFSET = coder;
}

static mut CLASS_NAME_FIELD_OFFSET: usize = 0;

pub fn class_name_field_offset() -> usize {
	unsafe { CLASS_NAME_FIELD_OFFSET }
}

static mut THREAD_HOLDER_FIELD_OFFSET: usize = 0;

/// `java.lang.Thread#holder` field offset
///
/// This will not change for the lifetime of the program.
///
/// Expected type: `java.lang.Thread$FieldHolder` reference
pub fn thread_holder_field_offset() -> usize {
	unsafe { THREAD_HOLDER_FIELD_OFFSET }
}

pub unsafe fn set_thread_holder_field_offset(value: usize) {
	THREAD_HOLDER_FIELD_OFFSET = value;
}

static mut FIELDHOLDER_PRIORITY_FIELD_OFFSET: usize = 0;
static mut FIELDHOLDER_DAEMON_FIELD_OFFSET: usize = 0;
static mut FIELDHOLDER_THREAD_STATUS_FIELD_OFFSET: usize = 0;

/// `java.lang.Thread$FieldHolder#priority` field offset
///
/// **THIS IS A STATIC FIELD**
///
/// This will not change for the lifetime of the program.
///
/// Expected field type: `jint`
pub fn field_holder_priority_field_offset() -> usize {
	unsafe { FIELDHOLDER_PRIORITY_FIELD_OFFSET }
}

pub unsafe fn set_field_holder_priority_field_offset(value: usize) {
	FIELDHOLDER_PRIORITY_FIELD_OFFSET = value;
}

/// `java.lang.Thread$FieldHolder#daemon` field offset
///
/// **THIS IS A STATIC FIELD**
///
/// This will not change for the lifetime of the program.
///
/// Expected field type: `jboolean`
pub fn field_holder_daemon_field_offset() -> usize {
	unsafe { FIELDHOLDER_DAEMON_FIELD_OFFSET }
}

pub unsafe fn set_field_holder_daemon_field_offset(value: usize) {
	FIELDHOLDER_DAEMON_FIELD_OFFSET = value;
}

/// `java.lang.Thread$FieldHolder#threadStatus` field offset
///
/// **THIS IS A STATIC FIELD**
///
/// This will not change for the lifetime of the program.
///
/// Expected field type: `jint`
pub fn field_holder_thread_status_field_offset() -> usize {
	unsafe { FIELDHOLDER_THREAD_STATUS_FIELD_OFFSET }
}

pub unsafe fn set_field_holder_thread_status_field_offset(value: usize) {
	FIELDHOLDER_THREAD_STATUS_FIELD_OFFSET = value;
}
