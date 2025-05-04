use classfile::FieldType;

crate::classes::field_module! {
	@CLASS java_lang_Class;

	@FIELDSTART
	/// `java.lang.Class#name` field offset
	///
	/// Expected type: `Reference` to `java.lang.String`
	@FIELD name: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
	/// `java.lang.Class#module` field offset
	///
	/// Expected type: `Reference` to `java.lang.Module`
	@FIELD module: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Module"),
	/// `java.lang.Class#classLoader` field offset
	///
	/// Expected type: `Reference` to `java.lang.ClassLoader`
	@FIELD classLoader: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/ClassLoader"),
	/// `java.lang.Class#classData` field offset
	///
	/// Expected type: Reference to `java.lang.Object`
	@FIELD classData: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Object"),
	/// `java.lang.Class#modifiers` field offset
	///
	/// Expected type: char
	@FIELD modifiers: FieldType::Character,
	/// `java.lang.Class#primitive` field offset
	///
	/// Expected type: boolean
	@FIELD primitive: FieldType::Boolean,
	/// `java.lang.Class#componentType` field offset
	///
	/// Expected type: `Reference` to `java.lang.Class`
	@FIELD componentType: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Class"),
}
