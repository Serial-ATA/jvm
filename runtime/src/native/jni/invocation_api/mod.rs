//! The Invocation API allows software vendors to load the Java VM into an arbitrary native application.
//!
//! Vendors can deliver Java-enabled applications without having to link with the Java VM source code.

use core::ffi::c_void;
use jni::{jint, jsize, JavaVM};

pub mod library;

pub extern "system" fn DestroyJavaVM(vm: *mut JavaVM) -> jint {
	unimplemented!("jni::DestroyJavaVM")
}

pub extern "system" fn AttachCurrentThread(
	vm: *mut JavaVM,
	penv: *mut *mut c_void,
	args: *mut c_void,
) -> jint {
	unimplemented!("jni::AttachCurrentThread")
}

pub extern "system" fn DetachCurrentThread(vm: *mut JavaVM) -> jint {
	unimplemented!("jni::DetachCurrentThread")
}

pub extern "system" fn GetEnv(vm: *mut JavaVM, penv: *mut *mut c_void, version: jint) -> jint {
	unimplemented!("jni::GetEnv")
}

pub extern "system" fn AttachCurrentThreadAsDaemon(
	vm: *mut JavaVM,
	penv: *mut *mut c_void,
	args: *mut c_void,
) -> jint {
	unimplemented!("jni::AttachCurrentThreadAsDaemon")
}

pub extern "system" fn JNI_GetDefaultJavaVMInitArgs(args: *mut c_void) -> jint {
	unimplemented!("jni::JNI_GetDefaultJavaVMInitArgs")
}

pub extern "system" fn JNI_CreateJavaVM(
	pvm: *mut *mut JavaVM,
	penv: *mut *mut c_void,
	args: *mut c_void,
) -> jint {
	unimplemented!("jni::JNI_CreateJavaVM")
}

pub extern "system" fn JNI_GetCreatedJavaVMs(
	vmBuf: *mut *mut JavaVM,
	bufLen: jsize,
	nVMs: *mut jsize,
) -> jint {
	unimplemented!("jni::JNI_GetCreatedJavaVMs")
}
