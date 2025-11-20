use crate::attribute::AttributeTag;
use crate::constant_pool::types::ConstantPoolEntryError;
use crate::parse::attributes::Location;

use std::error::Error;
use std::fmt::{Display, Formatter};

use common::int_types::u1;

pub type Result<T> = std::result::Result<T, ClassFileParseError>;

#[derive(Debug)]
pub enum ClassFileParseError {
	InvalidMagic,
	InvalidLocation(AttributeTag, Location),

	EmptyConstantPool,
	BadConstantPoolTag(u1),
	ConstantPoolEntry(ConstantPoolEntryError),

	BadAttributeTag(Vec<u8>),
	BadAttributeVerification(u1),
	BadElementTag(u1),

	BadFieldType(u1),
	InvalidMethodDescriptor(&'static str),

	Common(common::error::CommonError),
	Io(std::io::Error),
}

impl Display for ClassFileParseError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::InvalidMagic => write!(f, "File has an invalid magic signature!"),
			Self::InvalidLocation(tag, location) => write!(
				f,
				"Invalid attribute location: {:?} found in {:?}",
				tag, location
			),

			Self::EmptyConstantPool => write!(f, "Constant pool count is zero"),
			Self::BadConstantPoolTag(tag) => {
				write!(f, "Encountered invalid constant pool tag {tag}")
			},
			Self::ConstantPoolEntry(error) => error.fmt(f),

			Self::BadAttributeTag(attr) => {
				write!(
					f,
					"Encountered unknown attribute tag: `{}`",
					attr.escape_ascii()
				)
			},
			Self::BadAttributeVerification(tag) => {
				write!(f, "Encountered invalid verification type info tag: {tag}")
			},
			Self::BadElementTag(tag) => {
				write!(
					f,
					"Encountered unknown element tag: `{}`",
					tag.escape_ascii()
				)
			},

			Self::BadFieldType(ty) => {
				write!(f, "Encountered invalid field type descriptor: {ty}")
			},
			Self::InvalidMethodDescriptor(message) => {
				write!(f, "Encountered invalid method descriptor: {}", message)
			},

			Self::Common(err) => write!(f, "{}", err),
			Self::Io(err) => write!(f, "{}", err),
		}
	}
}

impl Error for ClassFileParseError {}

impl From<ConstantPoolEntryError> for ClassFileParseError {
	fn from(err: ConstantPoolEntryError) -> Self {
		Self::ConstantPoolEntry(err)
	}
}

impl From<common::error::CommonError> for ClassFileParseError {
	fn from(value: common::error::CommonError) -> Self {
		Self::Common(value)
	}
}

impl From<std::io::Error> for ClassFileParseError {
	fn from(value: std::io::Error) -> Self {
		Self::Io(value)
	}
}
