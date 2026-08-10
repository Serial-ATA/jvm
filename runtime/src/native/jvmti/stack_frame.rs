use jni::sys::{jint, jmethodID};
use jvmti::env::JvmtiEnv;
use jvmti::error::JvmtiError;
use jvmti::objects::JThread;
use jvmti::sys::{jlocation, jthread, jvmtiFrameInfo, jvmtiStackInfo};
use native_macros::jvmti_call;

#[jvmti_call]
pub extern "system" fn GetStackTrace(
	_env: JvmtiEnv,
	_thread: JThread,
	_start_depth: jint,
	_max_frame_count: jint,
	_frame_buffer: *mut jvmtiFrameInfo,
	_count_ptr: *mut jint,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::GetStackTrace")
}

#[jvmti_call]
pub extern "system" fn GetAllStackTraces(
	_env: JvmtiEnv,
	_max_frame_count: jint,
	_stack_info_ptr: *mut jvmtiStackInfo,
	_thread_count_ptr: *mut jint,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::GetAllStackTraces")
}

#[jvmti_call]
pub extern "system" fn GetThreadListStackTraces(
	_env: JvmtiEnv,
	_thread_count: jint,
	_thread_list: *const jthread,
	_max_frame_count: jint,
	_stack_info_ptr: *mut *mut jvmtiStackInfo,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::GetThreadListStackTraces")
}

#[jvmti_call]
pub extern "system" fn GetFrameCount(
	_env: JvmtiEnv,
	_thread: JThread,
	_count_ptr: *mut jint,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::GetFrameCount")
}

#[jvmti_call]
pub extern "system" fn PopFrame(_env: JvmtiEnv, _thread: JThread) -> JvmtiError {
	unimplemented!("jvmtiEnv::PopFrame")
}

#[jvmti_call]
pub extern "system" fn GetFrameLocation(
	_env: JvmtiEnv,
	_thread: JThread,
	_depth: jint,
	_method_ptr: *mut jmethodID,
	_location_ptr: *mut jlocation,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::GetFrameLocation")
}

#[jvmti_call]
pub extern "system" fn NotifyFramePop(
	_env: JvmtiEnv,
	_thread: JThread,
	_depth: jint,
) -> JvmtiError {
	unimplemented!("jvmtiEnv::NotifyFramePop")
}
