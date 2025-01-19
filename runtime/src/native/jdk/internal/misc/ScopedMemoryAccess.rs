use crate::objects::reference::Reference;

use std::ptr::NonNull;

use ::jni::env::JniEnv;

include_generated!("native/jdk/internal/misc/def/ScopedMemoryAccess.definitions.rs");
include_generated!("native/jdk/internal/misc/def/ScopedMemoryAccess.registerNatives.rs");

pub fn closeScope0(
	_env: JniEnv,
	_this: Reference,    // jdk.internal.misc.ScopedMemoryAccess
	_session: Reference, // jdk.internal.foreign.MemorySessionImpl
	_error: Reference,   // jdk.internal.misc.ScopedMemoryAccess.ScopedAccessError
) -> Reference {
	unimplemented!("jdk.internal.misc.ScopedMemoryAccess#closeScope0")
}
