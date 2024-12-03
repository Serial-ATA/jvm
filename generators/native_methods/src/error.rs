pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	Io(std::io::Error),
}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Error::Io(e) => e.fmt(f),
		}
	}
}

impl From<std::io::Error> for Error {
	fn from(err: std::io::Error) -> Self {
		Error::Io(err)
	}
}

impl core::error::Error for Error {}
