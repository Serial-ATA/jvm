use crate::objects::class::ClassPtr;
use crate::objects::reference::Reference;

use ::jni::env::JniEnv;
use ::jni::sys::{jint, jlong};

include_generated!("native/jdk/internal/misc/def/CDS.definitions.rs");

pub fn getCDSConfigStatus(_: JniEnv, _class: ClassPtr) -> jint {
	// TODO: Bitfield of:
	//     private static final int IS_DUMPING_ARCHIVE              = 1 << 0;
	//     private static final int IS_DUMPING_STATIC_ARCHIVE       = 1 << 1;
	//     private static final int IS_LOGGING_LAMBDA_FORM_INVOKERS = 1 << 2;
	//     private static final int IS_USING_ARCHIVE                = 1 << 3;
	0
}

pub fn logLambdaFormInvoker(_: JniEnv, _class: ClassPtr, _line: Reference) {
	unimplemented!("jdk.internal.misc.CDS#logLambdaFormInvoker")
}

pub fn initializeFromArchive(_: JniEnv, _this_class: ClassPtr, _class: Reference) {
	// TODO
}

pub fn defineArchivedModules(
	_: JniEnv,
	_class: ClassPtr,
	_platform_loader: Reference,
	_system_loader: Reference,
) {
	unimplemented!("jdk.internal.misc.CDS#defineArchivedModules")
}

pub fn getRandomSeedForDumping(_: JniEnv, _class: ClassPtr) -> jlong {
	// TODO: https://github.com/openjdk/jdk/blob/af564e46b006fcd57ec7391cd1438b3b9311b1d6/src/hotspot/share/prims/jvm.cpp#L3696
	0
}

pub fn dumpClassList(_: JniEnv, _class: ClassPtr, _list_file_name: Reference) {
	unimplemented!("jdk.internal.misc.CDS#dumpClassList")
}
pub fn dumpDynamicArchive(_: JniEnv, _class: ClassPtr, _archive_file_name: Reference) {
	unimplemented!("jdk.internal.misc.CDS#dumpDynamicArchive")
}
