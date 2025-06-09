#![allow(non_snake_case)]

use jni::env::JniEnv;
use jni::objects::JClass;
use jni::sys::jint;
use libc::{AF_INET, AF_INET6, sockaddr, sockaddr_in, sockaddr_in6};
use native_macros::jni_call;
use std::mem;

cfg_if::cfg_if! {
	if #[cfg(unix)] {
		mod unix;
		pub use unix::*;
	} else if #[cfg(windows)] {
		mod windows;
		pub use windows::*;
	} else {
		compile_error!("Unsupported platform for libnio");
	}
}

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
pub extern "system" fn Java_sun_nio_ch_NativeSocketAddress_sizeofSockAddr4(
	_env: JniEnv,
	_this: JClass,
) -> jint {
	size_of::<sockaddr_in>() as _
}

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_NativeSocketAddress_sizeofSockAddr6(
	_env: JniEnv,
	_this: JClass,
) -> jint {
	size_of::<sockaddr_in6>() as _
}

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_NativeSocketAddress_sizeofFamily(
	_env: JniEnv,
	_this: JClass,
) -> jint {
	let sockaddr = unsafe { mem::zeroed::<sockaddr>() };
	size_of_val(&sockaddr.sa_family) as _
}

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_NativeSocketAddress_offsetFamily(
	_env: JniEnv,
	_this: JClass,
) -> jint {
	mem::offset_of!(sockaddr, sa_family) as _
}

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_NativeSocketAddress_offsetSin4Port(
	_env: JniEnv,
	_this: JClass,
) -> jint {
	mem::offset_of!(sockaddr_in, sin_port) as _
}

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_NativeSocketAddress_offsetSin4Addr(
	_env: JniEnv,
	_this: JClass,
) -> jint {
	mem::offset_of!(sockaddr_in, sin_addr) as _
}

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_NativeSocketAddress_offsetSin6Port(
	_env: JniEnv,
	_this: JClass,
) -> jint {
	mem::offset_of!(sockaddr_in6, sin6_port) as _
}

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_NativeSocketAddress_offsetSin6Addr(
	_env: JniEnv,
	_this: JClass,
) -> jint {
	mem::offset_of!(sockaddr_in6, sin6_addr) as _
}

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_NativeSocketAddress_offsetSin6ScopeId(
	_env: JniEnv,
	_this: JClass,
) -> jint {
	mem::offset_of!(sockaddr_in6, sin6_scope_id) as _
}

#[jni_call]
pub extern "system" fn Java_sun_nio_ch_NativeSocketAddress_offsetSin6FlowInfo(
	_env: JniEnv,
	_this: JClass,
) -> jint {
	mem::offset_of!(sockaddr_in6, sin6_flowinfo) as _
}
