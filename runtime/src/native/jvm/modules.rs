#![native_macros::jni_fn_module]

use jni::env::JniEnv;
use jni::objects::{JObject, JObjectArray, JString};
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
pub extern "C" fn JVM_SetBootLoaderUnnamedModule(_env: JniEnv, _module: JObject) {
	todo!()
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
