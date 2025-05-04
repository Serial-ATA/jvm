use classfile::FieldType;

crate::classes::field_module! {
	@CLASS java_lang_ref_Reference;

	@FIELDSTART
	/// `java.lang.ref.Reference#referent` field offset
	///
	/// Expected type: `Reference`
	@FIELD referent: FieldType::Object(_),
}
