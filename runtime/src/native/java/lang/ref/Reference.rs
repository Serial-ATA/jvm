use crate::objects::reference::Reference;

use std::ptr::NonNull;

use jni::env::JniEnv;
use jni::sys::jboolean;

include_generated!("native/java/lang/ref/def/Reference.definitions.rs");

pub fn getAndClearReferencePendingList(_: NonNull<JniEnv>) -> Reference /* java.lang.ref.Reference */
{
	unimplemented!("java.lang.ref.Reference#getAndClearReferencePendingList")
}

pub fn hasReferencePendingList(_: NonNull<JniEnv>) -> jboolean {
	unimplemented!("java.lang.ref.Reference#hasReferencePendingList")
}

pub fn waitForReferencePendingList(_: NonNull<JniEnv>) {
	unimplemented!("java.lang.ref.Reference#waitForReferencePendingList")
}

pub fn refersTo0(
	_: NonNull<JniEnv>,
	_this: Reference, // java.lang.ref.Reference
	_o: Reference,    // java.lang.Object
) -> jboolean {
	unimplemented!("java.lang.ref.Reference#refersTo0")
}

pub fn clear0(_: NonNull<JniEnv>, _this: Reference /* java.lang.ref.Reference */) {
	unimplemented!("java.lang.ref.Reference#clear0")
}
