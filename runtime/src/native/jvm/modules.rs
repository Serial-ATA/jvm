#![native_macros::jni_fn_module]

use crate::classes;
use crate::classpath::loader::ClassLoader;
use crate::modules::Module;
use crate::native::jni::reference_from_jobject;
use crate::thread::JavaThread;
use crate::thread::exceptions::{handle_exception, throw};
use jni::env::JniEnv;
use jni::objects::{JClass, JObject, JObjectArray, JString};
use jni::sys::jboolean;
use native_macros::jni_call;

#[jni_call]
pub extern "C" fn JVM_DefineModule(
	_env: JniEnv,
	_module: JObject,
	_is_open: jboolean,
	_version: JString,
	_location: JString,
	_packages: JObjectArray,
) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_SetBootLoaderUnnamedModule(env: JniEnv, module: JObject) {
	let Some(module) = (unsafe { reference_from_jobject(module.raw()) }) else {
		panic!("Attempting to SetBootLoaderUnnamedModule with a null reference");
	};

	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	let module_entry_result = Module::unnamed(module);
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

#[jni_call]
pub extern "C" fn JVM_AddModuleExports(
	_env: JniEnv,
	_from_module: JObject,
	_package: JString,
	_to_module: JObject,
) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_AddModuleExportsToAllUnnamed(
	_env: JniEnv,
	_from_module: JObject,
	_package: JString,
) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_AddModuleExportsToAll(
	_env: JniEnv,
	_from_module: JObject,
	_package: JString,
) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_AddReadsModule(_env: JniEnv, _from_module: JObject, _source_module: JObject) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_DefineArchivedModules(
	_env: JniEnv,
	_platform_loader: JObject,
	_system_loader: JObject,
) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetSystemPackage(_env: JniEnv, _name: JString) -> JString {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetSystemPackages(_env: JniEnv) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_IsSameClassPackage(
	_env: JniEnv,
	_class1: JClass,
	_class2: JClass,
) -> jboolean {
	todo!()
}
