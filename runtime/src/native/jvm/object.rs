#![native_macros::jni_fn_module]

use jni::env::JniEnv;
use jni::objects::{JClass, JObject, JString};
use jni::sys::{jboolean, jint, jlong};
use native_macros::jni_call;

#[jni_call]
pub extern "C" fn JVM_IHashCode(_env: JniEnv, _handle: JObject) -> jint {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_MonitorWait(_env: JniEnv, _handle: JObject, _ms: jlong) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_MonitorNotify(_env: JniEnv, _handle: JObject) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_MonitorNotifyAll(_env: JniEnv, _handle: JObject) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_Clone(_env: JniEnv, _handle: JObject) -> JObject {
	todo!()
}
