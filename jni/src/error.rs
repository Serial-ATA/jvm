use std::error::Error;
use std::fmt::{Display, Formatter};

use jni_sys::{jint, JNI_EDETACHED, JNI_EEXIST, JNI_EINVAL, JNI_ENOMEM, JNI_ERR, JNI_EVERSION};

pub type Result<T> = std::result::Result<T, JniError>;

#[derive(Debug, Clone)]
pub enum JniError {
	ThreadDetached,
	BadVersion,
	OutOfMemory,
	AlreadyExists,
	InvalidArguments,
}

impl JniError {
	pub fn as_jint(&self) -> jint {
		match self {
			JniError::ThreadDetached => JNI_EDETACHED,
			JniError::BadVersion => JNI_EVERSION,
			JniError::OutOfMemory => JNI_ENOMEM,
			JniError::AlreadyExists => JNI_EEXIST,
			JniError::InvalidArguments => JNI_EINVAL,
			_ => JNI_ERR, // Unknown error
		}
	}
}

impl Display for JniError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			JniError::ThreadDetached => write!(f, "Current thread has detached from the VM"),
			JniError::BadVersion => write!(f, "Version error"),
			JniError::OutOfMemory => write!(f, "Out of memory"),
			JniError::AlreadyExists => write!(f, "VM has already been created"),
			JniError::InvalidArguments => write!(f, "Invalid arguments provided"),
		}
	}
}

impl Error for JniError {}
