use common::int_types::{u1, u4};

pub const JIMAGE_MAGIC: u4 = 0xCAFE_DADA;
pub const JIMAGE_MAGIC_INVERTED: u4 = 0xDADA_FECA;
pub const JIMAGE_MAJOR_VERSION: u1 = 1;
pub const JIMAGE_MINOR_VERSION: u1 = 0;

#[derive(Copy, Clone, Debug)]
pub struct JImageHeader {
	#[doc(hidden)]
	pub __magic: u4, // Only here to make it the correct size
	pub version: u4,
	pub flags: u4,
	pub resource_count: u4,
	pub table_length: u4,
	pub locations_size: u4,
	pub strings_size: u4,
}

impl JImageHeader {
	#[inline(always)]
	pub fn redirect_table_offset(&self) -> u4 {
		core::mem::size_of::<Self>() as u4
	}

	#[inline(always)]
	pub fn offset_table_offset(&self) -> u4 {
		self.redirect_table_offset() + self.table_length()
	}

	#[inline(always)]
	pub fn location_table_offset(&self) -> u4 {
		self.offset_table_offset() + self.table_length()
	}

	#[inline(always)]
	pub fn string_table_offset(&self) -> u4 {
		self.location_table_offset() + self.locations_size
	}

	#[inline(always)]
	pub fn table_length(&self) -> u4 {
		self.table_length * core::mem::size_of::<u4>() as u4
	}

	#[inline(always)]
	pub fn offset_table_length(&self) -> u4 {
		self.location_table_offset() - self.offset_table_offset()
	}

	#[inline(always)]
	pub fn location_table_length(&self) -> u4 {
		self.locations_size
	}

	#[inline(always)]
	pub fn string_table_length(&self) -> u4 {
		self.strings_size
	}
}
