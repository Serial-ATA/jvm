use std::ffi::c_void;

use jni::java_vm::JavaVm;
use jni::sys::{JNI_EVERSION, JavaVM, jint};
use jni::version::JniVersion;

cfg_if::cfg_if! {
	if #[cfg(unix)] {
		mod unix;
		pub use unix::*;
	} else {
		compile_error!("Unsupported platform for libnio");
	}
}

#[unsafe(no_mangle)]
pub extern "system" fn JNI_OnLoad(vm: *mut JavaVM, _reserved: *mut c_void) -> jint {
	let vm = unsafe { JavaVm::from_raw(vm) };

	if vm.get_env(JniVersion::V2).is_err() {
		return JNI_EVERSION;
	}

	JniVersion::V2 as jint
}
