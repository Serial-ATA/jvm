use crate::native::JNIEnv;
use crate::reference::Reference;

use ::jni::sys::jlong;

include_generated!("native/jdk/internal/misc/def/CDS.definitions.rs");

pub fn isDumpingClassList0(_env: JNIEnv) -> bool {
	false
}

pub fn isDumpingArchive0(_env: JNIEnv) -> bool {
	false
}

pub fn isSharingEnabled0(_env: JNIEnv) -> bool {
	false
}

pub fn logLambdaFormInvoker(_: JNIEnv, line: Reference) {
	unimplemented!("jdk.internal.misc.CDS#logLambdaFormInvoker")
}

pub fn initializeFromArchive(_: JNIEnv, class: Reference) {
	// TODO
}
pub fn defineArchivedModules(_: JNIEnv, platform_loader: Reference, system_loader: Reference) {
	unimplemented!("jdk.internal.misc.CDS#defineArchivedModules")
}
pub fn getRandomSeedForDumping(_: JNIEnv) -> jlong {
	// TODO: https://github.com/openjdk/jdk/blob/af564e46b006fcd57ec7391cd1438b3b9311b1d6/src/hotspot/share/prims/jvm.cpp#L3696
	0
}

pub fn dumpClassList(_: JNIEnv, list_file_name: Reference) {
	unimplemented!("jdk.internal.misc.CDS#dumpClassList")
}
pub fn dumpDynamicArchive(_: JNIEnv, archive_file_name: Reference) {
	unimplemented!("jdk.internal.misc.CDS#dumpDynamicArchive")
}
