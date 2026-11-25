#![native_macros::jni_fn_module]

use crate::native::jni::references::JObjectExt;

use jni::env::JniEnv;
use jni::objects::{JClass, JObject};
use jni::sys::jboolean;
use native_macros::jni_call;

// TODO: https://github.com/openjdk/jdk/blob/19373b2ff0cd795afa262c17dcb3388fd6a5be59/src/hotspot/share/classfile/javaAssertions.cpp#L195
#[jni_call]
pub extern "C" fn JVM_DesiredAssertionStatus(
	_env: JniEnv,
	_unused: JClass,
	class: JClass,
) -> jboolean {
	let class = unsafe { class.to_reference() }.expect("invalid class reference");

	let mirror = class.extract_mirror();
	let _name = &mirror.target_class().name();

	false
}

#[jni_call]
pub extern "C" fn JVM_AssertionStatusDirectives(_env: JniEnv, _unused: JClass) -> JObject {
	todo!()
}
