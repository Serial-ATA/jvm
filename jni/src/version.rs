use jni_sys::jint;

/// The version of the Java Native Interface
///
/// Up to V8, the version corresponds to `1.x`. After that point, the version number is simply `x`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JniVersion {
	/// 1.1
	V1,
	/// 1.2
	V2,
	/// 1.4
	V4,
	/// 1.6
	V6,
	/// 1.8
	V8,
	V9,
	V10,
	V19,
	V20,
	V21,
	V24,
}

impl JniVersion {
	/// The latest JNI version
	///
	/// NOTE: This is used as the default in [`VmInitArgs`](crate::java_vm::VmInitArgs)
	pub const LATEST: Self = JniVersion::V24;

	/// Get a `JniVersion` from an integer
	///
	/// NOTE: The valid version numbers are available in [`jni_sys`].
	pub fn from_raw(raw: jint) -> Option<Self> {
		match raw {
			jni_sys::JNI_VERSION_1_1 => Some(JniVersion::V1),
			jni_sys::JNI_VERSION_1_2 => Some(JniVersion::V2),
			jni_sys::JNI_VERSION_1_4 => Some(JniVersion::V4),
			jni_sys::JNI_VERSION_1_6 => Some(JniVersion::V6),
			jni_sys::JNI_VERSION_1_8 => Some(JniVersion::V8),
			jni_sys::JNI_VERSION_9 => Some(JniVersion::V9),
			jni_sys::JNI_VERSION_10 => Some(JniVersion::V10),
			jni_sys::JNI_VERSION_19 => Some(JniVersion::V19),
			jni_sys::JNI_VERSION_20 => Some(JniVersion::V20),
			jni_sys::JNI_VERSION_21 => Some(JniVersion::V21),
			jni_sys::JNI_VERSION_24 => Some(JniVersion::V24),
			_ => None,
		}
	}
}

impl From<JniVersion> for jint {
	fn from(version: JniVersion) -> jint {
		match version {
			JniVersion::V1 => jni_sys::JNI_VERSION_1_1,
			JniVersion::V2 => jni_sys::JNI_VERSION_1_2,
			JniVersion::V4 => jni_sys::JNI_VERSION_1_4,
			JniVersion::V6 => jni_sys::JNI_VERSION_1_6,
			JniVersion::V8 => jni_sys::JNI_VERSION_1_8,
			JniVersion::V9 => jni_sys::JNI_VERSION_9,
			JniVersion::V10 => jni_sys::JNI_VERSION_10,
			JniVersion::V19 => jni_sys::JNI_VERSION_19,
			JniVersion::V20 => jni_sys::JNI_VERSION_20,
			JniVersion::V21 => jni_sys::JNI_VERSION_21,
			JniVersion::V24 => jni_sys::JNI_VERSION_24,
		}
	}
}
