use std::error::Error;
use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, JImageError>;

#[derive(Debug)]
pub enum JImageError {
	InvalidMagic,

	Common(common::error::CommonError),
	Io(std::io::Error),
	Utf8(core::str::Utf8Error),
}

impl Display for JImageError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::InvalidMagic => write!(f, "File has an invalid magic signature!"),

			Self::Common(err) => write!(f, "{}", err),
			Self::Io(err) => write!(f, "{}", err),
			Self::Utf8(err) => write!(f, "{}", err),
		}
	}
}

impl Error for JImageError {}

impl From<common::error::CommonError> for JImageError {
	fn from(value: common::error::CommonError) -> Self {
		Self::Common(value)
	}
}

impl From<std::io::Error> for JImageError {
	fn from(value: std::io::Error) -> Self {
		Self::Io(value)
	}
}

impl From<core::str::Utf8Error> for JImageError {
	fn from(value: core::str::Utf8Error) -> Self {
		Self::Utf8(value)
	}
}
