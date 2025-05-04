use crate::objects::instance::Instance;
use crate::objects::reference::Reference;
use classfile::FieldType;
use jni::sys::jint;

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

pub fn fd(this: &Reference) -> jint {
	let fd_field_offset = fd_field_offset();
	this.get_field_value0(fd_field_offset).expect_int()
}
