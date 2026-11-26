use crate::globals;
use crate::native::java::lang::String::StringInterner;
use crate::objects::field::Field;
use crate::objects::instance::Instance;
use crate::objects::instance::array::PrimitiveArrayInstanceRef;
use crate::objects::instance::class::{ClassInstance, ClassInstanceRef};
use crate::objects::instance::mirror::MirrorInstanceRef;
use crate::objects::method::field_type_mirror;
use crate::objects::reference::Reference;
use crate::thread::exceptions::Throws;

use classfile::FieldType;
use instructions::Operand;
use jni::sys::{jboolean, jint};

// From java/lang/reflect/Field.java
const TRUSTED_FINAL: jint = 0x0010;
const NULL_RESTRICTED: jint = 0x0020;

/// Create a new `java.lang.reflect.Field` instance for the given field
pub fn new(field: &Field) -> Throws<ClassInstanceRef> {
	let reflect_field = ClassInstance::new(globals::classes::java_lang_reflect_Field());

	set_clazz(reflect_field, field.class.mirror());
	set_slot(reflect_field, field.index() as jint);
	set_type(
		reflect_field,
		field_type_mirror(field.class, &field.descriptor)?,
	);
	set_name(reflect_field, StringInterner::intern(field.name));
	set_modifiers(reflect_field, jint::from(field.access_flags.as_u2()));

	let mut flags = 0;
	if field.is_trusted_final() {
		flags |= TRUSTED_FINAL;
	}

	set_flags(reflect_field, flags);
	if let Some(generic_signature) = field.generic_signature() {
		set_signature(reflect_field, StringInterner::intern(generic_signature));
	}

	if let Some(annotations) = field.annotations_array() {
		set_annotations(reflect_field, annotations);
	}

	Throws::Ok(reflect_field)
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

pub fn set_type(instance: ClassInstanceRef, value: MirrorInstanceRef) {
	instance.put_field_value0(
		type__field_index(),
		Operand::Reference(Reference::mirror(value)),
	)
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

pub fn set_flags(instance: ClassInstanceRef, value: jint) {
	instance.put_field_value0(flags_field_index(), Operand::Int(value))
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
	/// `java.lang.reflect.Field#flags` field offset
	///
	/// Expected field type: `jint`
	@FIELD flags: FieldType::Integer,
	/// `java.lang.reflect.Field#signature` field offset
	///
	/// Expected field type: `Reference` to `java.lang.String`
	@FIELD signature: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
	/// `java.lang.reflect.Field#annotations` field offset
	///
	/// Expected field type: `Reference` to `byte[]`
	@FIELD annotations: FieldType::Array(val) if **val == FieldType::Byte,
}
