use crate::classes::AsClassInstanceRef;
use crate::modules::Module;
use crate::objects::instance::Instance;
use crate::objects::reference::Reference;

use classfile::FieldType;
use instructions::Operand;
use jni::sys::jlong;

pub fn injected_module_ptr_for(obj: Reference) -> Option<*const Module> {
	let ptr = obj.get_field_value0(module_ptr_field_index()).expect_long();
	if ptr == 0 {
		// Field not initialized yet.
		return None;
	}

	Some(ptr as *const Module)
}

pub fn set_injected_module_ptr_for(obj: Reference, ptr: jlong) {
	obj.put_field_value0(module_ptr_field_index(), Operand::Long(ptr))
}

/// `java.lang.Module#name` field
pub fn name<I: AsClassInstanceRef>(instance: I) -> Reference {
	instance
		.as_class_instance_ref()
		.get_field_value0(name_field_index())
		.expect_reference()
}

pub fn set_name<I: AsClassInstanceRef>(instance: I, value: Reference) {
	instance
		.as_class_instance_ref()
		.put_field_value0(name_field_index(), Operand::Reference(value))
}

/// `java.lang.Module#loader` field
pub fn loader<I: AsClassInstanceRef>(instance: I) -> Reference {
	instance
		.as_class_instance_ref()
		.get_field_value0(loader_field_index())
		.expect_reference()
}

pub fn set_loader<I: AsClassInstanceRef>(instance: I, value: Reference) {
	instance
		.as_class_instance_ref()
		.put_field_value0(loader_field_index(), Operand::Reference(value))
}

crate::classes::field_module! {
	@CLASS java_lang_Module;

	@FIELDSTART
	/// [`Module`] pointer field
	///
	/// Expected type: `jlong`
	/// [`Module`]: crate::modules::Module
	@INJECTED module_ptr: FieldType::Long => jni::sys::jlong,

	/// `java.lang.Module#name` field offset
	///
	/// Expected type: `Reference` to `java.lang.String`
	@FIELD name: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
	/// `java.lang.Module#loader` field offset
	///
	/// Expected type: `Reference` to `java.lang.ClassLoader`
	@FIELD loader: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/ClassLoader"),
}
