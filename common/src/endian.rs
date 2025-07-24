use crate::error::Result;
use crate::int_types::{s4, u1, u4, u8};
use crate::traits::{JavaLittleEndianRead, JavaReadExt};

use std::io::Read;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Endian {
	Little,
	Big,
}

impl Endian {
	/// Get the native `Endian` for this target
	///
	/// # Examples
	///
	/// ```rust
	/// use common::endian::Endian;
	///
	/// let endian = Endian::native();
	/// assert_eq!(endian, Endian::Little);
	/// ```
	pub const fn native() -> Self {
		#[cfg(target_endian = "little")]
		return Endian::Little;
		#[cfg(target_endian = "big")]
		return Endian::Big;
	}

	/// Convert this `Endian` to the opposite value
	///
	/// # Examples
	///
	/// ```rust
	/// use common::endian::Endian;
	///
	/// let endian = Endian::Little;
	/// assert_eq!(endian.invert(), Endian::Big);
	/// ```
	pub const fn invert(self) -> Self {
		match self {
			Self::Little => Self::Big,
			Self::Big => Self::Little,
		}
	}

	/// Whether this `Endian` is this target's endianness
	///
	/// # Examples
	///
	/// ```rust
	/// use common::endian::Endian;
	///
	/// let endian = Endian::Little;
	/// assert!(endian.is_target());
	/// assert!(!Endian::Big.is_target());
	/// ```
	pub const fn is_target(self) -> bool {
		match self {
			Self::Little => cfg!(target_endian = "little"),
			Self::Big => cfg!(target_endian = "big"),
		}
	}
}

impl Endian {
	pub fn read_u1<R>(self, reader: &mut R) -> Result<u1>
	where
		R: Read,
	{
		JavaReadExt::read_u1(reader)
	}

	pub fn read_u4<R>(self, reader: &mut R) -> Result<u4>
	where
		R: Read,
	{
		match self {
			Endian::Little => JavaLittleEndianRead::read_u4(reader),
			Endian::Big => JavaReadExt::read_u4(reader),
		}
	}

	pub fn read_u8<R>(self, reader: &mut R) -> Result<u8>
	where
		R: Read,
	{
		match self {
			Endian::Little => JavaLittleEndianRead::read_u8(reader),
			Endian::Big => JavaReadExt::read_u8(reader),
		}
	}

	pub fn read_s4<R>(self, reader: &mut R) -> Result<s4>
	where
		R: Read,
	{
		match self {
			Endian::Little => JavaLittleEndianRead::read_s4(reader),
			Endian::Big => JavaReadExt::read_s4(reader),
		}
	}

	pub fn read_s4_into<R>(self, reader: &mut R, dst: &mut [s4]) -> Result<()>
	where
		R: Read,
	{
		match self {
			Endian::Little => JavaLittleEndianRead::read_s4_into(reader, dst),
			Endian::Big => JavaReadExt::read_s4_into(reader, dst),
		}
	}

	pub fn read_u4_into<R>(self, reader: &mut R, dst: &mut [u4]) -> Result<()>
	where
		R: Read,
	{
		match self {
			Endian::Little => JavaLittleEndianRead::read_u4_into(reader, dst),
			Endian::Big => JavaReadExt::read_u4_into(reader, dst),
		}
	}
}
