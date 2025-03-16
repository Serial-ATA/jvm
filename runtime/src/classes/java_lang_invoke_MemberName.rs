use crate::objects::class_instance::ClassInstance;
use crate::objects::instance::Instance;
use crate::objects::method::Method;
use crate::objects::reference::{MirrorInstanceRef, Reference};
use crate::thread::exceptions::{throw, Throws};

use classfile::FieldType;
use common::traits::PtrType;
use instructions::Operand;
use jni::sys::{jint, jlong};

/// `java.lang.invoke.MemberName#clazz` field
pub fn clazz(instance: &ClassInstance) -> Throws<MirrorInstanceRef> {
	let clazz = instance
		.get_field_value0(clazz_field_offset())
		.expect_reference();

	if clazz.is_null() {
		throw!(@DEFER InternalError, "mname not resolved");
	}

	Throws::Ok(clazz.extract_mirror())
}

pub fn set_clazz(instance: &mut ClassInstance, value: Reference) {
	instance.put_field_value0(clazz_field_offset(), Operand::Reference(value))
}

/// `java.lang.invoke.MemberName#name` field
pub fn name(instance: &ClassInstance) -> Reference {
	instance
		.get_field_value0(name_field_offset())
		.expect_reference()
}

pub fn set_name(instance: &mut ClassInstance, value: Reference) {
	instance.put_field_value0(name_field_offset(), Operand::Reference(value))
}

/// `java.lang.invoke.MemberName#type` field
pub fn type_(instance: &ClassInstance) -> Reference {
	instance
		.get_field_value0(type_field_offset())
		.expect_reference()
}

pub fn set_type(instance: &mut ClassInstance, value: Reference) {
	instance.put_field_value0(type_field_offset(), Operand::Reference(value));
}

/// `java.lang.invoke.MemberName#flags` field
pub fn flags(instance: &ClassInstance) -> jint {
	instance.get_field_value0(flags_field_offset()).expect_int()
}

pub fn set_flags(instance: &mut ClassInstance, value: jint) {
	instance.put_field_value0(flags_field_offset(), Operand::Int(value));
}

/// `java.lang.invoke.MemberName#method` field
pub fn method(instance: &ClassInstance) -> Reference {
	instance
		.get_field_value0(method_field_offset())
		.expect_reference()
}

pub fn set_method(instance: &mut ClassInstance, value: Reference) {
	instance.put_field_value0(method_field_offset(), Operand::Reference(value));
}

/// Injected `java.lang.invoke.MemberName#vmindex` field
pub fn vmindex(instance: &ClassInstance) -> jlong {
	instance
		.get_field_value0(vmindex_field_offset())
		.expect_long()
}

pub fn set_vmindex(instance: &mut ClassInstance, value: jlong) {
	instance.put_field_value0(vmindex_field_offset(), Operand::Long(value));
}

pub fn target_method(instance: &ClassInstance) -> Throws<&'static Method> {
	let vmindex = vmindex(instance);

	let defining_class_mirror = clazz(instance)?;
	let defining_class = defining_class_mirror.get().target_class();

	Throws::Ok(&defining_class.vtable()[vmindex as usize])
}

super::field_module! {
	@CLASS java_lang_invoke_MemberName;

	@FIELDSTART
	/// `java.lang.invoke.MemberName#clazz` field offset
	///
	/// Expected field type: `Reference` to `java.lang.Class`
	@FIELD clazz: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Class"),
	/// `java.lang.invoke.MemberName#name` field offset
	///
	/// Expected field type: `Reference` to `java.lang.String`
	@FIELD name: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
	/// `java.lang.invoke.MemberName#type` field offset
	///
	/// Expected field type: `Reference` to `java.lang.Object`
	@FIELD r#type: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Object"),
	/// `java.lang.invoke.MemberName#flags` field offset
	///
	/// Expected field type: jint
	@FIELD flags: FieldType::Integer,
	/// `java.lang.invoke.MemberName#method` field offset
	///
	/// Expected field type: `Reference` to `java.lang.invoke.ResolvedMethodName`
	@FIELD method: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/invoke/ResolvedMethodName"),
	/// [`Method`] offset in target class [`VTable`]
	///
	/// Expected type: `jlong`
	/// [`Method`]: crate::objects::method::Method
	/// [`VTable`]: crate::objects::vtable::VTable
	@INJECTED vmindex: FieldType::Long => jni::sys::jlong,
}
