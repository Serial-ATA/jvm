use crate::objects::class_instance::ClassInstance;
use crate::objects::instance::Instance;
use crate::objects::reference::Reference;
use classfile::FieldType;
use instructions::Operand;
use jni::sys::jint;

pub fn set_declaringClassObject(instance: &mut ClassInstance, value: Reference) {
	assert!(value.is_instance_of(crate::globals::classes::java_lang_Class()));
	instance.put_field_value0(
		declaringClassObject_field_offset(),
		Operand::Reference(value),
	)
}

pub fn set_classLoaderName(instance: &mut ClassInstance, value: Reference) {
	assert!(value.is_instance_of(crate::globals::classes::java_lang_String()));
	instance.put_field_value0(classLoaderName_field_offset(), Operand::Reference(value))
}

pub fn set_moduleName(instance: &mut ClassInstance, value: Reference) {
	assert!(value.is_instance_of(crate::globals::classes::java_lang_String()));
	instance.put_field_value0(moduleName_field_offset(), Operand::Reference(value))
}

pub fn set_moduleVersion(instance: &mut ClassInstance, value: Reference) {
	assert!(value.is_instance_of(crate::globals::classes::java_lang_String()));
	instance.put_field_value0(moduleVersion_field_offset(), Operand::Reference(value))
}

pub fn set_declaringClass(instance: &mut ClassInstance, value: Reference) {
	assert!(value.is_instance_of(crate::globals::classes::java_lang_String()));
	instance.put_field_value0(declaringClass_field_offset(), Operand::Reference(value))
}

pub fn set_methodName(instance: &mut ClassInstance, value: Reference) {
	assert!(value.is_instance_of(crate::globals::classes::java_lang_String()));
	instance.put_field_value0(methodName_field_offset(), Operand::Reference(value))
}

pub fn set_fileName(instance: &mut ClassInstance, value: Reference) {
	assert!(value.is_instance_of(crate::globals::classes::java_lang_String()));
	instance.put_field_value0(fileName_field_offset(), Operand::Reference(value))
}

pub fn set_lineNumber(instance: &mut ClassInstance, value: jint) {
	instance.put_field_value0(lineNumber_field_offset(), Operand::Int(value))
}

super::field_module! {
	@CLASS java_lang_StackTraceElement;

	@FIELDSTART
	/// `java.lang.StackTraceElement#declaringClassObject` field offset
	///
	/// Expected field type: `Reference` to `java.lang.Class`
	@FIELD declaringClassObject: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Class"),
	/// `java.lang.StackTraceElement#classLoaderName` field offset
	///
	/// Expected field type: `Reference` to `java.lang.String`
	@FIELD classLoaderName: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
	/// `java.lang.StackTraceElement#moduleName` field offset
	///
	/// Expected field type: `Reference` to `java.lang.String`
	@FIELD moduleName: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
	/// `java.lang.StackTraceElement#moduleVersion` field offset
	///
	/// Expected field type: `Reference` to `java.lang.String`
	@FIELD moduleVersion: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
	/// `java.lang.StackTraceElement#declaringClass` field offset
	///
	/// Expected field type: `Reference` to `java.lang.String`
	@FIELD declaringClass: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
	/// `java.lang.StackTraceElement#methodName` field offset
	///
	/// Expected field type: `Reference` to `java.lang.String`
	@FIELD methodName: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
	/// `java.lang.StackTraceElement#fileName` field offset
	///
	/// Expected field type: `Reference` to `java.lang.String`
	@FIELD fileName: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
	/// `java.lang.StackTraceElement#lineNumber` field offset
	///
	/// Expected field type: `jint`
	@FIELD lineNumber: FieldType::Integer,
}
