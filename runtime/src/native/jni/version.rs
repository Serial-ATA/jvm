use jni::sys::{JNIEnv, jint};
use jni::version::JniVersion;

#[unsafe(no_mangle)]
pub extern "system" fn GetVersion(env: *mut JNIEnv) -> jint {
	JniVersion::LATEST as jint
}
