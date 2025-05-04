use crate::objects::class_instance::ClassInstance;
use crate::objects::instance::Instance;
use crate::objects::reference::ClassInstanceRef;
use classfile::FieldType;

/// `java.lang.invoke.LambdaForm#vmentry` field
pub fn vmentry(instance: &ClassInstance) -> ClassInstanceRef {
	instance
		.get_field_value0(vmentry_field_offset())
		.expect_reference()
		.extract_class()
}

crate::classes::field_module! {
	@CLASS java_lang_invoke_LambdaForm;

	@FIELDSTART
	/// `java.lang.invoke.LambdaForm#form` field offset
	///
	/// Expected field type: `Reference` to `java.lang.invoke.MemberName`
	@FIELD vmentry: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/invoke/MemberName"),
}
