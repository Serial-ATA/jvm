#![native_macros::jni_fn_module]

use jni::sys::jint;
use native_macros::jni_call;
use std::ffi::c_void;

#[jni_call(no_env, no_strict_types)]
pub extern "C" fn JVM_RawMonitorCreate() -> *mut c_void {
	todo!()
}

#[jni_call(no_env, no_strict_types)]
pub extern "C" fn JVM_RawMonitorDestroy(_mon: *mut c_void) {
	todo!()
}

#[jni_call(no_env, no_strict_types)]
pub extern "C" fn JVM_RawMonitorEnter(_mon: *mut c_void) -> jint {
	todo!()
}

#[jni_call(no_env, no_strict_types)]
pub extern "C" fn JVM_RawMonitorExit(_mon: *mut c_void) {
	todo!()
}
