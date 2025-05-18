use std::fmt::{Display, Formatter};
use std::string::FromUtf8Error;

pub enum OptionsError {
	UnrecognizedOption(String),
	BadCstr,
}

impl Display for OptionsError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::UnrecognizedOption(opt) => write!(f, "Unrecognized VM init option: {opt}"),
			Self::BadCstr => write!(f, "Encountered a bad C string"),
		}
	}
}

impl From<FromUtf8Error> for OptionsError {
	fn from(_: FromUtf8Error) -> Self {
		Self::BadCstr
	}
}
