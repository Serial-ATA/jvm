use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use zip::result::ZipError;

pub type Result<T> = std::result::Result<T, JmodError>;

#[derive(Debug)]
pub enum JmodError {
	MissingMagic,
	BadVersion(u8, u8),
	InvalidSectionName,
	Zip(ZipError),
	Io(std::io::Error),
}

impl Display for JmodError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			JmodError::MissingMagic => write!(f, "JMOD file is missing the magic signature"),
			JmodError::BadVersion(major, minor) => write!(
				f,
				"Found an invalid version (v{major}.{minor}), expected <= {}.{}",
				super::MAJOR_VERSION,
				super::MINOR_VERSION
			),
			JmodError::InvalidSectionName => write!(f, "Input does not map to a JMOD section"),
			JmodError::Zip(zip) => write!(f, "{}", zip),
			JmodError::Io(io) => write!(f, "{}", io),
		}
	}
}

impl Error for JmodError {}

impl From<ZipError> for JmodError {
	fn from(value: ZipError) -> Self {
		JmodError::Zip(value)
	}
}

impl From<std::io::Error> for JmodError {
	fn from(value: std::io::Error) -> Self {
		JmodError::Io(value)
	}
}
