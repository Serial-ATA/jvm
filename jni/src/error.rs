use std::error::Error;
use std::fmt::{Display, Formatter};

use jni_sys::{JNI_EDETACHED, JNI_EEXIST, JNI_EINVAL, JNI_ENOMEM, JNI_ERR, JNI_EVERSION, jint};

pub type Result<T> = std::result::Result<T, JniError>;

#[derive(Debug)]
pub enum JniError {
	// Generic JNI errors
	ThreadDetached,
	BadVersion,
	OutOfMemory,
	AlreadyExists,
	InvalidArguments,
	Unknown,

	// Our specific errors
	JavaVm(crate::java_vm::Error),
	ExceptionThrown,
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

	pub fn from_jint(value: jint) -> Option<Self> {
		match value {
			JNI_EDETACHED => Some(JniError::ThreadDetached),
			JNI_EVERSION => Some(JniError::BadVersion),
			JNI_ENOMEM => Some(JniError::OutOfMemory),
			JNI_EEXIST => Some(JniError::AlreadyExists),
			JNI_EINVAL => Some(JniError::InvalidArguments),
			JNI_ERR => Some(JniError::Unknown),
			_ => None,
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
			JniError::Unknown => write!(f, "An error occurred"),

			// Our specific errors
			JniError::JavaVm(e) => write!(f, "Java VM error: {}", e),
			JniError::ExceptionThrown => write!(f, "An exception was thrown"),
		}
	}
}

impl From<crate::java_vm::Error> for JniError {
	fn from(e: crate::java_vm::Error) -> Self {
		JniError::JavaVm(e)
	}
}

impl Error for JniError {}
