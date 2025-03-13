use crate::classpath::loader::ClassLoader;
use crate::objects::instance::Instance;
use crate::objects::reference::Reference;
use classfile::FieldType;

pub fn injected_loader_ptr_for(obj: Reference) -> Option<*const ClassLoader> {
	let ptr = obj
		.get_field_value0(loader_ptr_field_offset())
		.expect_long();
	if ptr == 0 {
		// Field not initialized yet.
		return None;
	}

	Some(ptr as *const ClassLoader)
}

/// Checks the `java.lang.ClassLoader#parallelLockMap` field for null
pub fn parallelCapable(instance: &Reference) -> bool {
	!instance
		.get_field_value0(parallelCapable_field_offset())
		.expect_reference()
		.is_null()
}

super::field_module! {
	@CLASS java_lang_ClassLoader;

	@FIELDSTART
	/// [`ClassLoader`] pointer field
	///
	/// Expected type: `jlong`
	/// [`ClassLoader`]: crate::classpath::loader::ClassLoader
	@INJECTED loader_ptr: FieldType::Long => jni::sys::jlong,

	/// `java.lang.ClassLoader#name` field offset
	///
	/// Expected type: `Reference` to `java.lang.String`
	@FIELD name: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
	/// `java.lang.ClassLoader#unnamedModule` field offset
	///
	/// Expected type: `Reference` to `java.lang.Module`
	@FIELD unnamedModule: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Module"),
	/// `java.lang.ClassLoader#nameAndId` field offset
	///
	/// Expected type: `Reference` to `java.lang.String`
	@FIELD nameAndId: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
	/// `java.lang.ClassLoader#parallelLockMap` field offset
	///
	/// Expected type: `Reference` to `java.lang.util.concurrent.ConcurrentHashMap`
	[sym: parallelLockMap] @FIELD parallelCapable: FieldType::Object(_),
}
