use crate::classes;
use crate::objects::class::Class;
use crate::objects::instance::Instance;
use crate::objects::reference::Reference;

use jni::env::JniEnv;
use jni::sys::jboolean;

include_generated!("native/java/lang/ref/def/Reference.definitions.rs");

pub fn getAndClearReferencePendingList(_: JniEnv, _class: &'static Class) -> Reference /* java.lang.ref.Reference */
{
	unimplemented!("java.lang.ref.Reference#getAndClearReferencePendingList")
}

pub fn hasReferencePendingList(_: JniEnv, _class: &'static Class) -> jboolean {
	unimplemented!("java.lang.ref.Reference#hasReferencePendingList")
}

pub fn waitForReferencePendingList(_: JniEnv, _class: &'static Class) {
	unimplemented!("java.lang.ref.Reference#waitForReferencePendingList")
}

pub fn refersTo0(
	_: JniEnv,
	this: Reference, // java.lang.ref.Reference
	o: Reference,    // java.lang.Object
) -> jboolean {
	let referent_field_offset = classes::java_lang_ref_Reference::referent_field_offset();
	let referent = this.get_field_value0(referent_field_offset);

	referent.expect_reference() == o
}

pub fn clear0(_: JniEnv, _this: Reference /* java.lang.ref.Reference */) {
	unimplemented!("java.lang.ref.Reference#clear0")
}
