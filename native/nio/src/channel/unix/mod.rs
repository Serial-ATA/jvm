use std::mem;

use jni::env::JniEnv;
use jni::objects::JClass;
use jni::sys::jint;
use native_macros::jni_call;

cfg_select! {
	target_os = "linux" => {
		mod linux;
		pub use linux::*;
	}
	target_os = "macos" => {
		mod macos;
		pub use macos::*;
	}
}

pub use libc::{AF_INET, AF_INET6, sockaddr, sockaddr_in, sockaddr_in6};

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_NativeSocketAddress_AFINET(
	_env: JniEnv,
	_this: JClass,
) -> jint {
	AF_INET
}

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_NativeSocketAddress_AFINET6(
	_env: JniEnv,
	_this: JClass,
) -> jint {
	AF_INET6
}

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_NativeSocketAddress_offsetSin6ScopeId(
	_env: JniEnv,
	_this: JClass,
) -> jint {
	mem::offset_of!(sockaddr_in6, sin6_scope_id) as _
}
