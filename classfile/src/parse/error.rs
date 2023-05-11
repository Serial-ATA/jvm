use crate::parse::attributes::Location;
use crate::AttributeTag;

use std::error::Error;
use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, ClassFileParseError>;

#[derive(Debug)]
pub enum ClassFileParseError {
	MissingMagic,
	InvalidLocation(AttributeTag, Location),
	Io(std::io::Error),
}

impl Display for ClassFileParseError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::MissingMagic => write!(f, "File has no magic signature!"),
			Self::InvalidLocation(tag, location) => write!(
				f,
				"Invalid attribute location: {:?} found in {:?}",
				tag, location
			),

			Self::Io(err) => write!(f, "{}", err),
		}
	}
}

impl Error for ClassFileParseError {}

impl From<std::io::Error> for ClassFileParseError {
	fn from(value: std::io::Error) -> Self {
		Self::Io(value)
	}
}
