use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
	Jmod(jmod::error::JmodError),
	Io(std::io::Error),
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::Jmod(e) => f.write_fmt(format_args!("{e}")),
			Error::Io(e) => f.write_fmt(format_args!("{e}")),
		}
	}
}

impl core::error::Error for Error {}

impl From<jmod::error::JmodError> for Error {
	fn from(value: jmod::error::JmodError) -> Self {
		Error::Jmod(value)
	}
}

impl From<std::io::Error> for Error {
	fn from(value: std::io::Error) -> Self {
		Error::Io(value)
	}
}
