use crate::objects::class::Class;
use crate::objects::reference::Reference;

use jni::env::JniEnv;
use jni::sys::{jboolean, jint, jlong};

include_generated!("native/java/lang/invoke/def/MethodHandleNatives.registerNatives.rs");
include_generated!("native/java/lang/invoke/def/MethodHandleNatives.definitions.rs");

// -- MemberName support --

pub fn init(
	_env: JniEnv,
	_class: &'static Class,
	self_: Reference, // java.lang.invoke.MemberName
	ref_: Reference,  // java.lang.Object
) {
	unimplemented!("java.lang.invoke.MethodHandleNatives#init");
}

pub fn expand(
	_env: JniEnv,
	_class: &'static Class,
	self_: Reference, // java.lang.invoke.MemberName
) {
	unimplemented!("java.lang.invoke.MethodHandleNatives#expand");
}

// throws LinkageError, ClassNotFoundException
pub fn resolve(
	_env: JniEnv,
	_class: &'static Class,
	self_: Reference,  // java.lang.invoke.MemberName
	caller: Reference, // java.lang.Class<?>
	lookup_mode: jint,
	speculative_resolve: jboolean,
) -> Reference /* java.lang.invoke.MemberName */ {
	unimplemented!("java.lang.invoke.MethodHandleNatives#resolve");
}

// -- Field layout queries parallel to jdk.internal.misc.Unsafe --

pub fn objectFieldOffset(
	_env: JniEnv,
	_class: &'static Class,
	self_: Reference, // java.lang.invoke.MemberName
) -> jlong {
	unimplemented!("java.lang.invoke.MethodHandleNatives#objectFieldOffset");
}

pub fn staticFieldOffset(
	_env: JniEnv,
	_class: &'static Class,
	self_: Reference, // java.lang.invoke.MemberName
) -> jlong {
	unimplemented!("java.lang.invoke.MethodHandleNatives#staticFieldOffset");
}

pub fn staticFieldBase(
	_env: JniEnv,
	_class: &'static Class,
	self_: Reference, // java.lang.invoke.MemberName
) -> Reference /* java.lang.Object */ {
	unimplemented!("java.lang.invoke.MethodHandleNatives#staticFieldBase");
}

pub fn getMemberVMInfo(
	_env: JniEnv,
	_class: &'static Class,
	self_: Reference, // java.lang.invoke.MemberName
) -> Reference /* java.lang.Object */ {
	unimplemented!("java.lang.invoke.MethodHandleNatives#getMemberVMInfo");
}

// -- CallSite support --

pub fn setCallSiteTargetNormal(
	_env: JniEnv,
	_class: &'static Class,
	site: Reference,   // java.lang.invoke.CallSite
	target: Reference, // java.lang.invoke.MethodHandle
) {
	unimplemented!("java.lang.invoke.MethodHandleNatives#setCallSiteTargetNormal");
}

pub fn setCallSiteTargetVolatile(
	_env: JniEnv,
	_class: &'static Class,
	site: Reference,   // java.lang.invoke.CallSite
	target: Reference, // java.lang.invoke.MethodHandle
) {
	unimplemented!("java.lang.invoke.MethodHandleNatives#setCallSiteTargetVolatile");
}

pub fn copyOutBootstrapArguments(
	_env: JniEnv,
	_class: &'static Class,
	caller: Reference,     // java.lang.Class<?>
	index_info: Reference, // int[]
	start: jint,
	end: jint,
	buf: Reference, // java.lang.Object[]
	pos: jint,
	resolve: jboolean,
	if_not_available: Reference, // java.lang.Object
) {
	unimplemented!("java.lang.invoke.MethodHandleNatives#copyOutBootstrapArguments");
}

pub fn clearCallSiteContext(
	_env: JniEnv,
	_class: &'static Class,
	context: Reference, // java.lang.invoke.CallSiteContext
) {
	unimplemented!("java.lang.invoke.MethodHandleNatives#clearCallSiteContext");
}

pub fn getNamedCon(
	_env: JniEnv,
	_class: &'static Class,
	which: jint,
	name: Reference, // java.lang.Object[]
) -> jint {
	unimplemented!("java.lang.invoke.MethodHandleNatives#getNamedCon");
}
