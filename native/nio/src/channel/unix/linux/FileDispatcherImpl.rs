use jni::env::JniEnv;
use jni::objects::{JClass, JObject};
use jni::sys::{jboolean, jint, jlong};
use native_macros::jni_call;

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_FileDispatcherImpl_init0(
	_env: JniEnv,
	_this: JClass,
	_efd: jint,
) {
	unimplemented!("sun.nio.ch.FileDispatcherImpl#init0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_FileDispatcherImpl_transferFrom0(
	_env: JniEnv,
	_this: JObject,
	_src_fdo: JObject,
	_dst_fdo: JObject,
	_position: jlong,
	_count: jlong,
	_append: jboolean,
) -> jlong {
	unimplemented!("sun.nio.ch.FileDispatcherImpl#transferFrom0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_FileDispatcherImpl_transferTo0(
	_env: JniEnv,
	_this: JObject,
	_src_fdo: JObject,
	_position: jlong,
	_count: jlong,
	_dst_fdo: JObject,
	_append: jboolean,
) -> jlong {
	unimplemented!("sun.nio.ch.FileDispatcherImpl#transferTo0");
}
