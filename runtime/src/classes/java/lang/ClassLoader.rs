use crate::classes::AsClassInstanceRef;
use crate::classpath::loader::ClassLoader;
use crate::objects::instance::Instance;
use crate::objects::reference::Reference;

use classfile::FieldType;
use instructions::Operand;
use jni::sys::jlong;

pub fn injected_loader_ptr_for(obj: Reference) -> Option<*const ClassLoader> {
	let ptr = obj.get_field_value0(loader_ptr_field_index()).expect_long();
	if ptr == 0 {
		// Field not initialized yet.
		return None;
	}

	Some(ptr as *const ClassLoader)
}

pub fn set_injected_loader_ptr_for(obj: Reference, value: jlong) {
	obj.put_field_value0(loader_ptr_field_index(), Operand::Long(value));
}

/// `java.lang.ClassLoader#name` field
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

/// `java.lang.ClassLoader#unnamedModule` field
pub fn unnamedModule<I: AsClassInstanceRef>(instance: I) -> Reference {
	instance
		.as_class_instance_ref()
		.get_field_value0(unnamedModule_field_index())
		.expect_reference()
}

pub fn set_unnamedModule<I: AsClassInstanceRef>(instance: I, value: Reference) {
	instance
		.as_class_instance_ref()
		.put_field_value0(unnamedModule_field_index(), Operand::Reference(value))
}

/// `java.lang.ClassLoader#nameAndId` field
pub fn nameAndId<I: AsClassInstanceRef>(instance: I) -> Reference {
	instance
		.as_class_instance_ref()
		.get_field_value0(nameAndId_field_index())
		.expect_reference()
}

pub fn set_nameAndId<I: AsClassInstanceRef>(instance: I, value: Reference) {
	instance
		.as_class_instance_ref()
		.put_field_value0(nameAndId_field_index(), Operand::Reference(value))
}

/// Checks the `java.lang.ClassLoader#parallelLockMap` field for null
pub fn parallelCapable(instance: &Reference) -> bool {
	!instance
		.get_field_value0(parallelCapable_field_index())
		.expect_reference()
		.is_null()
}

crate::classes::field_module! {
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

pub mod calls {
	use crate::classpath::loader::ClassLoader;
	use crate::objects::instance::mirror::MirrorInstanceRef;
	use crate::objects::method::Method;
	use crate::objects::reference::Reference;
	use crate::symbols::sym;
	use crate::thread::JavaThread;
	use crate::thread::exceptions::Throws;
	use crate::{globals, java_call};

	use std::sync::LazyLock;

	use instructions::Operand;

	// TODO: Would be nice to have a macro similar to `field_module` which lets us define globally, resolved once, methods
	pub fn addClass(
		thread: &'static JavaThread,
		loader: &ClassLoader,
		class: MirrorInstanceRef,
	) -> Throws<()> {
		static ADD_CLASS_METHOD: LazyLock<&'static Method> = LazyLock::new(|| {
			// TODO: Ideally, promote this to an exception
			globals::classes::java_lang_ClassLoader()
				.resolve_method(sym!(addClass), sym!(Class_void_signature))
				.expect("method should exist")
		});

		let _result = java_call!(
			thread,
			&ADD_CLASS_METHOD,
			Operand::Reference(loader.obj()),
			Operand::Reference(Reference::mirror(class))
		);

		if thread.has_pending_exception() {
			return Throws::PENDING_EXCEPTION;
		}

		Throws::Ok(())
	}
}
