use crate::attribute::AttributeTag;
use crate::parse::attributes::Location;

use std::error::Error;
use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, ClassFileParseError>;

#[derive(Debug)]
pub enum ClassFileParseError {
	InvalidMagic,
	InvalidLocation(AttributeTag, Location),

	InvalidMethodDescriptor(&'static str),

	Common(common::error::CommonError),
	Io(std::io::Error),
}

impl Display for ClassFileParseError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::InvalidMagic => write!(f, "File has an invalid magic signature!"),
			Self::InvalidLocation(tag, location) => write!(
				f,
				"Invalid attribute location: {:?} found in {:?}",
				tag, location
			),

			Self::InvalidMethodDescriptor(message) => {
				write!(f, "Encountered invalid method descriptor: {}", message)
			},

			Self::Common(err) => write!(f, "{}", err),
			Self::Io(err) => write!(f, "{}", err),
		}
	}
}

impl Error for ClassFileParseError {}

impl From<common::error::CommonError> for ClassFileParseError {
	fn from(value: common::error::CommonError) -> Self {
		Self::Common(value)
	}
}

impl From<std::io::Error> for ClassFileParseError {
	fn from(value: std::io::Error) -> Self {
		Self::Io(value)
	}
}
