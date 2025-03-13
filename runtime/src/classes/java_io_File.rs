use classfile::FieldType;

super::field_module! {
	@CLASS java_io_File;

	@FIELDSTART
	/// `java.io.File#path` field offset
	///
	/// Expected field type: `Reference` to `java.lang.String`
	@FIELD path: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
}
