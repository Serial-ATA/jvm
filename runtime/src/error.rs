use std::error::Error;
use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, RuntimeError>;

#[derive(Debug)]
pub enum RuntimeError {
	BadClassName,
}

impl Display for RuntimeError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::BadClassName => write!(f, "Encountered a bad class name"),
		}
	}
}

impl Error for RuntimeError {}
