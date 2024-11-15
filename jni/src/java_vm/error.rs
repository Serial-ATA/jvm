#[derive(Debug)]
pub enum Error {
	LibJvmNotFound,
	LibJvmLoad(libloading::Error),
	SymbolNotFound(&'static [u8]),
	JavaVmNull,
}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Error::LibJvmNotFound => write!(f, "Could not find libjvm_runtime"),
			Error::LibJvmLoad(e) => write!(f, "Could not load libjvm_runtime: {}", e),
			Error::SymbolNotFound(symbol) => {
				write!(
					f,
					"Could not find symbol `{}` in libjvm_runtime",
					symbol.escape_ascii()
				)
			},
			Error::JavaVmNull => write!(f, "Java VM was not populated"),
		}
	}
}

impl From<libloading::Error> for Error {
	fn from(e: libloading::Error) -> Self {
		Error::LibJvmLoad(e)
	}
}

impl core::error::Error for Error {}
