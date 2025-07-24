use std::fmt::{Display, Formatter};
use std::str::Utf8Error;

pub enum OptionsError {
	UnrecognizedOption(String),
	BadCstr(Utf8Error),
}

impl Display for OptionsError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::UnrecognizedOption(opt) => write!(f, "Unrecognized VM init option: {opt}"),
			Self::BadCstr(err) => write!(f, "Encountered a bad C string: {err}"),
		}
	}
}

impl From<Utf8Error> for OptionsError {
	fn from(value: Utf8Error) -> Self {
		Self::BadCstr(value)
	}
}
