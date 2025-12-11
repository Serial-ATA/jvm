use crate::native::java::lang::String::StringInterner;
use crate::native::jni::reference_from_jobject;
use crate::objects::reference::Reference;
use crate::symbols::sym;
use crate::thread::JavaThread;
use crate::thread::exceptions::Throws;
use crate::{classes, java_call};

use core::ffi::c_char;
use std::slice;

use ::jni::sys::{JNI_ERR, JNI_OK, JNIEnv, jboolean, jclass, jint, jthrowable};
use common::unicode;
use instructions::Operand;
use libc::strlen;

#[unsafe(no_mangle)]
pub extern "system" fn Throw(env: *mut JNIEnv, obj: jthrowable) -> jint {
	let thread = JavaThread::current();
	assert_eq!(thread.env().raw(), env);

	let Some(throwable) = (unsafe { reference_from_jobject(obj) }) else {
		return JNI_ERR;
	};

	if !throwable.is_instance_of(crate::globals::classes::java_lang_Throwable()) {
		return JNI_ERR;
	}

	thread.set_pending_exception(throwable);

	JNI_OK
}

#[unsafe(no_mangle)]
pub extern "system" fn ThrowNew(env: *mut JNIEnv, clazz: jclass, msg: *const c_char) -> jint {
	let thread = JavaThread::current();
	assert_eq!(thread.env().raw(), env);

	let Some(mirror) = (unsafe { reference_from_jobject(clazz) }) else {
		return JNI_ERR;
	};

	let mut message = None;
	if !msg.is_null() {
		// SAFETY: It's entirely up to the caller to pass in a valid string
		let len = unsafe { strlen(msg) };

		// SAFETY: c_char is always 8 bits
		let utf = unsafe { slice::from_raw_parts(msg.cast::<u8>(), len) };

		if let Ok(utf_8) = unicode::decode(utf) {
			message = Some(utf_8);
		};
	}

	let throwable_class = mirror.extract_target_class();

	let constructor;
	if message.is_none() {
		constructor = throwable_class
			.resolve_method(sym!(object_initializer_name), sym!(void_method_signature));
	} else {
		constructor = throwable_class
			.resolve_method(sym!(object_initializer_name), sym!(String_void_signature));
	}

	match constructor {
		Throws::Ok(constructor) => match message {
			Some(message) => {
				let string = StringInterner::intern(message);
				java_call!(
					thread,
					constructor,
					Operand::Reference(Reference::class(string))
				);
			},
			None => {
				java_call!(thread, constructor);
			},
		},
		Throws::Exception(e) => {
			e.throw(thread);
			return JNI_ERR;
		},
	}

	JNI_OK
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
