use crate::include_generated;
use crate::objects::class::Class;
use crate::objects::reference::Reference;

use std::ptr::NonNull;

use ::jni::env::JniEnv;
use common::int_types::s8;

include_generated!("native/jdk/internal/misc/def/VM.definitions.rs");

pub fn latestUserDefinedLoader0(_env: NonNull<JniEnv>, _class: &'static Class) -> Reference /* java.lang.ClassLoader */
{
	unimplemented!("jdk.internal.misc.VM#latestUserDefinedLoader0")
}
pub fn getuid(_env: NonNull<JniEnv>, _class: &'static Class) -> s8 {
	unimplemented!("jdk.internal.misc.VM#getuid")
}
pub fn geteuid(_env: NonNull<JniEnv>, _class: &'static Class) -> s8 {
	unimplemented!("jdk.internal.misc.VM#geteuid")
}
pub fn getgid(_env: NonNull<JniEnv>, _class: &'static Class) -> s8 {
	unimplemented!("jdk.internal.misc.VM#getgid")
}
pub fn getegid(_env: NonNull<JniEnv>, _class: &'static Class) -> s8 {
	unimplemented!("jdk.internal.misc.VM#getegid")
}
pub fn getNanoTimeAdjustment(_env: NonNull<JniEnv>, _class: &'static Class, _offset: s8) -> s8 {
	unimplemented!("jdk.internal.misc.VM#getNanoTimeAdjustment")
}
pub fn getRuntimeArguments(_env: NonNull<JniEnv>, _class: &'static Class) -> Reference /* String[] */
{
	unimplemented!("jdk.internal.misc.VM#getRuntimeArguments")
}
pub fn initialize(_env: NonNull<JniEnv>, _class: &'static Class) {
	// https://github.com/openjdk/jdk/blob/7abe26935ab4356de54acee93390a0d8be1ea289/src/java.base/share/native/libjava/VM.c#L44
}
