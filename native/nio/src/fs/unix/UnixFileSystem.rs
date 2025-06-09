#![native_macros::jni_fn_module]

use jni::env::JniEnv;
use jni::objects::JClass;
use jni::sys::{jint, jlong};
use native_macros::jni_call;

#[jni_call]
pub extern "system" fn Java_sun_nio_fs_UnixFileSystem_bufferedCopy0(
	_env: JniEnv,
	_this: JClass,
	_dest: jint,
	_src: jint,
	_address: jlong,
	_transfer_size: jint,
	_cancel_address: jlong,
) -> jint {
	unimplemented!("sun.nio.fs.UnixFileSystem#bufferedCopy0");
}
