use crate::classpath::classloader::ClassLoader;
use crate::modules::Module;
use crate::objects::class::Class;
use crate::objects::instance::Instance;
use crate::objects::reference::Reference;
use crate::thread::JavaThread;

use std::ptr::NonNull;

use ::jni::env::JniEnv;

include_generated!("native/jdk/internal/loader/def/BootLoader.definitions.rs");

/// Returns an array of the binary name of the packages defined by the boot loader, in VM
/// internal form (forward slashes instead of dot).
pub fn getSystemPackageNames(_env: NonNull<JniEnv>, _class: &'static Class) -> Reference /* String[] */
{
	unimplemented!("jdk.internal.loader.BootLoader#getSystemPackageNames")
}

/// Returns the location of the package of the given name, if defined by the boot loader;
/// otherwise `null` is returned.
///
/// The location may be a module from the runtime image or exploded image, or from the boot class
/// append path (i.e. -Xbootclasspath/a or BOOT-CLASS-PATH attribute specified in java agent).
pub fn getSystemPackageLocation(
	_env: NonNull<JniEnv>,
	_class: &'static Class,
	_name: Reference, // java.lang.String
) -> Reference /* java.lang.String */ {
	unimplemented!("jdk.internal.loader.BootLoader#getSystemPackageLocation")
}

/// # Throws
///
/// `IllegalArgumentException` is thrown if:
/// * Module is named
/// * Module is not an instance or subclass of j.l.r.Module
/// * Module is not loaded by the bootLoader
pub fn setBootLoaderUnnamedModule0(
	env: NonNull<JniEnv>,
	_class: &'static Class,
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
		.get_field_value0(crate::globals::fields::java_lang_Module::name_field_offset())
		.expect_reference();
	if !name.is_null() {
		let _thread = unsafe { JavaThread::for_env(env.as_ptr()) };
		panic!("IllegalArgumentException"); // TODO
	}

	let loader = module
		.get_field_value0(crate::globals::fields::java_lang_Module::loader_field_offset())
		.expect_reference();
	if !loader.is_null() {
		let _thread = unsafe { JavaThread::for_env(env.as_ptr()) };
		panic!("IllegalArgumentException"); // TODO
	}

	let module = Module::unnamed(module);
	ClassLoader::set_bootloader_unnamed_module(module);
}
