use classfile::FieldType;

super::field_module! {
	@CLASS java_io_FileOutputStream;

	@FIELDSTART
	/// `java.io.FileOutputStream#fd` field offset
	///
	/// Expected field type: `Reference` to `java.io.FileDescriptor`
	@FIELD fd: ty @ FieldType::Object(_) if ty.is_class(b"java/io/FileDescriptor"),
}
