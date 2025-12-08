#![native_macros::jni_fn_module]

use jni::env::JniEnv;
use jni::objects::{JClass, JObject, JObjectArray};
use native_macros::jni_call;

#[jni_call]
pub extern "C" fn JVM_InvokeMethod(
	_env: JniEnv,
	_method: JObject,
	_obj: JObject,
	_args0: JObjectArray,
) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_NewInstanceFromConstructor(
	_env: JniEnv,
	_c: JObject,
	_args0: JObjectArray,
) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetEnclosingMethodInfo(_env: JniEnv, _class: JClass) -> JObjectArray {
	todo!()
}
