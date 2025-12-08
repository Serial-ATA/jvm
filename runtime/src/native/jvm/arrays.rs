#![native_macros::jni_fn_module]

use jni::env::JniEnv;
use jni::objects::{JClass, JObject, JValue};
use jni::sys::{jboolean, jbyte, jint, jvalue};
use native_macros::jni_call;

#[jni_call]
pub extern "C" fn JVM_GetArrayLength(_env: JniEnv, _array: JObject) -> jint {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetArrayElement(_env: JniEnv, _array: JObject, _index: jint) -> JObject {
	todo!()
}

// TODO: Support jvalue, remove no_strict_types
#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetPrimitiveArrayElement(
	_env: JniEnv,
	_array: JObject,
	_index: jint,
	_w_code: jint,
) -> jvalue {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_SetArrayElement(_env: JniEnv, _array: JObject, _index: jint, _val: JObject) {
	todo!()
}

// TODO: Support jvalue, remove no_strict_types
#[jni_call(no_strict_types)]
pub extern "C" fn JVM_SetPrimitiveArrayElement(
	_env: JniEnv,
	_array: JObject,
	_index: jint,
	_val: jvalue,
	_v_code: jbyte,
) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_NewArray(_env: JniEnv, _element_class: JClass, _length: jint) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_NewMultiArray(
	_env: JniEnv,
	_element_class: JClass,
	_dimensions: jint,
) -> JObject {
	todo!()
}
