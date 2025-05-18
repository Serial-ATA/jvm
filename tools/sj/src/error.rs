use std::fmt::Formatter;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	MissingClasspath(String),
	MissingJar(String),
	NonUtf8Path,

	NoJarMain,
	Jni(jni::error::JniError),
}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::MissingClasspath(variant) => {
				write!(f, "Error: {variant} requires class path specification")
			},
			Self::MissingJar(variant) => {
				write!(f, "Error: {variant} requires jar file specification")
			},
			Self::NonUtf8Path => {
				write!(f, "Error: not a valid UTF-8 path")
			},

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
