use crate::classes;
use crate::objects::instance::Instance;
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
	let referent_field_offset = classes::java::lang::r#ref::Reference::referent_field_offset();
	let referent = this.get_field_value0(referent_field_offset);

	referent.expect_reference() == o
}

pub fn clear0(_: JniEnv, this: Reference /* java.lang.ref.PhantomReference */) {
	classes::java::lang::r#ref::Reference::set_referent(this, Reference::null());
}
