pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	Io(std::io::Error),
	Fmt(std::fmt::Error),
}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Error::Io(e) => e.fmt(f),
			Error::Fmt(e) => e.fmt(f),
		}
	}
}

impl From<std::io::Error> for Error {
	fn from(err: std::io::Error) -> Self {
		Error::Io(err)
	}
}

impl From<std::fmt::Error> for Error {
	fn from(err: std::fmt::Error) -> Self {
		Error::Fmt(err)
	}
}

impl core::error::Error for Error {}
