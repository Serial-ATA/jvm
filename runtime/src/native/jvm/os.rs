#![native_macros::jni_fn_module]

use std::ffi::{CStr, c_char, c_void};

use common::unicode;
use jni::sys::{jboolean, jint, jlong};
use native_macros::jni_call;

#[jni_call(no_env)]
pub extern "C" fn JVM_TotalMemory() -> jlong {
	todo!()
}

#[jni_call(no_env)]
pub extern "C" fn JVM_FreeMemory() -> jlong {
	todo!()
}

#[jni_call(no_env)]
pub extern "C" fn JVM_MaxMemory() -> jlong {
	todo!()
}

#[jni_call(no_env)]
pub extern "C" fn JVM_ActiveProcessorCount() -> jint {
	todo!()
}

#[jni_call(no_env)]
pub extern "C" fn JVM_IsUseContainerSupport() -> jboolean {
	todo!()
}

#[jni_call(no_env)]
pub extern "C" fn JVM_IsContainerized() -> jboolean {
	todo!()
}

#[jni_call(no_env, no_strict_types)]
pub extern "C" fn JVM_RegisterSignal(signal: jint, handler: *mut c_void) -> *mut c_void {
	const FAILED: isize = -1;
	const USER_HANDLER: usize = 2;

	let signal = platform::Signal::from(signal);

	if !signal.registration_allowed() {
		return FAILED as usize as *mut c_void;
	}

	let handler = match handler as usize {
		USER_HANDLER => platform::SignalHandler::user_handler(),
		other => unsafe { platform::SignalHandler::from_raw(other) },
	};

	let old = unsafe { signal.install(handler) };
	let Some(old) = old else {
		// Registration failed
		return FAILED as usize as *mut c_void;
	};

	if old == platform::SignalHandler::user_handler() {
		return USER_HANDLER as *mut c_void;
	}

	old.raw()
}

#[jni_call(no_env)]
pub extern "C" fn JVM_RaiseSignal(_signal: jint) -> jboolean {
	todo!()
}

#[jni_call(no_env, no_strict_types)]
pub extern "C" fn JVM_FindSignal(name: *const c_char) -> jint {
	let name_c = unsafe { CStr::from_ptr(name) };
	let Ok(name_utf8) = unicode::decode(name_c.to_bytes()) else {
		return -1;
	};

	match platform::Signal::from_name(name_utf8) {
		Some(signal) => signal.value(),
		None => -1,
	}
}

#[jni_call(no_env, no_strict_types)]
pub extern "C" fn JVM_NativePath(_name: *mut c_char) -> *mut c_char {
	todo!()
}
