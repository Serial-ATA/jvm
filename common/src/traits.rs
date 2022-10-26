use crate::types::{u1, u2, u4};

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
		u16::from_be_bytes(buf)
	}

	fn read_u4(&mut self) -> u4 {
		let mut buf = [0u8; 4];
		self.read_exact(&mut buf).unwrap();
		u32::from_be_bytes(buf)
	}
}

impl<R: Read> JavaReadExt for R {}

pub trait ReferenceType<T> {
	fn new(val: T) -> Self;
	fn as_raw(&self) -> *const T;
	fn as_mut_raw(&self) -> *mut T;
}