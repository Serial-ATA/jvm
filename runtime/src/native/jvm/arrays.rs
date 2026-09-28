#![native_macros::jni_fn_module]

use std::ffi::c_uchar;

use jni::env::JniEnv;
use jni::objects::{JArray, JClass, JIntArray, JObject};
use jni::sys::{jboolean, jint, jvalue};
use native_macros::jni_call;

#[jni_call]
pub extern "C" fn JVM_CopyOfSpecialArray(
	_env: JniEnv,
	_array: JArray,
	_from: jint,
	_to: jint,
) -> JArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_IsAtomicArray(_env: JniEnv, _array: JArray) -> jboolean {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_IsFlatArray(_env: JniEnv, _array: JArray) -> jboolean {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_IsNullRestrictedArray(_env: JniEnv, _array: JArray) -> jboolean {
	todo!()
}

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
	_v_code: c_uchar,
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
	_dimensions: JIntArray,
) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_NewNullRestrictedAtomicArray(
	_env: JniEnv,
	_element_class: JClass,
	_len: jint,
	_init_value: JObject,
) -> JArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_NewNullRestrictedNonAtomicArray(
	_env: JniEnv,
	_element_class: JClass,
	_len: jint,
	_init_value: JObject,
) -> JArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_NewNullableAtomicArray(
	_env: JniEnv,
	_element_class: JClass,
	_len: jint,
) -> JArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_NewReferenceArray(
	_env: JniEnv,
	_element_class: JClass,
	_len: jint,
) -> JArray {
	todo!()
}
