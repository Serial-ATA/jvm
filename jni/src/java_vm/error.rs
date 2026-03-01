#[derive(Debug)]
pub enum Error {
	LibJvmNotFound,
	LibJvmLoad(platform::libs::Error),
	SymbolNotFound(&'static [u8]),
	JavaVmNull,
}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Error::LibJvmNotFound => write!(f, "Could not find libjvm"),
			Error::LibJvmLoad(e) => write!(f, "Could not load libjvm: {}", e),
			Error::SymbolNotFound(symbol) => {
				write!(
					f,
					"Could not find symbol `{}` in libjvm",
					symbol.escape_ascii()
				)
			},
			Error::JavaVmNull => write!(f, "Java VM was not populated"),
		}
	}
}

impl From<platform::libs::Error> for Error {
	fn from(e: platform::libs::Error) -> Self {
		Error::LibJvmLoad(e)
	}
}

impl core::error::Error for Error {}
