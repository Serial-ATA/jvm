use crate::objects::class_instance::ClassInstance;
use crate::objects::instance::Instance;
use crate::objects::method::Method;
use crate::objects::reference::{MirrorInstanceRef, Reference};
use classfile::FieldType;
use instructions::Operand;
use jni::sys::jlong;

pub fn vmholder(instance: &ClassInstance) -> Reference {
	instance
		.get_field_value0(vmholder_field_offset())
		.expect_reference()
}

pub fn set_vmholder(instance: &mut ClassInstance, value: MirrorInstanceRef) {
	instance.put_field_value0(
		vmholder_field_offset(),
		Operand::Reference(Reference::mirror(value)),
	)
}

pub fn vmtarget(instance: &ClassInstance) -> Option<&'static Method> {
	let ptr = instance
		.get_field_value0(vmtarget_field_offset())
		.expect_long();
	if ptr == 0 {
		return None;
	}

	let ptr = ptr as *const Method;
	Some(unsafe { &*ptr })
}

pub fn set_vmtarget(instance: &mut ClassInstance, value: &'static Method) {
	instance.put_field_value0(
		vmtarget_field_offset(),
		Operand::Long(value as *const Method as jlong),
	)
}

super::field_module! {
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
