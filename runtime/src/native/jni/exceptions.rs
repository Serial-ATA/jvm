use crate::classes;
use crate::thread::JavaThread;

use core::ffi::c_char;

use jni::sys::{JNIEnv, jboolean, jclass, jint, jthrowable};

#[unsafe(no_mangle)]
pub extern "system" fn Throw(env: *mut JNIEnv, obj: jthrowable) -> jint {
	unimplemented!("jni::Throw");
}

#[unsafe(no_mangle)]
pub extern "system" fn ThrowNew(env: *mut JNIEnv, clazz: jclass, msg: *const c_char) -> jint {
	unimplemented!("jni::ThrowNew");
}

#[unsafe(no_mangle)]
pub extern "system" fn ExceptionOccurred(env: *mut JNIEnv) -> jthrowable {
	unimplemented!("jni::ExceptionOccurred");
}

#[unsafe(no_mangle)]
pub extern "system" fn ExceptionDescribe(env: *mut JNIEnv) {
	let thread = JavaThread::current();
	assert_eq!(thread.env().raw(), env);

	let Some(exception) = thread.take_pending_exception() else {
		return;
	};

	eprint!("Exception in thread \"{}\" ", thread.name());
	classes::java::lang::Throwable::print_stack_trace(exception, thread);

	// Mirroring the behavior of Hotspot, which discards any exceptions thrown in printStackTrace
	let _ = thread.take_pending_exception();
}

#[unsafe(no_mangle)]
pub extern "system" fn ExceptionClear(env: *mut JNIEnv) {
	unimplemented!("jni::ExceptionClear");
}

#[unsafe(no_mangle)]
pub extern "system" fn FatalError(env: *mut JNIEnv, msg: *const c_char) -> ! {
	unimplemented!("jni::FatalError");
}

#[unsafe(no_mangle)]
pub extern "system" fn ExceptionCheck(env: *mut JNIEnv) -> jboolean {
	let thread = JavaThread::current();
	assert_eq!(thread.env().raw(), env);

	thread.has_pending_exception()
}
