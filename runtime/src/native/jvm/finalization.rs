#![native_macros::jni_fn_module]

use jni::env::JniEnv;
use jni::objects::JObject;
use jni::sys::jboolean;
use native_macros::jni_call;

#[jni_call]
pub extern "C" fn JVM_ReportFinalizationComplete(_env: JniEnv, _finalizee: JObject) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_IsFinalizationEnabled(_env: JniEnv) -> jboolean {
	todo!()
}
