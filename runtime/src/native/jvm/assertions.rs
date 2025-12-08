#![native_macros::jni_fn_module]

use jni::env::JniEnv;
use jni::objects::{JClass, JObject};
use jni::sys::jboolean;
use native_macros::jni_call;

#[jni_call]
pub extern "C" fn JVM_DesiredAssertionStatus(
	_env: JniEnv,
	_unused: JClass,
	_class: JClass,
) -> jboolean {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_AssertionStatusDirectives(_env: JniEnv, _unused: JClass) -> JObject {
	todo!()
}
