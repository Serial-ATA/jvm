use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

#[derive(Debug, Clone)]
pub enum VersionError {
	NoMatch,
	PlusGroupWithPreAndOptional,
	UnMatchedPlusGroup,
	UnMatchedOptional,
	ParseInt(ParseIntError),
}

impl Display for VersionError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			VersionError::NoMatch => write!(f, "Invalid version string"),
			VersionError::PlusGroupWithPreAndOptional => write!(
				f,
				"Version string contains '+' with pre-release and optional components"
			),
			VersionError::UnMatchedPlusGroup => write!(
				f,
				"Version string contains '+' with neither build or optional components"
			),
			VersionError::UnMatchedOptional => write!(
				f,
				"Version string contains an optional component that is not preceded by a \
				 pre-release or '+'"
			),
			VersionError::ParseInt(parse_int_error) => write!(f, "{}", parse_int_error),
		}
	}
}

impl Error for VersionError {}

impl From<ParseIntError> for VersionError {
	fn from(value: ParseIntError) -> Self {
		Self::ParseInt(value)
	}
}
