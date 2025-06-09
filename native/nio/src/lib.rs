#![feature(custom_inner_attributes)]
#![feature(proc_macro_hygiene)]

pub mod channel;
pub mod fs;

use std::ffi::c_void;

use jni::java_vm::JavaVm;
use jni::sys::{JNI_EVERSION, JavaVM, jint};
use jni::version::JniVersion;

#[unsafe(no_mangle)]
pub extern "system" fn JNI_OnLoad(vm: *mut JavaVM, _reserved: *mut c_void) -> jint {
	let vm = unsafe { JavaVm::from_raw(vm) };

	if vm.get_env(JniVersion::V2).is_err() {
		return JNI_EVERSION;
	}

	JniVersion::V2 as jint
}
