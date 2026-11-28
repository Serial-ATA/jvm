use ::jni::env::{JniEnv, JniNativeMethod};
use ::jni::objects::{JClass, JObject, JString};
use ::jni::sys::{jint, jlong};
use native_macros::jni_call;

#[jni_call]
pub extern "C" fn attach0(_env: JniEnv, _perf: JObject, _vmid: jint) -> JObject {
	unimplemented!("jdk.internal.misc.Perf#attach0");
}

#[jni_call]
pub extern "C" fn detach(_env: JniEnv, _perf: JObject, _buffer: JObject) {
	unimplemented!("jdk.internal.misc.Perf#detach");
}

#[jni_call]
pub extern "C" fn createLong(
	_env: JniEnv,
	_perf: JObject,
	_name: JString,
	_variability: jint,
	_units: jint,
	_value: jlong,
) -> JObject {
	unimplemented!("jdk.internal.misc.Perf#createLong");
}

#[jni_call]
pub extern "C" fn createByteArray(
	_env: JniEnv,
	_perf: JObject,
	_name: JString,
	_variability: jint,
	_units: jint,
	_value: jlong,
	_max_length: jint,
) -> JObject {
	unimplemented!("jdk.internal.misc.Perf#createByteArray");
}

#[jni_call]
pub extern "C" fn highResCounter(_env: JniEnv, _perf: JObject) -> jlong {
	unimplemented!("jdk.internal.misc.Perf#highResCounter");
}

#[jni_call]
pub extern "C" fn highResFrequency(_env: JniEnv, _perf: JObject) -> jlong {
	unimplemented!("jdk.internal.misc.Perf#highResFrequency");
}

#[jni_call]
#[unsafe(export_name = "Java_jdk_internal_perf_Perf_registerNatives")]
pub extern "C" fn registerNatives(env: JniEnv, class: JClass) {
	#[allow(trivial_casts)]
    #[rustfmt::skip]
	let methods = [
        JniNativeMethod::from("attach0",          "(I)Ljava/nio/ByteBuffer;",                       attach0 as extern "C" fn(_, _, _) -> JObject),
        JniNativeMethod::from("detach",           "(Ljava/nio/ByteBuffer;)V",                       detach as extern "C" fn(_, _, _)),
        JniNativeMethod::from("createLong",       "(Ljava/lang/String;IIJ)Ljava/nio/ByteBuffer;",   createLong as extern "C" fn(_, _, _, _, _, _) -> JObject),
        JniNativeMethod::from("createByteArray",  "(Ljava/lang/String;II[BI)Ljava/nio/ByteBuffer;", createByteArray as extern "C" fn(_, _, _, _, _, _, _) -> JObject),
        JniNativeMethod::from("highResCounter",   "()J",                                            highResCounter as extern "C" fn(_, _) -> jlong),
        JniNativeMethod::from("highResFrequency", "()J",                                            highResFrequency as extern "C" fn(_, _) -> jlong),
	];

	env.register_natives(class, &methods)
		.expect("should register")
}
