use jni::{jclass, jint, JNIEnv, JNINativeMethod};

extern "system" fn RegisterNatives(
	env: *mut JNIEnv,
	clazz: jclass,
	methods: *const JNINativeMethod,
	nMethods: jint,
) -> jint {
	unimplemented!("jni::RegisterNatives")
}

pub extern "system" fn UnregisterNatives(env: *mut JNIEnv, clazz: jclass) -> jint {
	unimplemented!("jni::UnregisterNatives");
}
