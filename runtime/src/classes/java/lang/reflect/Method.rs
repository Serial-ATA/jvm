use crate::globals;
use crate::native::java::lang::String::StringInterner;
use crate::objects::instance::Instance;
use crate::objects::instance::array::{ObjectArrayInstanceRef, PrimitiveArrayInstanceRef};
use crate::objects::instance::class::{ClassInstance, ClassInstanceRef};
use crate::objects::instance::mirror::MirrorInstanceRef;
use crate::objects::method::Method;
use crate::objects::reference::Reference;
use crate::thread::exceptions::Throws;

use classfile::FieldType;
use instructions::Operand;
use jni::sys::jint;

/// Create a new `java.lang.reflect.Method` instance for the given method
pub fn new(method: &Method) -> Throws<ClassInstanceRef> {
	let reflect_method = ClassInstance::new(globals::classes::java_lang_reflect_Method());

	// The slot is the method's position in the vtable
	let slot = method
		.class()
		.vtable()
		.iter()
		.position(|m| m == method)
		.expect("a method must be present in a class vtable");

	set_clazz(reflect_method, method.class().mirror());
	set_slot(reflect_method, slot as jint);
	set_name(reflect_method, StringInterner::intern(method.name));
	set_returnType(reflect_method, method.return_type()?);
	set_parameterTypes(reflect_method, method.parameter_types_array()?);
	set_exceptionTypes(reflect_method, method.exception_types()?);
	set_modifiers(reflect_method, method.access_flags.as_u2() as jint);
	if let Some(generic_signature) = method.generic_signature() {
		set_signature(reflect_method, StringInterner::intern(generic_signature));
	}

	// // TODO
	// set_annotations(
	// 	constructor.get_mut(),
	// 	Reference::null(),
	// );
	// // TODO
	// set_parameterAnnotations(
	// 	constructor.get_mut(),
	// 	Reference::null(),
	// );
	// // TODO
	// set_annotationDefault(
	// 	constructor.get_mut(),
	// 	Reference::null(),
	// );

	Throws::Ok(reflect_method)
}

pub fn vmtarget(instance: ClassInstanceRef) -> Option<&'static Method> {
	let mirror = clazz(instance);
	let slot = slot(instance);
	mirror.target_class().vtable().iter().nth(slot as usize)
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

pub fn returnType(instance: ClassInstanceRef) -> MirrorInstanceRef {
	instance
		.get_field_value0(returnType_field_index())
		.expect_reference()
		.extract_mirror()
}

pub fn set_returnType(instance: ClassInstanceRef, value: MirrorInstanceRef) {
	instance.put_field_value0(
		returnType_field_index(),
		Operand::Reference(Reference::mirror(value)),
	)
}

pub fn parameterTypes(instance: ClassInstanceRef) -> ObjectArrayInstanceRef {
	instance
		.get_field_value0(parameterTypes_field_index())
		.expect_reference()
		.extract_object_array()
}

pub fn set_parameterTypes(instance: ClassInstanceRef, value: ObjectArrayInstanceRef) {
	instance.put_field_value0(
		parameterTypes_field_index(),
		Operand::Reference(Reference::object_array(value)),
	)
}

pub fn exceptionTypes(instance: ClassInstanceRef) -> ObjectArrayInstanceRef {
	instance
		.get_field_value0(exceptionTypes_field_index())
		.expect_reference()
		.extract_object_array()
}

pub fn set_exceptionTypes(instance: ClassInstanceRef, value: ObjectArrayInstanceRef) {
	instance.put_field_value0(
		exceptionTypes_field_index(),
		Operand::Reference(Reference::object_array(value)),
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

pub fn parameterAnnotations(instance: ClassInstanceRef) -> PrimitiveArrayInstanceRef {
	instance
		.get_field_value0(parameterAnnotations_field_index())
		.expect_reference()
		.extract_primitive_array()
}

pub fn set_parameterAnnotations(instance: ClassInstanceRef, value: PrimitiveArrayInstanceRef) {
	instance.put_field_value0(
		parameterAnnotations_field_index(),
		Operand::Reference(Reference::array(value)),
	)
}

pub fn annotationDefault(instance: ClassInstanceRef) -> PrimitiveArrayInstanceRef {
	instance
		.get_field_value0(annotationDefault_field_index())
		.expect_reference()
		.extract_primitive_array()
}

pub fn set_annotationDefault(instance: ClassInstanceRef, value: PrimitiveArrayInstanceRef) {
	instance.put_field_value0(
		annotationDefault_field_index(),
		Operand::Reference(Reference::array(value)),
	)
}

crate::classes::field_module! {
	@CLASS java_lang_reflect_Method;

	@FIELDSTART
	/// `java.lang.reflect.Method#clazz` field offset
	///
	/// Expected field type: `Reference` to `java.lang.Class`
	@FIELD clazz: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Class"),
	/// `java.lang.reflect.Method#slot` field offset
	///
	/// Expected field type: `jint`
	@FIELD slot: FieldType::Integer,
	/// `java.lang.reflect.Method#name` field offset
	///
	/// Expected field type: `Reference` to `java.lang.String`
	@FIELD name: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
	/// `java.lang.reflect.Method#returnType` field offset
	///
	/// Expected field type: `Reference` to `java.lang.Class]`
	@FIELD returnType: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Class"),
	/// `java.lang.reflect.Method#parameterTypes` field offset
	///
	/// Expected field type: `Reference` to `java.lang.Class[]`
	@FIELD parameterTypes: FieldType::Array(val) if val.is_class(b"java/lang/Class"),
	/// `java.lang.reflect.Method#exceptionTypes` field offset
	///
	/// Expected field type: `Reference` to `java.lang.Class[]`
	@FIELD exceptionTypes: FieldType::Array(val) if val.is_class(b"java/lang/Class"),
	/// `java.lang.reflect.Method#modifiers` field offset
	///
	/// Expected field type: `jint`
	@FIELD modifiers: FieldType::Integer,
	/// `java.lang.reflect.Method#signature` field offset
	///
	/// Expected field type: `Reference` to `java.lang.String`
	@FIELD signature: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
	/// `java.lang.reflect.Method#annotations` field offset
	///
	/// Expected field type: `Reference` to `byte[]`
	@FIELD annotations: FieldType::Array(val) if **val == FieldType::Byte,
	/// `java.lang.reflect.Method#parameterAnnotations` field offset
	///
	/// Expected field type: `Reference` to `byte[]`
	@FIELD parameterAnnotations: FieldType::Array(val) if **val == FieldType::Byte,
	/// `java.lang.reflect.Method#annotationDefault` field offset
	///
	/// Expected field type: `Reference` to `byte[]`
	@FIELD annotationDefault: FieldType::Array(val) if **val == FieldType::Byte,
}
