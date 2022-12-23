use crate::int_types::{s4, u1, u2, u4, u8};

use std::io::Read;

pub trait JavaReadExt: Read {
	fn read_u1(&mut self) -> u1 {
		let mut buf = [0u8; 1];
		self.read_exact(&mut buf).unwrap();
		buf[0]
	}

	fn read_u2(&mut self) -> u2 {
		let mut buf = [0u8; 2];
		self.read_exact(&mut buf).unwrap();
		u2::from_be_bytes(buf)
	}

	fn read_u4(&mut self) -> u4 {
		let mut buf = [0u8; 4];
		self.read_exact(&mut buf).unwrap();
		u4::from_be_bytes(buf)
	}

	fn read_u8(&mut self) -> u8 {
		let mut buf = [0u8; 8];
		self.read_exact(&mut buf).unwrap();
		u8::from_be_bytes(buf)
	}

	fn read_s4(&mut self) -> s4 {
		let mut buf = [0u8; 4];
		self.read_exact(&mut buf).unwrap();
		s4::from_be_bytes(buf)
	}
}

impl<R: Read> JavaReadExt for R {}

pub trait JavaLittleEndianRead: Read {
	fn read_u4(&mut self) -> u4 {
		let mut buf = [0u8; 4];
		self.read_exact(&mut buf).unwrap();
		u4::from_le_bytes(buf)
	}

	fn read_u8(&mut self) -> u8 {
		let mut buf = [0u8; 8];
		self.read_exact(&mut buf).unwrap();
		u8::from_le_bytes(buf)
	}

	fn read_s4(&mut self) -> s4 {
		let mut buf = [0u8; 4];
		self.read_exact(&mut buf).unwrap();
		s4::from_le_bytes(buf)
	}
}

impl<R: Read> JavaLittleEndianRead for R {}

pub trait JavaEndianAwareRead<R: Read> {
	fn read_u1(self, reader: &mut R) -> u1;
	fn read_u4(self, reader: &mut R) -> u4;
	fn read_u8(self, reader: &mut R) -> u8;
	fn read_s4(self, reader: &mut R) -> s4;
}

pub trait PtrType<T, RefType> {
	fn new(val: T) -> RefType;
	fn as_raw(&self) -> *const T;
	fn as_mut_raw(&self) -> *mut T;
	fn get(&self) -> &T;
	fn get_mut(&self) -> &mut T;
}
