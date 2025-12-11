#![native_macros::jni_fn_module]

use std::time::{SystemTime, UNIX_EPOCH};

use ::jni::env::JniEnv;
use ::jni::objects::{JClass, JObject, JObjectArray, JString};
use ::jni::sys::{jint, jlong};
use native_macros::jni_call;

#[jni_call]
pub extern "C" fn JVM_CurrentTimeMillis(_env: JniEnv, _unused: JClass) -> jlong {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_NanoTime(_env: JniEnv, _unused: JClass) -> jlong {
	let time_nanos = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.expect("current system time should not be before the UNIX epoch")
		.as_nanos();

	time_nanos as jlong
}

#[jni_call]
pub extern "C" fn JVM_GetNanoTimeAdjustment(
	_env: JniEnv,
	_unused: JClass,
	_offset_secs: jlong,
) -> jlong {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_ArrayCopy(
	_env: JniEnv,
	_unused: JClass,
	_src: JObject,
	_src_pos: jint,
	_dst: JObject,
	_dst_pos: jint,
	_length: jint,
) -> jlong {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetProperties(_env: JniEnv) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetTemporaryDirectory(_env: JniEnv) -> JString {
	todo!()
}
