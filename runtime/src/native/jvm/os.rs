#![native_macros::jni_fn_module]

use std::ffi::{c_char, c_void};

use jni::sys::{jboolean, jint, jlong};
use native_macros::jni_call;

#[jni_call(no_env)]
pub extern "C" fn JVM_TotalMemory() -> jlong {
	todo!()
}

#[jni_call(no_env)]
pub extern "C" fn JVM_FreeMemory() -> jlong {
	todo!()
}

#[jni_call(no_env)]
pub extern "C" fn JVM_MaxMemory() -> jlong {
	todo!()
}

#[jni_call(no_env)]
pub extern "C" fn JVM_ActiveProcessorCount() -> jint {
	todo!()
}

#[jni_call(no_env)]
pub extern "C" fn JVM_IsUseContainerSupport() -> jboolean {
	todo!()
}

#[jni_call(no_env)]
pub extern "C" fn JVM_IsContainerized() -> jboolean {
	todo!()
}

#[jni_call(no_env, no_strict_types)]
pub extern "C" fn JVM_RegisterSignal(_signal: jint, _handler: *mut c_void) -> *mut c_void {
	todo!()
}

#[jni_call(no_env)]
pub extern "C" fn JVM_RaiseSignal(_signal: jint) -> jboolean {
	todo!()
}

#[jni_call(no_env, no_strict_types)]
pub extern "C" fn JVM_FindSignal(_name: *const c_char) -> jint {
	todo!()
}

#[jni_call(no_env, no_strict_types)]
pub extern "C" fn JVM_NativePath(_name: *mut c_char) -> *mut c_char {
	todo!()
}
