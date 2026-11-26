#![native_macros::jni_fn_module]

use std::sync::OnceLock;

use ::jni::env::JniEnv;
use ::jni::objects::{JClass, JObject, JObjectArray};
use ::jni::sys::{jboolean, jint, jlong};
use ::jni::version::JniVersion;
use native_macros::jni_call;

#[jni_call(no_env)]
pub extern "C" fn JVM_BeforeHalt() {
	todo!()
}

#[jni_call(no_env)]
pub extern "C" fn JVM_Halt(_code: jint) {
	todo!()
}

#[jni_call(no_env)]
pub extern "C" fn JVM_GC() {
	todo!()
}

#[jni_call(no_env)]
pub extern "C" fn JVM_MaxObjectInspectionAge() -> jlong {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_RegisterContinuationMethods(_env: JniEnv, _class: JClass) {
	todo!()
}

#[jni_call(no_env)]
pub extern "C" fn JVM_IsSupportedJNIVersion(version: jint) -> jboolean {
	JniVersion::from_raw(version).is_some()
}

// Modified in `src/options/mod.rs`
pub(crate) static PREVIEW_ENABLED: OnceLock<bool> = OnceLock::new();
#[jni_call(no_env)]
pub extern "C" fn JVM_IsPreviewEnabled() -> jboolean {
	*PREVIEW_ENABLED.get_or_init(|| false)
}

#[jni_call(no_env)]
pub extern "C" fn JVM_IsContinuationsSupported() -> jboolean {
	todo!()
}

#[jni_call(no_env)]
pub extern "C" fn JVM_IsForeignLinkerSupported() -> jboolean {
	todo!()
}

#[jni_call(no_env)]
pub extern "C" fn JVM_IsStaticallyLinked() -> jboolean {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_LatestUserDefinedLoader(_env: JniEnv) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_InitAgentProperties(_env: JniEnv, _properties: JObject) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetVmArguments(_env: JniEnv) -> JObjectArray {
	todo!()
}
