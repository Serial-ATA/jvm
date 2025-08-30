use crate::objects::instance::Instance;
use crate::objects::instance::class::ClassInstanceRef;
use crate::objects::instance::mirror::MirrorInstanceRef;
use crate::objects::method::Method;
use crate::objects::reference::Reference;
use crate::thread::exceptions::{Throws, throw};

use classfile::FieldType;
use instructions::Operand;
use jni::sys::{jint, jlong};

/// `java.lang.invoke.MemberName#clazz` field
pub fn clazz(instance: ClassInstanceRef) -> Throws<MirrorInstanceRef> {
	let clazz = instance
		.get_field_value0(clazz_field_index())
		.expect_reference();

	if clazz.is_null() {
		throw!(@DEFER InternalError, "mname not resolved");
	}

	Throws::Ok(clazz.extract_mirror())
}

pub fn set_clazz(instance: ClassInstanceRef, value: Reference) {
	instance.put_field_value0(clazz_field_index(), Operand::Reference(value))
}

/// `java.lang.invoke.MemberName#name` field
pub fn name(instance: ClassInstanceRef) -> Reference {
	instance
		.get_field_value0(name_field_index())
		.expect_reference()
}

pub fn set_name(instance: ClassInstanceRef, value: Reference) {
	instance.put_field_value0(name_field_index(), Operand::Reference(value))
}

/// `java.lang.invoke.MemberName#type` field
pub fn type_(instance: ClassInstanceRef) -> Reference {
	instance
		.get_field_value0(type_field_index())
		.expect_reference()
}

pub fn set_type(instance: ClassInstanceRef, value: Reference) {
	instance.put_field_value0(type_field_index(), Operand::Reference(value));
}

/// `java.lang.invoke.MemberName#flags` field
pub fn flags(instance: ClassInstanceRef) -> jint {
	instance.get_field_value0(flags_field_index()).expect_int()
}

pub fn set_flags(instance: ClassInstanceRef, value: jint) {
	instance.put_field_value0(flags_field_index(), Operand::Int(value));
}

/// `java.lang.invoke.MemberName#method` field
pub fn method(instance: ClassInstanceRef) -> Reference {
	instance
		.get_field_value0(method_field_index())
		.expect_reference()
}

pub fn set_method(instance: ClassInstanceRef, value: Reference) {
	instance.put_field_value0(method_field_index(), Operand::Reference(value));
}

/// Injected `java.lang.invoke.MemberName#vmindex` field
///
/// **NOTE**: For [`Method`]s, this is an index into the [`VTable`]. For [`Field`]s, this is the [**byte offset**].
///
/// [`VTable`]: crate::objects::vtable::VTable
/// [`Field`]: crate::objects::field::Field
/// [**byte offset**]: crate::objects::field::Field::offset
pub fn vmindex(instance: ClassInstanceRef) -> jlong {
	instance
		.get_field_value0(vmindex_field_index())
		.expect_long()
}

pub fn set_vmindex(instance: ClassInstanceRef, value: jlong) {
	instance.put_field_value0(vmindex_field_index(), Operand::Long(value));
}

pub fn target_method(instance: ClassInstanceRef) -> Throws<&'static Method> {
	let vmindex = vmindex(instance);

	let defining_class_mirror = clazz(instance)?;
	let defining_class = defining_class_mirror.target_class();

	Throws::Ok(&defining_class.vtable()[vmindex as usize])
}

crate::classes::field_module! {
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
	/// [`Method`] offset in target class [`VTable`] *or* [`Field`] offset
	///
	/// Expected type: `jlong`
	/// [`Method`]: crate::objects::method::Method
	/// [`VTable`]: crate::objects::vtable::VTable
	/// [`Field`]: crate::objects::field::Field
	@INJECTED vmindex: FieldType::Long => jni::sys::jlong,
}
