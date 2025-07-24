use crate::classes;
use crate::objects::reference::Reference;

use ::jni::env::JniEnv;
use ::jni::sys::jboolean;

include_generated!("native/java/lang/ref/def/PhantomReference.definitions.rs");

// TODO: Actual implementations of PhantomReference

pub fn refersTo0(
	_: JniEnv,
	this: Reference, // java.lang.ref.PhantomReference
	o: Reference,    // java.lang.Object
) -> jboolean {
	classes::java::lang::r#ref::Reference::referent(this) == o
}

pub fn clear0(_: JniEnv, this: Reference /* java.lang.ref.PhantomReference */) {
	classes::java::lang::r#ref::Reference::set_referent(this, Reference::null());
}
