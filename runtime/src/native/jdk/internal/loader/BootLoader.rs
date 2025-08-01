use crate::classes;
use crate::classpath::loader::ClassLoader;
use crate::modules::Module;
use crate::objects::class::ClassPtr;
use crate::objects::reference::Reference;
use crate::thread::JavaThread;
use crate::thread::exceptions::{handle_exception, throw};

use ::jni::env::JniEnv;

include_generated!("native/jdk/internal/loader/def/BootLoader.definitions.rs");

/// Returns an array of the binary name of the packages defined by the boot loader, in VM
/// internal form (forward slashes instead of dot).
pub fn getSystemPackageNames(_env: JniEnv, _class: ClassPtr) -> Reference /* String[] */
{
	unimplemented!("jdk.internal.loader.BootLoader#getSystemPackageNames")
}

/// Returns the location of the package of the given name, if defined by the boot loader;
/// otherwise `null` is returned.
///
/// The location may be a module from the runtime image or exploded image, or from the boot class
/// append path (i.e. -Xbootclasspath/a or BOOT-CLASS-PATH attribute specified in java agent).
pub fn getSystemPackageLocation(
	_env: JniEnv,
	_class: ClassPtr,
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
	env: JniEnv,
	_class: ClassPtr,
	module: Reference, // java.lang.Module
) {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	let module_entry_result = Module::unnamed(module.clone());
	let module_entry = handle_exception!(thread, module_entry_result);

	let loader = classes::java::lang::Module::loader(module);
	if !loader.is_null() {
		throw!(
			thread,
			IllegalArgumentException,
			"Class loader must be the boot class loader"
		);
	}

	ClassLoader::set_bootloader_unnamed_module(module_entry);
}
