use crate::objects::class_instance::ClassInstance;
use crate::objects::instance::Instance;
use crate::objects::reference::ClassInstanceRef;
use classfile::FieldType;

/// `java.lang.invoke.MethodHandle#form` field
pub fn form(instance: &ClassInstance) -> ClassInstanceRef {
	assert!(instance
		.class()
		.is_subclass_of(crate::globals::classes::java_lang_invoke_MethodHandle()));
	instance
		.get_field_value0(form_field_offset())
		.expect_reference()
		.extract_class()
}

super::field_module! {
	@CLASS java_lang_invoke_MethodHandle;

	@FIELDSTART
	/// `java.lang.invoke.MethodHandle#form` field offset
	///
	/// Expected field type: `Reference` to `java.lang.invoke.LambdaForm`
	@FIELD form: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/invoke/LambdaForm"),
}
