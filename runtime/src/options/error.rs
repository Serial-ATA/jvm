use crate::options::logging::LogParseError;

use std::fmt::{Display, Write};
use std::str::Utf8Error;

pub enum SizeTarget {
	InitialHeap,
	MaxHeap,
}

impl Display for SizeTarget {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			SizeTarget::InitialHeap => f.write_str("initial heap size"),
			SizeTarget::MaxHeap => f.write_str("maximum heap size"),
		}
	}
}

pub enum OptionsError {
	UnrecognizedOption(String),
	BadCstr(Utf8Error),
	/// Failed to parse a `-Xlog` option
	Logging(LogParseError),
    /// Failed to a parse a `<size>[unit]` string (e.g., `64m` in `-Xms64m`)
	InvalidSize(SizeTarget, String),
}

impl Display for OptionsError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::UnrecognizedOption(opt) => write!(f, "Unrecognized VM init option: {opt}"),
			Self::BadCstr(err) => write!(f, "Encountered a bad C string: {err}"),
			Self::Logging(err) => err.fmt(f),
			Self::InvalidSize(target, opt) => write!(f, "Invalid {target}: {opt}"),
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
