#![native_macros::jni_fn_module]

use jni::env::JniEnv;
use jni::objects::JString;
use native_macros::jni_call;

#[jni_call]
pub extern "C" fn JVM_InternString(_env: JniEnv, _string: JString) -> JString {
	todo!()
}
