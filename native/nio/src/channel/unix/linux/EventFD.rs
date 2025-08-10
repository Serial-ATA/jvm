use jni::env::JniEnv;
use jni::objects::JClass;
use jni::sys::jint;

use native_macros::jni_call;

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_EventFD_eventfd0(_env: JniEnv, _this: JClass) -> jint {
	unimplemented!("sun.nio.ch.EventFD#eventfd0");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_EventFD_set0(
	_env: JniEnv,
	_this: JClass,
	_efd: jint,
) -> jint {
	unimplemented!("sun.nio.ch.EventFD#set0");
}
