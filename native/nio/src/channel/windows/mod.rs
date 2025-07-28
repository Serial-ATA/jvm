use std::mem;

use jni::env::JniEnv;
use jni::objects::JClass;
use jni::sys::jint;
use native_macros::jni_call;

pub use windows::Win32::Networking::WinSock::{
	AF_INET, AF_INET6, SOCKADDR as sockaddr, SOCKADDR_IN as sockaddr_in,
	SOCKADDR_IN6 as sockaddr_in6,
};

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_NativeSocketAddress_AFINET(
	_env: JniEnv,
	_this: JClass,
) -> jint {
	AF_INET.0 as jint
}

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_NativeSocketAddress_AFINET6(
	_env: JniEnv,
	_this: JClass,
) -> jint {
	AF_INET6.0 as jint
}

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_NativeSocketAddress_offsetSin6ScopeId(
	_env: JniEnv,
	_this: JClass,
) -> jint {
	mem::offset_of!(sockaddr_in6, Anonymous) as _
}
