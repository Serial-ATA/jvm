use crate::error::Result;
use crate::int_types::{s4, u1, u2, u4, u8};

use std::io::Read;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

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

pub trait JavaEndianAwareRead<R: Read> {
	fn read_u1(self, reader: &mut R) -> Result<u1>;
	fn read_u4(self, reader: &mut R) -> Result<u4>;
	fn read_u8(self, reader: &mut R) -> Result<u8>;
	fn read_s4(self, reader: &mut R) -> Result<s4>;

	fn read_s4_into(self, reader: &mut R, dst: &mut [s4]) -> Result<()>;
	fn read_u4_into(self, reader: &mut R, dst: &mut [u4]) -> Result<()>;
}

pub trait PtrType<T, RefType> {
	fn new(val: T) -> RefType;
	fn as_raw(&self) -> *const T;
	fn as_mut_raw(&self) -> *mut T;
	fn get(&self) -> &T;
	fn get_mut(&self) -> &mut T;
}
