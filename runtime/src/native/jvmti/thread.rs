use jni::objects::JObject;
use jni::sys::{jint, jlong, jobject};
use jvmti::env::JvmtiEnv;
use jvmti::error::JvmtiError;
use jvmti::objects::JThread;
use jvmti::sys::{
	jthread, jvmtiError, jvmtiMonitorStackDepthInfo, jvmtiStartFunction, jvmtiThreadInfo,
};
use native_macros::jvmti_call;
use std::ffi::{c_uchar, c_void};

#[jvmti_call]
pub extern "system" fn GetThreadState(
	_env: JvmtiEnv,
	_thread: JThread,
	_thread_state_ptr: *mut jint,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::GetThreadState")
}

#[jvmti_call]
pub extern "system" fn GetCurrentThread(_env: JvmtiEnv, _thread_ptr: *mut jthread) -> JvmtiError {
	unimplemented!("jvmtiEnv::GetCurrentThread")
}

#[jvmti_call]
pub extern "system" fn GetAllThreads(
	_env: JvmtiEnv,
	_threads_count_ptr: *mut jint,
	_threads_ptr: *mut *mut jthread,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::GetAllThreads")
}

#[jvmti_call]
pub extern "system" fn SuspendThread(_env: JvmtiEnv, _thread: JThread) -> JvmtiError {
	unimplemented!("jvmtiEnv::SuspendThread")
}

#[jvmti_call]
pub extern "system" fn SuspendThreadList(
	_env: JvmtiEnv,
	_request_count: jint,
	_request_list: *const jthread,
	_results: *mut jvmtiError,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::SuspendThreadList")
}

#[jvmti_call]
pub extern "system" fn ResumeThread(_env: JvmtiEnv, _thread: JThread) -> JvmtiError {
	unimplemented!("jvmtiEnv::ResumeThread")
}

#[jvmti_call]
pub extern "system" fn ResumeThreadList(
	_env: JvmtiEnv,
	_request_count: jint,
	_request_list: *const jthread,
	_results: *mut jvmtiError,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::ResumeThreadList")
}

#[jvmti_call]
pub extern "system" fn StopThread(
	_env: JvmtiEnv,
	_thread: JThread,
	_exception: JObject,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::StopThread")
}

#[jvmti_call]
pub extern "system" fn InterruptThread(_env: JvmtiEnv, _thread: JThread) -> JvmtiError {
	unimplemented!("jvmtiEnv::InterruptThread")
}

#[jvmti_call]
pub extern "system" fn GetThreadInfo(
	_env: JvmtiEnv,
	_thread: JThread,
	_info_ptr: *mut jvmtiThreadInfo,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::GetThreadInfo")
}

#[jvmti_call]
pub extern "system" fn GetOwnedMonitorInfo(
	_env: JvmtiEnv,
	_thread: JThread,
	_owned_monitor_count_ptr: *mut jint,
	_owned_monitors_ptr: *mut *mut jobject,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::GetOwnedMonitorInfo")
}

#[jvmti_call]
pub extern "system" fn GetOwnedMonitorStackDepthInfo(
	_env: JvmtiEnv,
	_thread: JThread,
	_monitor_info_count_ptr: *mut jint,
	_monitor_info_ptr: *mut *mut jvmtiMonitorStackDepthInfo,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::GetOwnedMonitorStackDepthInfo")
}

#[jvmti_call]
pub extern "system" fn GetCurrentContendedMonitor(
	_env: JvmtiEnv,
	_thread: JThread,
	_monitor_ptr: *mut jobject,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::GetCurrentContendedMonitor")
}

#[jvmti_call]
pub extern "system" fn RunAgentThread(
	_env: JvmtiEnv,
	_thread: JThread,
	_proc: jvmtiStartFunction,
	_arg: *const c_void,
	_priority: jint,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::RunAgentThread")
}

#[jvmti_call]
pub extern "system" fn SetThreadLocalStorage(
	_env: JvmtiEnv,
	_thread: JThread,
	_data: *const c_void,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::SetThreadLocalStorage")
}

#[jvmti_call]
pub extern "system" fn GetThreadLocalStorage(
	_env: JvmtiEnv,
	_thread: JThread,
	_data_ptr: *mut *mut c_void,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::GetThreadLocalStorage")
}
