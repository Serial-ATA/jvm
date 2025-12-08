#![native_macros::jni_fn_module]

use jni::env::JniEnv;
use jni::objects::{JClass, JObject, JObjectArray, JString};
use jni::sys::{jboolean, jint, jlong};
use native_macros::jni_call;

#[jni_call]
pub extern "C" fn JVM_StartThread(_env: JniEnv, _thread: JObject) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_SetThreadPriority(_env: JniEnv, _thread: JObject, _priority: jint) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_Yield(_env: JniEnv, _class: JClass) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_SleepNanos(_env: JniEnv, _class: JClass, _nanos: jlong) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_CurrentCarrierThread(_env: JniEnv, _class: JClass) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_CurrentThread(_env: JniEnv, _class: JClass) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_SetCurrentThread(_env: JniEnv, _this: JObject, _thread: JObject) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetNextThreadIdOffset(_env: JniEnv, _class: JClass) -> jlong {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_Interrupt(_env: JniEnv, _thread: JObject) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_HoldsLock(_env: JniEnv, _class: JClass, _thread: JObject) -> jboolean {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetStackTrace(_env: JniEnv, _thread: JObject) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_CreateThreadSnapshot(_env: JniEnv, _thread: JObject) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_SetNativeThreadName(_env: JniEnv, _thread: JObject, _name: JString) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_ScopedValueCache(_env: JniEnv, _class: JClass) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_SetScopedValueCache(_env: JniEnv, _class: JClass, _cache: JObject) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetAllThreads(_env: JniEnv, _dummy: JClass) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_DumpThreads(
	_env: JniEnv,
	_thread_class: JClass,
	_threads: JObjectArray,
) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_VirtualThreadStart(_env: JniEnv, _vthread: JObject) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_VirtualThreadEnd(_env: JniEnv, _vthread: JObject) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_VirtualThreadMount(_env: JniEnv, _vthread: JObject, _hide: jboolean) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_VirtualThreadUnmount(_env: JniEnv, _vthread: JObject, _hide: jboolean) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_VirtualThreadDisableSuspend(_env: JniEnv, _class: JClass, _enter: jboolean) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_VirtualThreadPinnedEvent(_env: JniEnv, _class: JClass, _op: JString) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_TakeVirtualThreadListToUnblock(_env: JniEnv, _class: JClass) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_EnsureMaterializedForStackWalk_func(
	_env: JniEnv,
	_vthread: JObject,
	_value: JObject,
) {
	todo!()
}
