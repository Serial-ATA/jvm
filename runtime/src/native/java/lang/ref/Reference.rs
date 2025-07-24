use crate::classes;
use crate::objects::class::ClassPtr;
use crate::objects::reference::Reference;

use jni::env::JniEnv;
use jni::sys::jboolean;

include_generated!("native/java/lang/ref/def/Reference.definitions.rs");

pub fn getAndClearReferencePendingList(_: JniEnv, _class: ClassPtr) -> Reference /* java.lang.ref.Reference */
{
	unimplemented!("java.lang.ref.Reference#getAndClearReferencePendingList")
}

pub fn hasReferencePendingList(_: JniEnv, _class: ClassPtr) -> jboolean {
	unimplemented!("java.lang.ref.Reference#hasReferencePendingList")
}

pub fn waitForReferencePendingList(_: JniEnv, _class: ClassPtr) {
	unimplemented!("java.lang.ref.Reference#waitForReferencePendingList")
}

pub fn refersTo0(
	_: JniEnv,
	this: Reference, // java.lang.ref.Reference
	o: Reference,    // java.lang.Object
) -> jboolean {
	classes::java::lang::r#ref::Reference::referent(this) == o
}

pub fn clear0(_: JniEnv, _this: Reference /* java.lang.ref.Reference */) {
	unimplemented!("java.lang.ref.Reference#clear0")
}
