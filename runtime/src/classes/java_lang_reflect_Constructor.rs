use crate::globals;
use crate::native::java::lang::String::StringInterner;
use crate::objects::class_instance::ClassInstance;
use crate::objects::instance::Instance;
use crate::objects::method::Method;
use crate::objects::reference::{
	ClassInstanceRef, MirrorInstanceRef, ObjectArrayInstanceRef, Reference,
};
use crate::symbols::sym;
use crate::thread::exceptions::Throws;

use classfile::FieldType;
use common::traits::PtrType;
use instructions::Operand;
use jni::sys::jint;

/// Create a new `java.lang.reflect.Constructor` instance for the given method
pub fn new(method: &Method) -> Throws<ClassInstanceRef> {
	assert!(
		method.name == sym!(object_initializer_name) || method.name == sym!(class_initializer_name)
	);

	let constructor = ClassInstance::new(globals::classes::java_lang_reflect_Constructor());

	// The slot is the method's position in the vtable
	let slot = method
		.class()
		.vtable()
		.iter()
		.position(|m| m == method)
		.expect("a method must be present in a class vtable");

	let parameter_types = method.parameter_types_array()?;
	let exception_types = method.exception_types()?;

	set_clazz(
		constructor.get_mut(),
		Reference::mirror(method.class().mirror()),
	);
	set_slot(constructor.get_mut(), slot as jint);
	set_parameterTypes(
		constructor.get_mut(),
		Reference::object_array(parameter_types),
	);
	set_exceptionTypes(
		constructor.get_mut(),
		Reference::object_array(exception_types),
	);
	set_modifiers(constructor.get_mut(), method.access_flags.as_u2() as jint);
	if let Some(generic_signature) = method.generic_signature() {
		let signature = StringInterner::intern(generic_signature);
		set_signature(constructor.get_mut(), Reference::class(signature));
	}
	// // TODO
	// fields::java_lang_reflect_Constructor::set_annotations(
	// 	constructor.get_mut(),
	// 	Reference::null(),
	// );
	// // TODO
	// fields::java_lang_reflect_Constructor::set_parameterAnnotations(
	// 	constructor.get_mut(),
	// 	Reference::null(),
	// );

	Throws::Ok(constructor)
}

pub fn set_clazz(instance: &mut ClassInstance, value: Reference) {
	assert!(value.is_mirror());
	instance.put_field_value0(clazz_field_offset(), Operand::Reference(value))
}

pub fn clazz(instance: &ClassInstance) -> MirrorInstanceRef {
	instance
		.get_field_value0(clazz_field_offset())
		.expect_reference()
		.extract_mirror()
}

pub fn set_slot(instance: &mut ClassInstance, value: jint) {
	instance.put_field_value0(slot_field_offset(), Operand::Int(value))
}

pub fn slot(instance: &ClassInstance) -> jint {
	instance.get_field_value0(slot_field_offset()).expect_int()
}

pub fn set_parameterTypes(instance: &mut ClassInstance, value: Reference) {
	assert!(value.is_object_array());
	instance.put_field_value0(parameterTypes_field_offset(), Operand::Reference(value))
}

pub fn parameterTypes(instance: &ClassInstance) -> ObjectArrayInstanceRef {
	instance
		.get_field_value0(parameterTypes_field_offset())
		.expect_reference()
		.extract_object_array()
}

pub fn set_exceptionTypes(instance: &mut ClassInstance, value: Reference) {
	assert!(value.is_object_array());
	instance.put_field_value0(exceptionTypes_field_offset(), Operand::Reference(value))
}

pub fn set_modifiers(instance: &mut ClassInstance, value: jint) {
	instance.put_field_value0(modifiers_field_offset(), Operand::Int(value))
}

pub fn set_signature(instance: &mut ClassInstance, value: Reference) {
	assert!(value.is_instance_of(crate::globals::classes::java_lang_String()));
	instance.put_field_value0(signature_field_offset(), Operand::Reference(value))
}

pub fn set_annotations(instance: &mut ClassInstance, value: Reference) {
	assert!(value.is_object_array());
	instance.put_field_value0(annotations_field_offset(), Operand::Reference(value))
}

pub fn set_parameterAnnotations(instance: &mut ClassInstance, value: Reference) {
	assert!(value.is_object_array());
	instance.put_field_value0(
		parameterAnnotations_field_offset(),
		Operand::Reference(value),
	)
}

super::field_module! {
	@CLASS java_lang_reflect_Constructor;

	@FIELDSTART
	/// `java.lang.reflect.Constructor#clazz` field offset
	///
	/// Expected field type: `Reference` to `java.lang.Class`
	@FIELD clazz: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Class"),
	/// `java.lang.reflect.Constructor#slot` field offset
	///
	/// Expected field type: `jint`
	@FIELD slot: FieldType::Integer,
	/// `java.lang.reflect.Constructor#parameterTypes` field offset
	///
	/// Expected field type: `Reference` to `java.lang.Class[]`
	@FIELD parameterTypes: FieldType::Array(ref val) if val.is_class(b"java/lang/Class"),
	/// `java.lang.reflect.Constructor#exceptionTypes` field offset
	///
	/// Expected field type: `Reference` to `java.lang.Class[]`
	@FIELD exceptionTypes: FieldType::Array(ref val) if val.is_class(b"java/lang/Class"),
	/// `java.lang.reflect.Constructor#modifiers` field offset
	///
	/// Expected field type: `jint`
	@FIELD modifiers: FieldType::Integer,
	/// `java.lang.reflect.Constructor#signature` field offset
	///
	/// Expected field type: `Reference` to `java.lang.String`
	@FIELD signature: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
	/// `java.lang.reflect.Constructor#annotations` field offset
	///
	/// Expected field type: `Reference` to `byte[]`
	@FIELD annotations: FieldType::Array(ref val) if **val == FieldType::Byte,
	/// `java.lang.reflect.Constructor#parameterAnnotations` field offset
	///
	/// Expected field type: `Reference` to `byte[]`
	@FIELD parameterAnnotations: FieldType::Array(ref val) if **val == FieldType::Byte,
}
