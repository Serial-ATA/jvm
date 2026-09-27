#![native_macros::jni_fn_module]

use crate::native::jni::references::JObjectExt;
use crate::objects::instance::CloneableInstance;
use crate::objects::instance::object::Object;
use crate::objects::reference::Reference;
use crate::thread::JavaThread;
use crate::thread::exceptions::{Throws, throw_with_ret};

use std::time::Duration;

use ::jni::env::JniEnv;
use ::jni::objects::JObject;
use ::jni::sys::{jint, jlong};
use native_macros::jni_call;

#[jni_call]
pub extern "C" fn JVM_IHashCode(env: JniEnv, handle: JObject) -> jint {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	assert_eq!(thread.env(), env);

	// Null references can be hashed
	let handle = unsafe { handle.to_reference_maybe_null() };

	// This will only calculate a hash if one isn't already cached in the header
	handle.hash(thread)
}

#[jni_call]
pub extern "C" fn JVM_MonitorWait(env: JniEnv, handle: JObject, timeout_millis: jlong) {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	assert_eq!(thread.env(), env);

	let Some(handle) = (unsafe { handle.to_reference() }) else {
		panic!("Attempting to MonitorWait on a null reference");
	};

	let timeout;
	if timeout_millis > 0 {
		timeout = Some(Duration::from_millis(timeout_millis as u64));
	} else {
		timeout = None;
	}

	if let Throws::Exception(e) = handle.wait(thread, timeout) {
		e.throw(thread);
	}
}

#[jni_call]
pub extern "C" fn JVM_MonitorNotify(env: JniEnv, handle: JObject) {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	assert_eq!(thread.env(), env);

	let Some(handle) = (unsafe { handle.to_reference() }) else {
		panic!("Attempting to MonitorNotify on a null reference");
	};

	if let Throws::Exception(e) = handle.notify(thread) {
		e.throw(thread);
	}
}

#[jni_call]
pub extern "C" fn JVM_MonitorNotifyAll(env: JniEnv, handle: JObject) {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	assert_eq!(thread.env(), env);

	let Some(handle) = (unsafe { handle.to_reference() }) else {
		panic!("Attempting to MonitorNotifyAll on a null reference");
	};

	if let Throws::Exception(e) = handle.notify_all(thread) {
		e.throw(thread);
	}
}

#[jni_call]
pub extern "C" fn JVM_Clone(env: JniEnv, handle: JObject) -> JObject {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	assert_eq!(thread.env(), env);

	let Some(handle) = (unsafe { handle.to_reference() }) else {
		panic!("Attempting to clone a null reference");
	};

	// An array is always cloneable
	{
		if handle.is_primitive_array() {
			let array = handle.extract_primitive_array();
			let cloned = unsafe { CloneableInstance::clone(&array) };
			return thread.jni_refs().allocate_wrapped(Reference::array(cloned));
		}

		if handle.is_object_array() {
			let array = handle.extract_object_array();
			let cloned = unsafe { CloneableInstance::clone(&array) };
			return thread
				.jni_refs()
				.allocate_wrapped(Reference::object_array(cloned));
		}
	}

	let instance = handle.extract_class();
	if !instance.class().is_cloneable() {
		throw_with_ret!(JObject::null(), thread, CloneNotSupportedException);
	}

	let cloned = unsafe { CloneableInstance::clone(&instance) };
	thread.jni_refs().allocate_wrapped(Reference::class(cloned))
}
