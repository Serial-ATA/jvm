use ::jni::env::{JniEnv, JniNativeMethod};
use ::jni::objects::{JClass, JObject};
use native_macros::jni_call;

#[jni_call]
pub extern "C" fn closeScope0(
	_env: JniEnv,
	_receiver: JObject,
	_session: JObject,
	_error: JObject,
) {
	unimplemented!("jdk.internal.misc.ScopedMemoryAccess#closeScope0");
}

#[jni_call]
#[unsafe(export_name = "Java_jdk_internal_misc_ScopedMemoryAccess_registerNatives")]
pub extern "C" fn registerNatives(env: JniEnv, class: JClass) {
	#[allow(trivial_casts)]
	let methods = [JniNativeMethod::from(
		"closeScope0",
		"(Ljdk/internal/foreign/MemorySessionImpl;Ljdk/internal/misc/\
		 ScopedMemoryAccess$ScopedAccessError;)V",
		closeScope0 as extern "C" fn(_, _, _, _),
	)];

	env.register_natives(class, &methods)
		.expect("should register")
}
