use crate::native::JniEnv;
use crate::reference::Reference;

use std::ptr::NonNull;

use jni::sys::jint;

include_generated!("native/java/lang/def/ClassLoader.registerNatives.rs");
include_generated!("native/java/lang/def/ClassLoader.definitions.rs");

pub fn defineClass1(
	_env: NonNull<JniEnv>,
	_loader: Reference, // java.lang.ClassLoader
	_name: Reference,   // java.lang.String
	_b: Reference,      // byte[],
	_off: jint,
	_len: jint,
	_pd: Reference,     // ProtectionDomain
	_source: Reference, // java.lang.String
) -> Reference // java.lang.Class
{
	unimplemented!("java.lang.Class#defineClass1")
}

pub fn defineClass2(
	_env: NonNull<JniEnv>,
	_loader: Reference, // java.lang.ClassLoader
	_name: Reference,   // java.lang.String
	_b: Reference,      // java.nio.ByteBuffer,
	_off: jint,
	_len: jint,
	_pd: Reference,     // ProtectionDomain
	_source: Reference, // java.lang.String
) -> Reference // java.lang.Class
{
	unimplemented!("java.lang.Class#defineClass2")
}

pub fn defineClass0(
	_env: NonNull<JniEnv>,
	_loader: Reference, // java.lang.ClassLoader
	_lookup: Reference, // java.lang.Class
	_name: Reference,   // java.lang.String
	_b: Reference,      // byte[],
	_off: jint,
	_len: jint,
	_pd: Reference, // ProtectionDomain
	_initialize: bool,
	_flags: jint,
	_source: Reference, // java.lang.String
) -> Reference // java.lang.Class
{
	unimplemented!("java.lang.Class#defineClass0")
}

pub fn findBootstrapClass(
	_env: NonNull<JniEnv>,
	_name: Reference, // java.lang.String
) -> Reference // java.lang.Class
{
	unimplemented!("java.lang.Class#findBootstrapClass")
}

pub fn findLoadedClass0(
	_env: NonNull<JniEnv>,
	_this: Reference, // java.lang.Class
	_name: Reference, // java.lang.String
) -> Reference // java.lang.Class
{
	unimplemented!("java.lang.Class#findLoadedClass0")
}

pub fn retrieveDirectives(_env: NonNull<JniEnv>) -> Reference // AssertionStatusDirectives
{
	unimplemented!("java.lang.Class#retrieveDirectives")
}
