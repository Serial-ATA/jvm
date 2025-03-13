use crate::objects::class_instance::ClassInstance;
use crate::objects::instance::Instance;
use crate::objects::reference::Reference;
use classfile::FieldType;
use instructions::Operand;
use jni::sys::{jboolean, jbyte, jint};

/// `java.lang.String#value` field
pub fn value(instance: &ClassInstance) -> Reference {
	instance
		.get_field_value0(value_field_offset())
		.expect_reference()
}

pub fn set_value(instance: &mut ClassInstance, value: Reference) {
	instance.put_field_value0(value_field_offset(), Operand::Reference(value))
}

/// `java.lang.String#coder` field
pub fn coder(instance: &ClassInstance) -> jbyte {
	instance.get_field_value0(coder_field_offset()).expect_int() as jbyte
}

pub fn set_coder(instance: &mut ClassInstance, value: jbyte) {
	instance.put_field_value0(coder_field_offset(), Operand::Int(value as jint))
}

/// `java.lang.String#hash` field
pub fn hash(instance: &ClassInstance) -> jint {
	instance.get_field_value0(hash_field_offset()).expect_int()
}

pub fn set_hash(instance: &mut ClassInstance, value: jint) {
	instance.put_field_value0(hash_field_offset(), Operand::Int(value))
}

/// `java.lang.String#hashIsZero` field
pub fn hashIsZero(instance: &ClassInstance) -> jboolean {
	instance
		.get_field_value0(hashIsZero_field_offset())
		.expect_int()
		!= 0
}

pub fn set_hashIsZero(instance: &mut ClassInstance, value: jboolean) {
	instance.put_field_value0(hashIsZero_field_offset(), Operand::Int(value as jint))
}

super::field_module! {
	@CLASS java_lang_String;

	@FIELDSTART
	/// `java.lang.String#value` field offset
	///
	/// Expected type: `jByteArray`
	@FIELD value: FieldType::Array(ref val) if **val == FieldType::Byte,
	/// `java.lang.String#coder` field offset
	///
	/// Expected type: `jbyte`
	@FIELD coder: FieldType::Byte,
	/// `java.lang.String#hash` field offset
	///
	/// Expected type: `jint`
	@FIELD hash: FieldType::Integer,
	/// `java.lang.String#hashIsZero` field offset
	///
	/// Expected type: `jboolean`
	@FIELD hashIsZero: FieldType::Boolean,
}
