/// Safer wrapper around `jni_sys::JNIEnv`
pub struct JNIEnv {
	sys_env: *mut jni_sys::JNIEnv,
}

impl JNIEnv {
	pub fn raw(&self) -> *mut jni_sys::JNIEnv {
		self.sys_env
	}
}

impl From<*mut jni_sys::JNIEnv> for JNIEnv {
	fn from(value: *mut jni_sys::JNIEnv) -> Self {
		Self { sys_env: value }
	}
}
