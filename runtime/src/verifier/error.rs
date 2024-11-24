use common::int_types::u2;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	SuperClassFinal,
	FinalMethodOverridden,
	BadExceptionHandlerRange(u2, u2),
	InstructionOutOfBounds(u2, usize),
	HandlerNotThrowable,
}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Error::SuperClassFinal => write!(f, "Super class is declared final"),
			Error::FinalMethodOverridden => write!(f, "A method marked `final` was overridden"),
			Error::BadExceptionHandlerRange(start, end) => {
				write!(f, "Exception handler has a bad range ({start}..{end})")
			},
			Error::InstructionOutOfBounds(position, end) => {
				write!(
					f,
					"Instruction pointer is out of bounds (position: {position}, end: {end})"
				)
			},
			Error::HandlerNotThrowable => write!(f, "A method exception handler is not throwable"),
		}
	}
}

impl core::error::Error for Error {}
