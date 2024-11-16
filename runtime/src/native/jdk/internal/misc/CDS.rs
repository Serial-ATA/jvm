use crate::reference::Reference;

use std::ptr::NonNull;

use ::jni::env::JniEnv;
use ::jni::sys::{jint, jlong};

include_generated!("native/jdk/internal/misc/def/CDS.definitions.rs");

pub fn getCDSConfigStatus(_: NonNull<JniEnv>) -> jint {
	// TODO: Bitfield of:
	//     private static final int IS_DUMPING_ARCHIVE              = 1 << 0;
	//     private static final int IS_DUMPING_STATIC_ARCHIVE       = 1 << 1;
	//     private static final int IS_LOGGING_LAMBDA_FORM_INVOKERS = 1 << 2;
	//     private static final int IS_USING_ARCHIVE                = 1 << 3;
	0
}

pub fn logLambdaFormInvoker(_: NonNull<JniEnv>, _line: Reference) {
	unimplemented!("jdk.internal.misc.CDS#logLambdaFormInvoker")
}

pub fn initializeFromArchive(_: NonNull<JniEnv>, _class: Reference) {
	// TODO
}

pub fn defineArchivedModules(
	_: NonNull<JniEnv>,
	_platform_loader: Reference,
	_system_loader: Reference,
) {
	unimplemented!("jdk.internal.misc.CDS#defineArchivedModules")
}

pub fn getRandomSeedForDumping(_: NonNull<JniEnv>) -> jlong {
	// TODO: https://github.com/openjdk/jdk/blob/af564e46b006fcd57ec7391cd1438b3b9311b1d6/src/hotspot/share/prims/jvm.cpp#L3696
	0
}

pub fn dumpClassList(_: NonNull<JniEnv>, _list_file_name: Reference) {
	unimplemented!("jdk.internal.misc.CDS#dumpClassList")
}
pub fn dumpDynamicArchive(_: NonNull<JniEnv>, _archive_file_name: Reference) {
	unimplemented!("jdk.internal.misc.CDS#dumpDynamicArchive")
}
