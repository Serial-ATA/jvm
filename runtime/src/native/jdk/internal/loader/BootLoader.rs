use crate::objects::class_instance::Instance;
use crate::objects::reference::Reference;
use crate::thread::JavaThread;

use std::ptr::NonNull;

use ::jni::env::JniEnv;

include_generated!("native/jdk/internal/loader/def/BootLoader.definitions.rs");

/// Returns an array of the binary name of the packages defined by the boot loader, in VM
/// internal form (forward slashes instead of dot).
pub fn getSystemPackageNames(_env: NonNull<JniEnv>) -> Reference /* String[] */ {
	unimplemented!("jdk.internal.loader.BootLoader#getSystemPackageNames")
}

/// Returns the location of the package of the given name, if defined by the boot loader;
/// otherwise `null` is returned.
///
/// The location may be a module from the runtime image or exploded image, or from the boot class
/// append path (i.e. -Xbootclasspath/a or BOOT-CLASS-PATH attribute specified in java agent).
pub fn getSystemPackageLocation(
	_env: NonNull<JniEnv>,
	_name: Reference, // java.lang.String
) -> Reference /* java.lang.String */ {
	unimplemented!("jdk.internal.loader.BootLoader#getSystemPackageLocation")
}

pub fn setBootLoaderUnnamedModule0(
	env: NonNull<JniEnv>,
	module: Reference, // java.lang.Module
) {
	if module.is_null() {
		let _thread = unsafe { JavaThread::for_env(env.as_ptr()) };
		panic!("NullPointerException"); // TODO
	}

	if !module.is_instance_of(crate::globals::classes::java_lang_Module()) {
		let _thread = unsafe { JavaThread::for_env(env.as_ptr()) };
		panic!("IllegalArgumentException"); // TODO
	}

	let name = module
		.get_field_value0(crate::globals::field_offsets::java_lang_Module::name_field_offset())
		.expect_reference();
	if !name.is_null() {
		let _thread = unsafe { JavaThread::for_env(env.as_ptr()) };
		panic!("IllegalArgumentException"); // TODO
	}

	let loader = module
		.get_field_value0(crate::globals::field_offsets::java_lang_Module::loader_field_offset())
		.expect_reference();
	if !loader.is_null() {
		let _thread = unsafe { JavaThread::for_env(env.as_ptr()) };
		panic!("IllegalArgumentException"); // TODO
	}

	tracing::warn!("(!!!) UNIMPLEMENTED jdk.internal.loader.BootLoader#setBootLoaderUnnamedModule0")
}
