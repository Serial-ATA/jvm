use crate::error::Result;
use crate::int_types::{s4, u1, u4, u8};
use crate::traits::{JavaEndianAwareRead, JavaLittleEndianRead, JavaReadExt};

use std::io::Read;

#[derive(Copy, Clone, Debug)]
pub enum Endian {
	Little,
	Big,
}

impl Endian {
	pub fn invert(self) -> Self {
		match self {
			Self::Little => Self::Big,
			Self::Big => Self::Little,
		}
	}

	pub fn is_target(self) -> bool {
		match self {
			Self::Little => cfg!(target_endian = "little"),
			Self::Big => cfg!(target_endian = "big"),
		}
	}
}

impl<R: Read> JavaEndianAwareRead<R> for Endian {
	fn read_u1(self, reader: &mut R) -> Result<u1> {
		JavaReadExt::read_u1(reader)
	}

	fn read_u4(self, reader: &mut R) -> Result<u4> {
		match self {
			Endian::Little => JavaLittleEndianRead::read_u4(reader),
			Endian::Big => JavaReadExt::read_u4(reader),
		}
	}

	fn read_u8(self, reader: &mut R) -> Result<u8> {
		match self {
			Endian::Little => JavaLittleEndianRead::read_u8(reader),
			Endian::Big => JavaReadExt::read_u8(reader),
		}
	}

	fn read_s4(self, reader: &mut R) -> Result<s4> {
		match self {
			Endian::Little => JavaLittleEndianRead::read_s4(reader),
			Endian::Big => JavaReadExt::read_s4(reader),
		}
	}
}
