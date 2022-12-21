use std::io::Read;

use common::int_types::{s4, u1, u4};
use common::traits::JavaReadExt;
use jimage::Endian;

pub(crate) trait JavaLittleEndianRead: Read {
	fn read_u4(&mut self) -> u4 {
		let mut buf = [0u8; 4];
		self.read_exact(&mut buf).unwrap();
		u4::from_le_bytes(buf)
	}

	fn read_s4(&mut self) -> s4 {
		let mut buf = [0u8; 4];
		self.read_exact(&mut buf).unwrap();
		s4::from_le_bytes(buf)
	}
}

impl<R: Read> JavaLittleEndianRead for R {}

pub(crate) trait JavaEndianAwareRead<R: Read> {
	fn read_u1(self, reader: &mut R) -> u1;
	fn read_u4(self, reader: &mut R) -> u4;
	fn read_s4(self, reader: &mut R) -> s4;
}

impl<R: Read> JavaEndianAwareRead<R> for Endian {
	fn read_u1(self, reader: &mut R) -> u1 {
		JavaReadExt::read_u1(reader)
	}

	fn read_u4(self, reader: &mut R) -> u4 {
		match self {
			Endian::Little => JavaLittleEndianRead::read_u4(reader),
			Endian::Big => JavaReadExt::read_u4(reader),
		}
	}

	fn read_s4(self, reader: &mut R) -> s4 {
		match self {
			Endian::Little => JavaLittleEndianRead::read_s4(reader),
			Endian::Big => JavaReadExt::read_s4(reader),
		}
	}
}
