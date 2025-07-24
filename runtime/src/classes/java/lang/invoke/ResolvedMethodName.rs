use crate::globals;
use crate::objects::instance::Instance;
use crate::objects::instance::class::{ClassInstance, ClassInstanceRef};
use crate::objects::instance::mirror::MirrorInstanceRef;
use crate::objects::method::Method;
use crate::objects::reference::Reference;

use classfile::FieldType;
use instructions::Operand;
use jni::sys::jlong;

pub fn new(method: &'static Method) -> Reference {
	let instance = ClassInstance::new(globals::classes::java_lang_invoke_ResolvedMethodName());

	set_vmtarget(instance, method);
	set_vmholder(instance, method.class().mirror());

	Reference::class(instance)
}

pub fn vmholder(instance: ClassInstanceRef) -> Reference {
	instance
		.get_field_value0(vmholder_field_index())
		.expect_reference()
}

pub fn set_vmholder(instance: ClassInstanceRef, value: MirrorInstanceRef) {
	instance.put_field_value0(
		vmholder_field_index(),
		Operand::Reference(Reference::mirror(value)),
	)
}

pub fn vmtarget(instance: ClassInstanceRef) -> Option<&'static Method> {
	let ptr = instance
		.get_field_value0(vmtarget_field_index())
		.expect_long();
	if ptr == 0 {
		return None;
	}

	let ptr = ptr as *const Method;
	Some(unsafe { &*ptr })
}

pub fn set_vmtarget(instance: ClassInstanceRef, value: &'static Method) {
	instance.put_field_value0(
		vmtarget_field_index(),
		Operand::Long(value as *const Method as jlong),
	)
}

crate::classes::field_module! {
	@CLASS java_lang_invoke_ResolvedMethodName;

	@FIELDSTART
	/// `java.lang.invoke.ResolvedMethodName#vmholder` field offset
	///
	/// Expected field type: `Reference` to `java.lang.Class`
	@FIELD vmholder: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Class"),
	/// [`Method`] pointer
	///
	/// Expected type: `jlong`
	/// [`Method`]: crate::objects::method::Method
	@INJECTED vmtarget: FieldType::Long => jni::sys::jlong,
}
