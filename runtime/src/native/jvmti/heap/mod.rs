use std::ffi::c_void;

use jni::objects::{JClass, JObject};
use jni::sys::{jint, jlong, jobject};
use jvmti::env::JvmtiEnv;
use jvmti::error::JvmtiError;
use jvmti::sys::jvmtiHeapCallbacks;
use native_macros::jvmti_call;

mod v1_0;

#[jvmti_call]
pub extern "system" fn FollowReferences(
	_env: JvmtiEnv,
	_heap_filter: jint,
	_klass: JClass,
	_initial_object: JObject,
	_callbacks: *const jvmtiHeapCallbacks,
	_user_data: *const c_void,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::FollowReferences")
}

#[jvmti_call]
pub extern "system" fn IterateThroughHeap(
	_env: JvmtiEnv,
	_heap_filter: jint,
	_klass: JClass,
	_callbacks: *const jvmtiHeapCallbacks,
	_user_data: *const c_void,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::IterateThroughHeap")
}

#[jvmti_call]
pub extern "system" fn GetTag(
	_env: JvmtiEnv,
	_object: JObject,
	_tag_ptr: *mut jlong,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::GetTag")
}

#[jvmti_call]
pub extern "system" fn SetTag(_env: JvmtiEnv, _object: JObject, _tag: jlong) -> JvmtiError {
	unimplemented!("jvmtiEnv::SetTag")
}

#[jvmti_call]
pub extern "system" fn GetObjectsWithTags(
	_env: JvmtiEnv,
	_tag_count: jint,
	_tags: *const jlong,
	_count_ptr: *mut jint,
	_object_result_ptr: *mut *mut jobject,
	_tag_result_ptr: *mut *mut jlong,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::GetObjectsWithTags")
}

#[jvmti_call]
pub extern "system" fn ForceGarbageCollection(_env: JvmtiEnv) -> JvmtiError {
	unimplemented!("jvmtiEnv::ForceGarbageCollection")
}
