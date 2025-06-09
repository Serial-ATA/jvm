#![native_macros::jni_fn_module]

use jni::env::JniEnv;
use jni::objects::{JClass, JString};
use jni::sys::jlong;
use native_macros::jni_call;

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_RegistryFileTypeDetector_queryStringValue(
	_env: JniEnv,
	_this: JClass,
	_key_address: jlong,
	_name_address: jlong,
) -> JString {
	unimplemented!("sun.nio.fs.RegistryFileTypeDetector#queryStringValue")
}
