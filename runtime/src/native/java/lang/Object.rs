use crate::objects::instance::CloneableInstance;
use crate::objects::reference::Reference;
use crate::thread::JavaThread;

use crate::thread::exceptions::throw_and_return_null;
use ::jni::env::JniEnv;
use ::jni::sys::{jint, jlong};
use common::traits::PtrType;

include_generated!("native/java/lang/def/Object.definitions.rs");

pub fn getClass(_: JniEnv, this: Reference /* java.lang.Object */) -> Reference /* java.lang.Class<?> */
{
	Reference::mirror(this.extract_class_mirror())
}

pub fn hashCode(env: JniEnv, this: Reference /* java.lang.Object */) -> jint {
	// Hash already generated, nothing to do
	if let Some(hash) = this.hash() {
		return hash;
	}

	// We need to generate a hash, this will update the object for future calls
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	this.generate_hash(thread)
}

// throws CloneNotSupportedException
pub fn clone(_: JniEnv, this: Reference /* java.lang.Object */) -> Reference /* java.lang.Object */
{
	// An array is always cloneable
	if this.is_array() {
		let array = this.extract_array();
		let instance = array.get();
		let cloned = unsafe { CloneableInstance::clone(instance) };
		return Reference::array(cloned);
	}

	let instance_ref = this.extract_class();
	let instance = instance_ref.get();
	if !instance.class().is_cloneable() {
		throw_and_return_null!(JavaThread::current(), CloneNotSupportedException);
	}

	let cloned = unsafe { CloneableInstance::clone(instance) };
	Reference::class(cloned)
}

pub fn notify(_: JniEnv, _this: Reference /* java.lang.Object */) {
	unimplemented!("Object#notify")
}

pub fn notifyAll(_: JniEnv, this: Reference /* java.lang.Object */) {
	this.notify_all();
}

pub fn wait0(
	_: JniEnv,
	_this: Reference, // java.lang.Object
	_timeout_millis: jlong,
) {
	unimplemented!("Object#wait0")
}
