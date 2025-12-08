#![native_macros::jni_fn_module]

use jni::env::JniEnv;
use jni::objects::{JObject, JObjectArray, JString, JThrowable};
use jni::sys::jint;
use native_macros::jni_call;

#[jni_call]
pub extern "C" fn JVM_FillInStackTrace(_env: JniEnv, _receiver: JObject) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetExtendedNPEMessage(_env: JniEnv, _throwable: JThrowable) -> JString {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_InitStackTraceElementArray(
	_env: JniEnv,
	_elements: JObjectArray,
	_backtrace: JObject,
	_depth: jint,
) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_InitStackTraceElement(
	_env: JniEnv,
	_element: JObject,
	_stack_frame_info: JObject,
) {
	todo!()
}
