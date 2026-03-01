#![native_macros::jni_fn_module]

use std::ffi::{c_char, c_void};

use jni::env::JniEnv;
use jni::objects::{JObject, JObjectArray, JString};
use jni::sys::jboolean;
use native_macros::jni_call;

#[jni_call(no_env, no_strict_types)]
pub extern "C" fn JVM_LoadZipLibrary() -> *mut c_void {
	todo!()
}

#[jni_call(no_env, no_strict_types)]
pub extern "C" fn JVM_LoadLibrary(_name: *const c_char, _throw_exception: jboolean) -> *mut c_void {
	todo!()
}

#[jni_call(no_env, no_strict_types)]
pub extern "C" fn JVM_UnloadLibrary(_handle: *mut c_void) {
	todo!()
}

#[jni_call(no_env, no_strict_types)]
pub extern "C" fn JVM_FindLibraryEntry(_handle: *mut c_void, _name: *const c_char) -> *mut c_void {
	todo!()
}
