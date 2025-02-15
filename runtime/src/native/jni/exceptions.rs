use crate::java_call;
use crate::native::java::lang::String::rust_string_from_java_string;
use crate::symbols::sym;
use crate::thread::JavaThread;

use core::ffi::c_char;

use classfile::accessflags::MethodAccessFlags;
use common::traits::PtrType;
use instructions::Operand;
use jni::sys::{jboolean, jclass, jint, jthrowable, JNIEnv};

#[no_mangle]
pub extern "system" fn Throw(env: *mut JNIEnv, obj: jthrowable) -> jint {
	unimplemented!("jni::Throw");
}

#[no_mangle]
pub extern "system" fn ThrowNew(env: *mut JNIEnv, clazz: jclass, msg: *const c_char) -> jint {
	unimplemented!("jni::ThrowNew");
}

#[no_mangle]
pub extern "system" fn ExceptionOccurred(env: *mut JNIEnv) -> jthrowable {
	unimplemented!("jni::ExceptionOccurred");
}

#[no_mangle]
pub extern "system" fn ExceptionDescribe(env: *mut JNIEnv) {
	let thread = JavaThread::current();
	assert_eq!(thread.env().raw(), env);

	let Some(exception) = thread.take_pending_exception() else {
		return;
	};

	assert!(exception.is_instance_of(crate::globals::classes::java_lang_Throwable()));

	eprint!("Exception in thread \"{}\" ", thread.name());

	let exception_class = exception.extract_target_class();

	let print_stack_trace = exception_class
		.vtable()
		.find(
			sym!(printStackTrace_name),
			sym!(void_method_signature),
			MethodAccessFlags::NONE,
		)
		.expect("java/lang/Throwable#printStackTrace should exist");

	java_call!(thread, print_stack_trace, Operand::Reference(exception));

	// Mirroring the behavior of Hotspot, which discards any exceptions thrown in printStackTrace
	let _ = thread.take_pending_exception();
}

#[no_mangle]
pub extern "system" fn ExceptionClear(env: *mut JNIEnv) {
	unimplemented!("jni::ExceptionClear");
}

#[no_mangle]
pub extern "system" fn FatalError(env: *mut JNIEnv, msg: *const c_char) -> ! {
	unimplemented!("jni::FatalError");
}

#[no_mangle]
pub extern "system" fn ExceptionCheck(env: *mut JNIEnv) -> jboolean {
	let thread = JavaThread::current();
	assert_eq!(thread.env().raw(), env);

	thread.has_pending_exception()
}
