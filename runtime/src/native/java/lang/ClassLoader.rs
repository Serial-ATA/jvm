use crate::native::JNIEnv;
use crate::reference::Reference;
use common::int_types::s4;
use jni::sys::jint;

include_generated!("native/java/lang/def/ClassLoader.registerNatives.rs");
include_generated!("native/java/lang/def/ClassLoader.definitions.rs");

pub fn defineClass1(
	_env: JNIEnv,
	loader: Reference, // java.lang.ClassLoader
	name: Reference,   // java.lang.String
	b: Reference,      // byte[],
	off: jint,
	len: jint,
	pd: Reference,     // ProtectionDomain
	source: Reference, // java.lang.String
) -> Reference // java.lang.Class
{
	unimplemented!("java.lang.Class#defineClass1")
}

pub fn defineClass2(
	_env: JNIEnv,
	loader: Reference, // java.lang.ClassLoader
	name: Reference,   // java.lang.String
	b: Reference,      // java.nio.ByteBuffer,
	off: jint,
	len: jint,
	pd: Reference,     // ProtectionDomain
	source: Reference, // java.lang.String
) -> Reference // java.lang.Class
{
	unimplemented!("java.lang.Class#defineClass2")
}

pub fn defineClass0(
	_env: JNIEnv,
	loader: Reference, // java.lang.ClassLoader
	lookup: Reference, // java.lang.Class
	name: Reference,   // java.lang.String
	b: Reference,      // byte[],
	off: jint,
	len: jint,
	pd: Reference, // ProtectionDomain
	initialize: bool,
	flags: jint,
	source: Reference, // java.lang.String
) -> Reference // java.lang.Class
{
	unimplemented!("java.lang.Class#defineClass0")
}

pub fn findBootstrapClass(
	_env: JNIEnv,
	name: Reference, // java.lang.String
) -> Reference // java.lang.Class
{
	unimplemented!("java.lang.Class#findBootstrapClass")
}

pub fn findLoadedClass0(
	_env: JNIEnv,
	name: Reference, // java.lang.String
) -> Reference // java.lang.Class
{
	unimplemented!("java.lang.Class#findLoadedClass0")
}

pub fn retrieveDirectives(_env: JNIEnv) -> Reference // AssertionStatusDirectives
{
	unimplemented!("java.lang.Class#retrieveDirectives")
}
