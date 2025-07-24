use crate::globals;
use crate::objects::field::Field;
use crate::objects::instance::Instance;
use crate::objects::instance::array::PrimitiveArrayInstanceRef;
use crate::objects::instance::class::{ClassInstance, ClassInstanceRef};
use crate::objects::instance::mirror::MirrorInstanceRef;
use crate::objects::reference::Reference;
use crate::thread::exceptions::Throws;

use classfile::FieldType;
use instructions::Operand;
use jni::sys::jint;

/// Create a new `java.lang.reflect.Field` instance for the given field
pub fn new(_field: &Field) -> Throws<ClassInstanceRef> {
	let _reflect_field = ClassInstance::new(globals::classes::java_lang_reflect_Field());
	todo!()
}

pub fn clazz(instance: ClassInstanceRef) -> MirrorInstanceRef {
	instance
		.get_field_value0(clazz_field_index())
		.expect_reference()
		.extract_mirror()
}

pub fn set_clazz(instance: ClassInstanceRef, value: MirrorInstanceRef) {
	instance.put_field_value0(
		clazz_field_index(),
		Operand::Reference(Reference::mirror(value)),
	)
}

pub fn slot(instance: ClassInstanceRef) -> jint {
	instance.get_field_value0(slot_field_index()).expect_int()
}

pub fn set_slot(instance: ClassInstanceRef, value: jint) {
	instance.put_field_value0(slot_field_index(), Operand::Int(value))
}

pub fn name(instance: ClassInstanceRef) -> ClassInstanceRef {
	instance
		.get_field_value0(name_field_index())
		.expect_reference()
		.extract_class()
}

pub fn set_name(instance: ClassInstanceRef, value: ClassInstanceRef) {
	instance.put_field_value0(
		name_field_index(),
		Operand::Reference(Reference::class(value)),
	)
}

pub fn modifiers(instance: ClassInstanceRef) -> jint {
	instance
		.get_field_value0(modifiers_field_index())
		.expect_int()
}

pub fn set_modifiers(instance: ClassInstanceRef, value: jint) {
	instance.put_field_value0(modifiers_field_index(), Operand::Int(value))
}

pub fn signature(instance: ClassInstanceRef) -> ClassInstanceRef {
	instance
		.get_field_value0(signature_field_index())
		.expect_reference()
		.extract_class()
}

pub fn set_signature(instance: ClassInstanceRef, value: ClassInstanceRef) {
	instance.put_field_value0(
		signature_field_index(),
		Operand::Reference(Reference::class(value)),
	)
}

pub fn annotations(instance: ClassInstanceRef) -> PrimitiveArrayInstanceRef {
	instance
		.get_field_value0(annotations_field_index())
		.expect_reference()
		.extract_primitive_array()
}

pub fn set_annotations(instance: ClassInstanceRef, value: PrimitiveArrayInstanceRef) {
	instance.put_field_value0(
		annotations_field_index(),
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
	@FIELD annotations: FieldType::Array(val) if **val == FieldType::Byte,
}
