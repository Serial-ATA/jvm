use std::error::Error;
use std::fmt::{Display, Formatter};

pub type Result<T> = std::result::Result<T, CommonError>;

#[derive(Debug)]
pub enum CommonError {
	Io(std::io::Error),
}

impl Display for CommonError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Io(err) => write!(f, "{}", err),
		}
	}
}

impl Error for CommonError {}

impl From<std::io::Error> for CommonError {
	fn from(value: std::io::Error) -> Self {
		Self::Io(value)
	}
}
