use crate::modules::Module;
use crate::objects::instance::Instance;
use crate::objects::reference::Reference;
use classfile::FieldType;

pub fn injected_module_ptr_for(obj: Reference) -> Option<*const Module> {
	let ptr = obj
		.get_field_value0(module_ptr_field_offset())
		.expect_long();
	if ptr == 0 {
		// Field not initialized yet.
		return None;
	}

	Some(ptr as *const Module)
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
