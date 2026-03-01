#![native_macros::jni_fn_module]

use jni::env::JniEnv;
use jni::objects::JClass;
use jni::sys::jint;
use native_macros::jni_call;
use std::ffi::c_int;

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetFieldIxModifiers(_env: JniEnv, _cb: JClass, _index: c_int) -> jint {
	todo!()
}
