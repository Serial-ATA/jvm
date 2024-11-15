use crate::reference::Reference;

use std::ptr::NonNull;

use ::jni::env::JniEnv;
use common::int_types::{s4, s8};

include_generated!("native/java/lang/def/Runtime.definitions.rs");

pub fn availableProcessors(
	_: NonNull<JniEnv>,
	_this: Reference, // java.lang.Runtime
) -> s4 {
	num_cpus::get() as s4
}
pub fn freeMemory(_: NonNull<JniEnv>, _this: Reference /* java.lang.Runtime */) -> s8 {
	unimplemented!("java.lang.Runtime#freeMemory")
}
pub fn totalMemory(_: NonNull<JniEnv>, _this: Reference /* java.lang.Runtime */) -> s8 {
	unimplemented!("java.lang.Runtime#totalMemory")
}
pub fn maxMemory(_: NonNull<JniEnv>, _this: Reference /* java.lang.Runtime */) -> s8 {
	// TODO: Xmx
	s8::MAX
}
pub fn gc(_: NonNull<JniEnv>, _this: Reference /* java.lang.Runtime */) {
	unimplemented!("java.lang.Runtime#gc")
}
