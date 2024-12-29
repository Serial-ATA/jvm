use crate::include_generated;
use crate::native::JniEnv;
use crate::objects::instance::CloneableInstance;
use crate::objects::reference::Reference;
use crate::thread::JavaThread;

use std::ptr::NonNull;

use ::jni::sys::{jint, jlong};
use common::traits::PtrType;

include_generated!("native/java/lang/def/Object.definitions.rs");

pub fn getClass(_: NonNull<JniEnv>, this: Reference /* java.lang.Object */) -> Reference /* java.lang.Class<?> */
{
	Reference::mirror(this.extract_class_mirror())
}

pub fn hashCode(env: NonNull<JniEnv>, this: Reference /* java.lang.Object */) -> jint {
	// Hash already generated, nothing to do
	if let Some(hash) = this.hash() {
		return hash;
	}

	// We need to generate a hash, this will update the object for future calls
	let thread = unsafe { &*JavaThread::for_env(env.as_ptr()) };
	this.generate_hash(thread)
}

// throws CloneNotSupportedException
pub fn clone(_: NonNull<JniEnv>, this: Reference /* java.lang.Object */) -> Reference /* java.lang.Object */
{
	// An array is always cloneable
	if this.is_array() {
		let array = this.extract_array();
		let instance = array.get();
		let cloned = unsafe { instance.clone() };
		return Reference::array(cloned);
	}

	let instance_ref = this.extract_class();
	let instance = instance_ref.get();
	if !instance.class().is_cloneable() {
		// TODO
		panic!("CloneNotSupportedException");
	}

	let cloned = unsafe { instance.clone() };
	Reference::class(cloned)
}

pub fn notify(_: NonNull<JniEnv>, _this: Reference /* java.lang.Object */) {
	unimplemented!("Object#notify")
}

pub fn notifyAll(_: NonNull<JniEnv>, this: Reference /* java.lang.Object */) {
	this.notify_all();
}

pub fn wait0(
	_: NonNull<JniEnv>,
	_this: Reference, // java.lang.Object
	_timeout_millis: jlong,
) {
	unimplemented!("Object#wait0")
}
