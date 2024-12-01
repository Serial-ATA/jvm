use common::int_types::{u1, u2, u4};

pub const JIMAGE_MAGIC: u4 = 0xCAFE_DADA;
pub const JIMAGE_MAGIC_INVERTED: u4 = 0xDADA_FECA;
pub const JIMAGE_MAJOR_VERSION: u1 = 1;
pub const JIMAGE_MINOR_VERSION: u1 = 0;

#[derive(Copy, Clone, Debug)]
pub struct JImageHeader {
	#[doc(hidden)]
	pub(crate) __magic: u4, // Only here to make it the correct size
	pub(crate) version: u4,
	pub(crate) flags: u4,
	pub(crate) resource_count: u4,
	pub(crate) table_length: u4,
	pub(crate) locations_size: u4,
	pub(crate) strings_size: u4,
}

impl JImageHeader {
	/// Get the major version number for the JImage file
	#[inline(always)]
	pub fn major_version(&self) -> u2 {
		(self.version >> 16) as u2
	}

	/// Get the minor version number for the JImage file
	#[inline(always)]
	pub fn minor_version(&self) -> u2 {
		(self.version & 0xFFFF) as u2
	}

	/// Get the flags
	///
	/// Currently unused
	#[inline(always)]
	pub fn flags(&self) -> u4 {
		self.flags
	}

	/// Get the resource count
	///
	/// This is the number of resources in the JImage file.
	#[inline(always)]
	pub fn resource_count(&self) -> u4 {
		self.resource_count
	}

	/// The length of the lookup tables in the [`JImageIndex`](crate::JImageIndex) (**in elements, not bytes!**)
	#[inline(always)]
	pub fn table_length(&self) -> usize {
		self.table_length as usize
	}

	/// The start of the redirect table, relative to the beginning of the file
	#[inline(always)]
	pub fn redirect_table_offset(&self) -> usize {
		size_of::<Self>()
	}

	/// The size of the redirect table, in bytes
	#[inline(always)]
	pub fn redirect_table_length(&self) -> usize {
		self.table_length() * size_of::<u4>()
	}

	/// The start of the offset table, relative to the beginning of the file
	#[inline(always)]
	pub fn offset_table_offset(&self) -> usize {
		self.redirect_table_offset() + self.table_length()
	}

	/// The size of the offset table, in bytes
	#[inline(always)]
	pub fn offset_table_length(&self) -> usize {
		self.table_length() * size_of::<u4>()
	}

	/// The start of the location table, relative to the beginning of the file
	#[inline(always)]
	pub fn location_table_offset(&self) -> usize {
		self.offset_table_offset() + self.table_length()
	}

	/// The size of the location table, in bytes
	#[inline(always)]
	pub fn location_table_length(&self) -> u4 {
		self.locations_size
	}

	/// The start of the string table, relative to the beginning of the file
	#[inline(always)]
	pub fn string_table_offset(&self) -> usize {
		self.location_table_offset() + self.locations_size as usize
	}

	/// The size of the string table, in bytes
	#[inline(always)]
	pub fn string_table_length(&self) -> u4 {
		self.strings_size
	}

	/// The size of the entire index
	pub fn index_length(&self) -> usize {
		core::mem::size_of::<Self>()
			+ self.redirect_table_length()
			+ self.offset_table_length()
			+ self.locations_size as usize
			+ self.strings_size as usize
	}
}
