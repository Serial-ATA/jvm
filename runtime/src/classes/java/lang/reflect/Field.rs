use crate::globals;
use crate::objects::class_instance::ClassInstance;
use crate::objects::field::Field;
use crate::objects::instance::Instance;
use crate::objects::reference::{
	ClassInstanceRef, MirrorInstanceRef, PrimitiveArrayInstanceRef, Reference,
};
use crate::thread::exceptions::Throws;

use classfile::FieldType;
use instructions::Operand;
use jni::sys::jint;

/// Create a new `java.lang.reflect.Field` instance for the given field
pub fn new(_field: &Field) -> Throws<ClassInstanceRef> {
	let _reflect_field = ClassInstance::new(globals::classes::java_lang_reflect_Field());
	todo!()
}

pub fn clazz(instance: &ClassInstance) -> MirrorInstanceRef {
	instance
		.get_field_value0(clazz_field_offset())
		.expect_reference()
		.extract_mirror()
}

pub fn set_clazz(instance: &mut ClassInstance, value: MirrorInstanceRef) {
	instance.put_field_value0(
		clazz_field_offset(),
		Operand::Reference(Reference::mirror(value)),
	)
}

pub fn slot(instance: &ClassInstance) -> jint {
	instance.get_field_value0(slot_field_offset()).expect_int()
}

pub fn set_slot(instance: &mut ClassInstance, value: jint) {
	instance.put_field_value0(slot_field_offset(), Operand::Int(value))
}

pub fn name(instance: &ClassInstance) -> ClassInstanceRef {
	instance
		.get_field_value0(name_field_offset())
		.expect_reference()
		.extract_class()
}

pub fn set_name(instance: &mut ClassInstance, value: ClassInstanceRef) {
	instance.put_field_value0(
		name_field_offset(),
		Operand::Reference(Reference::class(value)),
	)
}

pub fn modifiers(instance: &ClassInstance) -> jint {
	instance
		.get_field_value0(modifiers_field_offset())
		.expect_int()
}

pub fn set_modifiers(instance: &mut ClassInstance, value: jint) {
	instance.put_field_value0(modifiers_field_offset(), Operand::Int(value))
}

pub fn signature(instance: &ClassInstance) -> ClassInstanceRef {
	instance
		.get_field_value0(signature_field_offset())
		.expect_reference()
		.extract_class()
}

pub fn set_signature(instance: &mut ClassInstance, value: ClassInstanceRef) {
	instance.put_field_value0(
		signature_field_offset(),
		Operand::Reference(Reference::class(value)),
	)
}

pub fn annotations(instance: &ClassInstance) -> PrimitiveArrayInstanceRef {
	instance
		.get_field_value0(annotations_field_offset())
		.expect_reference()
		.extract_primitive_array()
}

pub fn set_annotations(instance: &mut ClassInstance, value: PrimitiveArrayInstanceRef) {
	instance.put_field_value0(
		annotations_field_offset(),
		Operand::Reference(Reference::array(value)),
	)
}

crate::classes::field_module! {
	@CLASS java_lang_reflect_Field;

	@FIELDSTART
	/// `java.lang.reflect.Field#clazz` field offset
	///
	/// Expected field type: `Reference` to `java.lang.Class`
	@FIELD clazz: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Class"),
	/// `java.lang.reflect.Field#slot` field offset
	///
	/// Expected field type: `jint`
	@FIELD slot: FieldType::Integer,
	/// `java.lang.reflect.Field#name` field offset
	///
	/// Expected field type: `Reference` to `java.lang.String`
	@FIELD name: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
	/// `java.lang.reflect.Field#type` field offset
	///
	/// Expected field type: `Reference` to `java.lang.Class]`
	[sym: r#type] @FIELD type_: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Class"),
	/// `java.lang.reflect.Field#modifiers` field offset
	///
	/// Expected field type: `jint`
	@FIELD modifiers: FieldType::Integer,
	/// `java.lang.reflect.Field#modifiers` field offset
	///
	/// Expected field type: `jboolean`
	@FIELD trustedFinal: FieldType::Boolean,
	/// `java.lang.reflect.Field#signature` field offset
	///
	/// Expected field type: `Reference` to `java.lang.String`
	@FIELD signature: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
	/// `java.lang.reflect.Field#annotations` field offset
	///
	/// Expected field type: `Reference` to `byte[]`
	@FIELD annotations: FieldType::Array(ref val) if **val == FieldType::Byte,
}
