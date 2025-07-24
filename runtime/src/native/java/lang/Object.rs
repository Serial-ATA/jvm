use crate::objects::instance::CloneableInstance;
use crate::objects::instance::object::Object;
use crate::objects::reference::Reference;
use crate::thread::JavaThread;
use crate::thread::exceptions::{Throws, throw_and_return_null};

use std::time::Duration;

use ::jni::env::JniEnv;
use ::jni::sys::{jint, jlong};

include_generated!("native/java/lang/def/Object.definitions.rs");

pub fn getClass(_: JniEnv, this: Reference /* java.lang.Object */) -> Reference /* java.lang.Class<?> */
{
	Reference::mirror(this.extract_class_mirror())
}

pub fn hashCode(env: JniEnv, this: Reference /* java.lang.Object */) -> jint {
	// This will only calculate a hash if one isn't already cached in the header
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	this.hash(thread)
}

// throws CloneNotSupportedException
pub fn clone(_: JniEnv, this: Reference /* java.lang.Object */) -> Reference /* java.lang.Object */
{
	// An array is always cloneable
	{
		if this.is_primitive_array() {
			let array = this.extract_primitive_array();
			let cloned = unsafe { CloneableInstance::clone(&array) };
			return Reference::array(cloned);
		}

		if this.is_object_array() {
			let array = this.extract_object_array();
			let cloned = unsafe { CloneableInstance::clone(&array) };
			return Reference::object_array(cloned);
		}
	}

	let instance = this.extract_class();
	if !instance.class().is_cloneable() {
		throw_and_return_null!(JavaThread::current(), CloneNotSupportedException);
	}

	let cloned = unsafe { CloneableInstance::clone(&instance) };
	Reference::class(cloned)
}

pub fn notify(env: JniEnv, this: Reference /* java.lang.Object */) {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	if let Throws::Exception(e) = this.notify(thread) {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		e.throw(thread);
	}
}

pub fn notifyAll(env: JniEnv, this: Reference /* java.lang.Object */) {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	if let Throws::Exception(e) = this.notify_all(thread) {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		e.throw(thread);
	}
}

pub fn wait0(
	env: JniEnv,
	this: Reference, // java.lang.Object
	timeout_millis: jlong,
) {
	let timeout;
	if timeout_millis > 0 {
		timeout = Some(Duration::from_millis(timeout_millis as u64));
	} else {
		timeout = None;
	}

	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	if let Throws::Exception(e) = this.wait(thread, timeout) {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		e.throw(thread);
	}
}
