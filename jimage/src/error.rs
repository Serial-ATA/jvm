use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	InvalidMagic,
	InvalidTableSize,
	BadIndexSize,

	Common(common::error::CommonError),
	Io(std::io::Error),
	Utf8(core::str::Utf8Error),
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::InvalidMagic => write!(f, "File has an invalid magic signature!"),
			Self::InvalidTableSize => write!(f, "Encountered invalid table size!"),
			Self::BadIndexSize => write!(
				f,
				"The index does not match the size provided in the header"
			),

			Self::Common(err) => write!(f, "{}", err),
			Self::Io(err) => write!(f, "{}", err),
			Self::Utf8(err) => write!(f, "{}", err),
		}
	}
}

impl core::error::Error for Error {}

impl From<common::error::CommonError> for Error {
	fn from(value: common::error::CommonError) -> Self {
		Self::Common(value)
	}
}

impl From<std::io::Error> for Error {
	fn from(value: std::io::Error) -> Self {
		Self::Io(value)
	}
}

impl From<core::str::Utf8Error> for Error {
	fn from(value: core::str::Utf8Error) -> Self {
		Self::Utf8(value)
	}
}
