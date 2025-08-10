use jni::env::JniEnv;
use jni::objects::JClass;
use jni::sys::{jint, jlong};
use native_macros::jni_call;

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_EPoll_eventSize(_env: JniEnv, _this: JClass) -> jint {
	unimplemented!("sun.nio.ch.EPoll#eventSize");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_EPoll_eventsOffset(_env: JniEnv, _this: JClass) -> jint {
	unimplemented!("sun.nio.ch.EPoll#eventsOffset");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_EPoll_dataOffset(_env: JniEnv, _this: JClass) -> jint {
	unimplemented!("sun.nio.ch.EPoll#dataOffset");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_EPoll_create(_env: JniEnv, _this: JClass) -> jint {
	unimplemented!("sun.nio.ch.EPoll#create");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_EPoll_ctl(
	_env: JniEnv,
	_this: JClass,
	_epfd: jint,
	_opcode: jint,
	_fd: jint,
	_events: jint,
) -> jint {
	unimplemented!("sun.nio.ch.EPoll#ctl");
}

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_EPoll_wait(
	_env: JniEnv,
	_this: JClass,
	_epfd: jint,
	_address: jlong,
	_num_fds: jint,
	_timeout: jint,
) -> jint {
	unimplemented!("sun.nio.ch.EPoll#wait");
}
