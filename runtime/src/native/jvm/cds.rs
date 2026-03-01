#![native_macros::jni_fn_module]

use jni::env::JniEnv;
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jboolean, jint, jlong};
use native_macros::jni_call;

#[jni_call]
pub extern "C" fn JVM_InitializeFromArchive(_env: JniEnv, _cls: JClass) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_RegisterLambdaProxyClassForArchiving(
	_env: JniEnv,
	_caller: JClass,
	_interface_method_name: JString,
	_factory_type: JObject,
	_interface_method_type: JObject,
	_implementation_member: JObject,
	_dynamic_method_type: JObject,
	_lambda_proxy_class: JClass,
) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_LookupLambdaProxyClassFromArchive(
	_env: JniEnv,
	_caller: JClass,
	_interface_method_name: JString,
	_factory_type: JObject,
	_interface_method_type: JObject,
	_implementation_member: JObject,
	_dynamic_method_type: JObject,
) -> JClass {
	todo!()
}

#[jni_call(no_env)]
pub extern "C" fn JVM_GetRandomSeedForDumping() -> jlong {
	todo!()
}

#[jni_call(no_env)]
pub extern "C" fn JVM_GetCDSConfigStatus() -> jint {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_LogLambdaFormInvoker(_env: JniEnv, _line: JString) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_DumpClassListToFile(_env: JniEnv, _list_file_name: JString) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_DumpDynamicArchive(_env: JniEnv, _archive_name: JString) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_NeedsClassInitBarrierForCDS(_env: JniEnv, _cls: JClass) -> jboolean {
	todo!()
}
