#![native_macros::jni_fn_module]

use jni::env::JniEnv;
use jni::objects::JObject;
use jni::sys::jboolean;
use native_macros::jni_call;

#[jni_call]
pub extern "C" fn JVM_GetAndClearReferencePendingList(_env: JniEnv) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_HasReferencePendingList(_env: JniEnv) -> jboolean {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_WaitForReferencePendingList(_env: JniEnv) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_ReferenceGet(_env: JniEnv, _reference: JObject) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_ReferenceRefersTo(
	_env: JniEnv,
	_reference: JObject,
	_obj: JObject,
) -> jboolean {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_ReferenceClear(_env: JniEnv, _reference: JObject) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_PhantomReferenceRefersTo(
	_env: JniEnv,
	_reference: JObject,
	_obj: JObject,
) -> jboolean {
	todo!()
}
