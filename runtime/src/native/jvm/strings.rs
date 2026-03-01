#![native_macros::jni_fn_module]

use jni::env::JniEnv;
use jni::objects::JString;
use native_macros::jni_call;
use std::ffi::c_char;

#[jni_call]
pub extern "C" fn JVM_InternString(_env: JniEnv, _string: JString) -> JString {
	todo!()
}

#[jni_call(no_env, no_strict_types)]
pub extern "C" fn JVM_ReleaseUTF(_utf: *const c_char) {
	todo!()
}
