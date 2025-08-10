use jni::env::JniEnv;
use jni::objects::{JClass, JObject};
use jni::sys::{jboolean, jint, jlong};
use native_macros::jni_call;

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_KQueue_keventSize(_env: JniEnv, _this: JClass) -> jint {
	unimplemented!("sun.nio.ch.KQueue#keventSize");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_KQueue_identOffset(_env: JniEnv, _this: JClass) -> jint {
	unimplemented!("sun.nio.ch.KQueue#identOffset");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_KQueue_filterOffset(_env: JniEnv, _this: JClass) -> jint {
	unimplemented!("sun.nio.ch.KQueue#filterOffset");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_KQueue_flagsOffset(_env: JniEnv, _this: JClass) -> jint {
	unimplemented!("sun.nio.ch.KQueue#flagsOffset");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_KQueue_create(_env: JniEnv, _this: JClass) -> jint {
	unimplemented!("sun.nio.ch.KQueue#create");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_KQueue_register(
	_env: JniEnv,
	_this: JClass,
	_kqfd: jint,
	_fd: jint,
	_filter: jint,
	_flags: jint,
) -> jint {
	unimplemented!("sun.nio.ch.KQueue#register");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_KQueue_poll(
	_env: JniEnv,
	_this: JClass,
	_kqfd: jint,
	_address: jlong,
	_nevents: jint,
	_timeout: jint,
) -> jint {
	unimplemented!("sun.nio.ch.KQueue#poll");
}
