use crate::error::Result;
use crate::int_types::{s4, u1, u2, u4, u8};

use std::io::Read;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

/// Big endian read operations for Java integer types
///
/// Java's specification uses big endian, so this is the default "Java" reader extension trait.
/// See [`JavaLittleEndianRead`] for the little endian equivalents.
pub trait JavaReadExt: Read {
	fn read_u1(&mut self) -> Result<u1> {
		let mut buf = [0u8; 1];
		self.read_exact(&mut buf)?;
		Ok(buf[0])
	}

	fn read_u2(&mut self) -> Result<u2> {
		let mut buf = [0u8; 2];
		self.read_exact(&mut buf)?;
		Ok(u2::from_be_bytes(buf))
	}

	fn read_u4(&mut self) -> Result<u4> {
		let mut buf = [0u8; 4];
		self.read_exact(&mut buf)?;
		Ok(u4::from_be_bytes(buf))
	}

	fn read_u8(&mut self) -> Result<u8> {
		let mut buf = [0u8; 8];
		self.read_exact(&mut buf)?;
		Ok(u8::from_be_bytes(buf))
	}

	fn read_s4(&mut self) -> Result<s4> {
		let mut buf = [0u8; 4];
		self.read_exact(&mut buf)?;
		Ok(s4::from_be_bytes(buf))
	}

	fn read_s4_into(&mut self, dst: &mut [s4]) -> Result<()> {
		Ok(self.read_i32_into::<BigEndian>(dst)?)
	}

	fn read_u4_into(&mut self, dst: &mut [u4]) -> Result<()> {
		Ok(self.read_u32_into::<BigEndian>(dst)?)
	}
}

impl<R: Read> JavaReadExt for R {}

/// Little endian read operations for Java integer types
///
/// See [`JavaReadExt`] for the big endian counterpart.
pub trait JavaLittleEndianRead: Read {
	fn read_u4(&mut self) -> Result<u4> {
		let mut buf = [0u8; 4];
		self.read_exact(&mut buf)?;
		Ok(u4::from_le_bytes(buf))
	}

	fn read_u8(&mut self) -> Result<u8> {
		let mut buf = [0u8; 8];
		self.read_exact(&mut buf)?;
		Ok(u8::from_le_bytes(buf))
	}

	fn read_s4(&mut self) -> Result<s4> {
		let mut buf = [0u8; 4];
		self.read_exact(&mut buf)?;
		Ok(s4::from_le_bytes(buf))
	}

	fn read_s4_into(&mut self, dst: &mut [s4]) -> Result<()> {
		Ok(self.read_i32_into::<LittleEndian>(dst)?)
	}

	fn read_u4_into(&mut self, dst: &mut [u4]) -> Result<()> {
		Ok(self.read_u32_into::<LittleEndian>(dst)?)
	}
}

impl<R: Read> JavaLittleEndianRead for R {}
