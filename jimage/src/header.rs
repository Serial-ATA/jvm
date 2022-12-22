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
	pub fn redirect_table_offset(&self) -> usize {
		core::mem::size_of::<Self>()
	}

	#[inline(always)]
	pub fn offset_table_offset(&self) -> usize {
		self.redirect_table_offset() + self.table_length()
	}

	#[inline(always)]
	pub fn location_table_offset(&self) -> usize {
		self.offset_table_offset() + self.table_length()
	}

	#[inline(always)]
	pub fn string_table_offset(&self) -> usize {
		self.location_table_offset() + self.locations_size as usize
	}

	#[inline(always)]
	pub fn table_length(&self) -> usize {
		self.table_length as usize * core::mem::size_of::<u4>()
	}

	#[inline(always)]
	pub fn offset_table_length(&self) -> usize {
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
