use jni::sys::{JNIEnv, JNINativeMethod, jclass, jint};

#[unsafe(no_mangle)]
pub extern "system" fn RegisterNatives(
	env: *mut JNIEnv,
	clazz: jclass,
	methods: *const JNINativeMethod,
	nMethods: jint,
) -> jint {
	unimplemented!("jni::RegisterNatives")
}

#[unsafe(no_mangle)]
pub extern "system" fn UnregisterNatives(env: *mut JNIEnv, clazz: jclass) -> jint {
	unimplemented!("jni::UnregisterNatives");
}
