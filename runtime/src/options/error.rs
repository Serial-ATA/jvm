use crate::options::logging::LogParseError;

use std::fmt::{Display, Formatter};
use std::str::Utf8Error;

pub enum OptionsError {
	UnrecognizedOption(String),
	BadCstr(Utf8Error),
	/// Failed to parse a `-Xlog` option
	Logging(LogParseError),
}

impl Display for OptionsError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::UnrecognizedOption(opt) => write!(f, "Unrecognized VM init option: {opt}"),
			Self::BadCstr(err) => write!(f, "Encountered a bad C string: {err}"),
			Self::Logging(err) => err.fmt(f),
		}
	}
}

impl From<Utf8Error> for OptionsError {
	fn from(value: Utf8Error) -> Self {
		Self::BadCstr(value)
	}
}

impl From<LogParseError> for OptionsError {
	fn from(value: LogParseError) -> Self {
		Self::Logging(value)
	}
}
