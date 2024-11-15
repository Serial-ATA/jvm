use crate::class_instance::{ArrayInstancePtr, ClassInstance, ClassInstancePtr};
use crate::include_generated;
use crate::native::JniEnv;
use crate::reference::Reference;

use std::ptr::NonNull;

use ::jni::sys::{jint, jlong};
use common::traits::PtrType;

include_generated!("native/java/lang/def/Object.definitions.rs");

pub fn getClass(_: NonNull<JniEnv>, this: Reference /* java.lang.Object */) -> Reference /* java.lang.Class<?> */
{
	Reference::mirror(this.extract_class_mirror())
}

pub fn hashCode(_: NonNull<JniEnv>, this: Reference /* java.lang.Object */) -> jint {
	let ptr = this.ptr();
	if ptr.is_null() {
		return 0;
	}

	ptr as jint
}

// throws CloneNotSupportedException
pub fn clone(_: NonNull<JniEnv>, this: Reference /* java.lang.Object */) -> Reference /* java.lang.Object */
{
	// An array is always cloneable
	if this.is_array() {
		let array = this.extract_array();
		return Reference::array(ArrayInstancePtr::new(array.get().clone()));
	}

	let instance_ref = this.extract_class();
	let instance = instance_ref.get();
	if !instance.class.is_cloneable() {
		// TODO
		panic!("CloneNotSupportedException");
	}

	Reference::class(ClassInstancePtr::new(instance.clone()))
}

pub fn notify(_: NonNull<JniEnv>, _this: Reference /* java.lang.Object */) {
	unimplemented!("Object#notify")
}

pub fn notifyAll(_: NonNull<JniEnv>, _this: Reference /* java.lang.Object */) {
	unimplemented!("Object#notifyAll")
}

pub fn wait0(
	_: NonNull<JniEnv>,
	_this: Reference, // java.lang.Object
	_timeout_millis: jlong,
) {
	unimplemented!("Object#wait0")
}
