use crate::objects::class::Class;
use crate::objects::reference::Reference;

use ::jni::env::JniEnv;
use ::jni::sys::{jboolean, jlong};

include_generated!("native/jdk/internal/loader/def/NativeLibraries.definitions.rs");

pub fn load(
	_env: JniEnv,
	_class: &'static Class,
	_impl_: Reference, // jdk.internal.loader.NativeLibraries$NativeLibraryImpl
	_name: Reference,  // java.lang.String
	_is_builtin: jboolean,
	_throw_exception_if_fail: jboolean,
) -> jboolean {
	unimplemented!("jdk.internal.loader.NativeLibraries#load")
}

pub fn unload(
	_env: JniEnv,
	_class: &'static Class,
	_name: Reference, // java.lang.String
	_is_builtin: jboolean,
	_handle: jlong,
) {
	unimplemented!("jdk.internal.loader.NativeLibraries#unload")
}

pub fn findBuiltinLib(
	_env: JniEnv,
	_class: &'static Class,
	_name: Reference, // java.lang.String
) -> Reference /* java.lang.String */
{
	unimplemented!("jdk.internal.loader.NativeLibraries#findBuiltinLib")
}
