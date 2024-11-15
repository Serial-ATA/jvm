use std::fmt::Formatter;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	NoJarMain,
	Jni(jni::error::JniError),
}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::NoJarMain => f.write_str("Unable to find main class in jar manifest"),
			Self::Jni(e) => e.fmt(f),
		}
	}
}

impl From<jni::error::JniError> for Error {
	fn from(e: jni::error::JniError) -> Self {
		Self::Jni(e)
	}
}

impl core::error::Error for Error {}
