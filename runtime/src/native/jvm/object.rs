#![native_macros::jni_fn_module]

use crate::native::jni::{IntoJni, reference_from_jobject, reference_from_jobject_maybe_null};
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
	// Null references can be hashed
	let handle = (unsafe { reference_from_jobject_maybe_null(handle.raw()) });

	// This will only calculate a hash if one isn't already cached in the header
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	handle.hash(thread)
}

#[jni_call]
pub extern "C" fn JVM_MonitorWait(env: JniEnv, handle: JObject, timeout_millis: jlong) {
	let Some(handle) = (unsafe { reference_from_jobject(handle.raw()) }) else {
		panic!("Attempting to MonitorWait on a null reference");
	};

	let timeout;
	if timeout_millis > 0 {
		timeout = Some(Duration::from_millis(timeout_millis as u64));
	} else {
		timeout = None;
	}

	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	if let Throws::Exception(e) = handle.wait(thread, timeout) {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		e.throw(thread);
	}
}

#[jni_call]
pub extern "C" fn JVM_MonitorNotify(env: JniEnv, handle: JObject) {
	let Some(handle) = (unsafe { reference_from_jobject(handle.raw()) }) else {
		panic!("Attempting to MonitorNotify on a null reference");
	};

	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	if let Throws::Exception(e) = handle.notify(thread) {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		e.throw(thread);
	}
}

#[jni_call]
pub extern "C" fn JVM_MonitorNotifyAll(env: JniEnv, handle: JObject) {
	let Some(handle) = (unsafe { reference_from_jobject(handle.raw()) }) else {
		panic!("Attempting to MonitorNotifyAll on a null reference");
	};

	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	if let Throws::Exception(e) = handle.notify_all(thread) {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		e.throw(thread);
	}
}

#[jni_call]
pub extern "C" fn JVM_Clone(_env: JniEnv, handle: JObject) -> JObject {
	let Some(handle) = (unsafe { reference_from_jobject(handle.raw()) }) else {
		panic!("Attempting to clone a null reference");
	};

	// An array is always cloneable
	{
		if handle.is_primitive_array() {
			let array = handle.extract_primitive_array();
			let cloned = unsafe { CloneableInstance::clone(&array) };
			return Reference::array(cloned).into_jni_safe();
		}

		if handle.is_object_array() {
			let array = handle.extract_object_array();
			let cloned = unsafe { CloneableInstance::clone(&array) };
			return Reference::object_array(cloned).into_jni_safe();
		}
	}

	let instance = handle.extract_class();
	if !instance.class().is_cloneable() {
		throw_with_ret!(
			JObject::null(),
			JavaThread::current(),
			CloneNotSupportedException
		);
	}

	let cloned = unsafe { CloneableInstance::clone(&instance) };
	Reference::class(cloned).into_jni_safe()
}
