use crate::include_generated;
use crate::native::JNIEnv;
use crate::reference::Reference;

use ::jni::sys::{jint, jlong};
use common::traits::PtrType;

include_generated!("native/java/lang/def/Object.definitions.rs");

pub fn getClass(_: JNIEnv, this: Reference /* Object */) -> Reference /* Class<?> */ {
	Reference::Mirror(this.extract_class_mirror())
}

pub fn hashCode(_: JNIEnv, this: Reference /* Object */) -> jint {
	let hash_code = match this {
		Reference::Class(class) => class.as_raw() as jint,
		Reference::Array(array) => array.as_raw() as jint,
		Reference::Mirror(mirror) => mirror.as_raw() as jint,
		Reference::Null => 0,
	};

	hash_code
}

// throws CloneNotSupportedException
pub fn clone(_: JNIEnv, this: Reference /* Object */) -> Reference /* Object */ {
	unimplemented!("Object#clone")
}

pub fn notify(_: JNIEnv, this: Reference /* Object */) {
	unimplemented!("Object#notify")
}

pub fn notifyAll(_: JNIEnv, this: Reference /* Object */) {
	unimplemented!("Object#notifyAll")
}

pub fn wait0(_: JNIEnv, this: Reference /* Object */, timeout_millis: jlong) {
	unimplemented!("Object#wait0")
}
