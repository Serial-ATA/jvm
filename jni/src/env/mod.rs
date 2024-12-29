mod array;
mod class;
mod exceptions;
mod field;
mod method;
mod monitor;
mod nio;
mod object;
mod references;
mod reflection;
mod register;
mod string;
mod version;
mod vm;
mod weak;

/// Safer wrapper around `jni_sys::JNIEnv`
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct JniEnv(jni_sys::JNIEnv);

impl JniEnv {
	pub fn raw(&self) -> jni_sys::JNIEnv {
		self.0
	}

	pub unsafe fn from_raw(env: jni_sys::JNIEnv) -> Self {
		Self(env)
	}

	unsafe fn as_native_interface(&self) -> *const jni_sys::JNINativeInterface_ {
		self.0 as _
	}
}
