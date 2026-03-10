use jni::sys::jint;
use jvmti::env::JvmtiEnv;
use jvmti::error::JvmtiError;
use jvmti::objects::JThreadGroup;
use jvmti::sys::{jthread, jthreadGroup, jvmtiThreadGroupInfo};
use native_macros::jvmti_call;

#[jvmti_call]
pub extern "system" fn GetTopThreadGroups(
	_env: JvmtiEnv,
	_group_count_ptr: *mut jint,
	_groups_ptr: *mut *mut jthreadGroup,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::GetTopThreadGroups")
}

#[jvmti_call]
pub extern "system" fn GetThreadGroupInfo(
	_env: JvmtiEnv,
	_group: JThreadGroup,
	_info_ptr: *mut jvmtiThreadGroupInfo,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::GetThreadGroupInfo")
}

#[jvmti_call]
pub extern "system" fn GetThreadGroupChildren(
	_env: JvmtiEnv,
	_group: JThreadGroup,
	_thread_count_ptr: *mut jint,
	_threads_ptr: *mut *mut jthread,
	_group_count_ptr: *mut jint,
	_groups_ptr: *mut *mut jthreadGroup,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::GetThreadGroupChildren")
}
