#![native_macros::jni_fn_module]

use crate::native::jni::reference_from_jobject_maybe_null;
use crate::objects::instance::object::Object;
use crate::thread::JavaThread;

use jni::env::JniEnv;
use jni::objects::JObject;
use jni::sys::{jint, jlong};
use native_macros::jni_call;

#[jni_call]
pub extern "C" fn JVM_IHashCode(env: JniEnv, handle: JObject) -> jint {
	// Null references can be hashed
	let handle = (unsafe { reference_from_jobject_maybe_null(handle.raw()) });

	// This will only calculate a hash if one isn't already cached in the header
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	handle.hash(thread)
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
