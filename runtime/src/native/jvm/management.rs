#![native_macros::jni_fn_module]

use std::ffi::c_void;

use jni::sys::jint;
use native_macros::jni_call;

#[jni_call(no_env, no_strict_types)]
pub extern "C" fn JVM_GetManagement(_version: jint) -> *mut c_void {
	todo!()
}
