use ::jni::env::JniEnv;
use ::jni::objects::{JObject, JObjectArray};
use native_macros::jni_call;

#[jni_call]
extern "C" fn invoke(
	_env: JniEnv,
	_this: JObject,
	_args: JObjectArray, // Object...
) -> JObject /* java.lang.Object */ {
	unreachable!(
		"java.lang.invoke.MethodHandle#invoke called through native method. should have been \
		 intercepted"
	);
}

#[jni_call]
extern "C" fn invokeExact(
	_env: JniEnv,
	_this: JObject,
	_args: JObjectArray, // Object...
) -> JObject /* java.lang.Object */ {
	unreachable!(
		"java.lang.invoke.MethodHandle#invokeExact called through native method. should have been \
		 intercepted"
	);
}
