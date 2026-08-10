use std::ffi::c_void;

use jni::objects::{JClass, JObject};
use jvmti::env::JvmtiEnv;
use jvmti::error::JvmtiError;
use jvmti::sys::{
	jvmtiHeapObjectCallback, jvmtiHeapObjectFilter, jvmtiHeapRootCallback,
	jvmtiObjectReferenceCallback, jvmtiStackReferenceCallback,
};
use native_macros::jvmti_call;

#[jvmti_call]
pub extern "system" fn IterateOverObjectsReachableFromObject(
	_env: JvmtiEnv,
	_object: JObject,
	_object_reference_callback: jvmtiObjectReferenceCallback,
	_user_data: *const c_void,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::IterateOverObjectsReachableFromObject")
}

#[jvmti_call]
pub extern "system" fn IterateOverReachableObjects(
	_env: JvmtiEnv,
	_heap_root_callback: jvmtiHeapRootCallback,
	_stack_ref_callback: jvmtiStackReferenceCallback,
	_object_ref_callback: jvmtiObjectReferenceCallback,
	_user_data: *const c_void,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::IterateOverReachableObjects")
}

#[jvmti_call]
pub extern "system" fn IterateOverHeap(
	_env: JvmtiEnv,
	_object_filter: jvmtiHeapObjectFilter,
	_heap_object_callback: jvmtiHeapObjectCallback,
	_user_data: *const c_void,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::IterateOverHeap")
}

#[jvmti_call]
pub extern "system" fn IterateOverInstancesOfClass(
	_env: JvmtiEnv,
	_klass: JClass,
	_object_filter: jvmtiHeapObjectFilter,
	_heap_object_callback: jvmtiHeapObjectCallback,
	_user_data: *const c_void,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::IterateOverInstancesOfClass")
}
