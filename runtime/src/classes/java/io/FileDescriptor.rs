use crate::objects::instance::Instance;
use crate::objects::reference::Reference;

use classfile::FieldType;
use common::traits::PtrType;
use instructions::Operand;
use jni::sys::{jint, jlong, jobject};

#[cfg(unix)]
crate::classes::field_module! {
	@CLASS java_io_FileDescriptor;

	@FIELDSTART
	/// `java.io.FileDescriptor#fd` field offset
	///
	/// Expected field type: `jint`
	@FIELD fd: FieldType::Integer,
	/// `java.io.FileDescriptor#append` field offset
	///
	/// Expected field type: `jboolean`
	[sym: append_name] @FIELD append: FieldType::Boolean,
}

#[cfg(windows)]
crate::classes::field_module! {
	@CLASS java_io_FileDescriptor;

	@FIELDSTART
	/// `java.io.FileDescriptor#fd` field offset
	///
	/// Expected field type: `jint`
	@FIELD fd: FieldType::Integer,
	/// `java.io.FileDescriptor#handle` field offset
	///
	/// Expected field type: `jlong`
	@FIELD handle: FieldType::Long,
	/// `java.io.FileDescriptor#append` field offset
	///
	/// Expected field type: `jboolean`
	[sym: append_name] @FIELD append: FieldType::Boolean,
}

pub fn fd(this: Reference) -> jint {
	this.get_field_value0(fd_field_offset()).expect_int()
}

pub fn set_fd(this: Reference, fd: jint) {
	this.extract_class()
		.get_mut()
		.put_field_value0(fd_field_offset(), Operand::from(fd));
}

#[cfg(windows)]
pub fn handle(this: Reference) -> jlong {
	this.get_field_value0(handle_field_offset()).expect_long()
}

#[cfg(windows)]
pub fn set_handle(this: Reference, handle: jlong) {
	this.extract_class()
		.get_mut()
		.put_field_value0(handle_field_offset(), Operand::from(handle));
}
