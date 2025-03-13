use classfile::FieldType;

#[cfg(unix)]
super::field_module! {
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
super::field_module! {
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
