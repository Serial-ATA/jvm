use crate::objects::class_instance::ClassInstance;
use crate::objects::instance::Instance;
use crate::objects::reference::Reference;
use classfile::FieldType;
use instructions::Operand;

/// `java.lang.Throwable#backtrace` field
pub fn backtrace(instance: &ClassInstance) -> Reference {
	instance
		.get_field_value0(backtrace_field_offset())
		.expect_reference()
}

pub fn set_backtrace(instance: &mut ClassInstance, value: Reference) {
	instance.put_field_value0(backtrace_field_offset(), Operand::Reference(value))
}

super::field_module! {
	@CLASS java_lang_Throwable;

	@FIELDSTART
	/// `java.lang.Throwable#stackTrace` field offset
	///
	/// Expected field type: `Reference` to `StackTraceElement[]`
	@FIELD stackTrace: FieldType::Array(ref val) if val.is_class(b"java/lang/StackTraceElement"),
	/// `java.lang.Throwable#backtrace` field offset
	///
	/// Expected field type: `Reference` to `java.lang.Object`
	@FIELD backtrace: FieldType::Object(_),
	/// `java.lang.Throwable#depth` field offset
	///
	/// Expected field type: `jint`
	@FIELD depth: FieldType::Integer,
}
