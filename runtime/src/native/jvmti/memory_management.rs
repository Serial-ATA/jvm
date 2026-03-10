use jni::sys::jlong;
use jvmti::env::JvmtiEnv;
use jvmti::error::JvmtiError;
use native_macros::jvmti_call;
use std::ffi::c_uchar;

#[jvmti_call]
pub extern "system" fn Allocate(
	_env: JvmtiEnv,
	_size: jlong,
	_mem_ptr: *mut *mut c_uchar,
) -> JvmtiError {
	todo!("jvmtiEnv::Allocate")
}

#[jvmti_call]
pub extern "system" fn Deallocate(_env: JvmtiEnv, _mem: *mut c_uchar) -> JvmtiError {
	todo!("jvmtiEnv::Deallocate")
}
