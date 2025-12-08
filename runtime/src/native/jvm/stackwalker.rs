#![native_macros::jni_fn_module]

use jni::env::JniEnv;
use jni::objects::{JByteArray, JClass, JObject, JObjectArray, JString};
use jni::sys::{jboolean, jint, jlong};
use native_macros::jni_call;

#[jni_call]
pub extern "C" fn JVM_ExpandStackFrameInfo(_env: JniEnv, _obj: JObject) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_CallStackWalk(
	_env: JniEnv,
	_stack_stream: JObject,
	_mode: jint,
	_skip_frames: jint,
	_cont_scope: JObject,
	_cont: JObject,
	_buffer_size: jint,
	_start_index: jint,
	_frames: JObjectArray,
) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_MoreStackWalk(
	_env: JniEnv,
	_stack_stream: JObject,
	_mode: jint,
	_anchor: jlong,
	_last_batch_count: jint,
	_buffer_size: jint,
	_start_index: jint,
	_frames: JObjectArray,
) -> jint {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_SetStackWalkContinuation(
	_env: JniEnv,
	_stack_stream: JObject,
	_anchor: jlong,
	_frames: JObjectArray,
	_cont: JObject,
) {
	todo!()
}
