pub const JIMAGE_MAGIC: u32 = 0xCAFE_DADA;
pub const JIMAGE_MAGIC_INVERTED: u32 = 0xDADA_FECA;
pub const JIMAGE_MAJOR_VERSION: u8 = 1;
pub const JIMAGE_MINOR_VERSION: u8 = 0;

#[derive(Copy, Clone, Debug)]
pub struct JImageHeader {
	#[doc(hidden)]
	pub __magic: u32, // Only here to make it the correct size
	pub version: u32,
	pub flags: u32,
	pub resource_count: u32,
	pub table_length: u32,
	pub locations_size: u32,
	pub strings_size: u32,
}

impl JImageHeader {
	#[inline(always)]
	pub fn redirect_table_offset(&self) -> u32 {
		core::mem::size_of::<Self>() as u32
	}

	#[inline(always)]
	pub fn offset_table_offset(&self) -> u32 {
		self.redirect_table_offset() + self.table_length()
	}

	#[inline(always)]
	pub fn location_table_offset(&self) -> u32 {
		self.offset_table_offset() + self.table_length()
	}

	#[inline(always)]
	pub fn string_table_offset(&self) -> u32 {
		self.location_table_offset() + self.locations_size
	}

	#[inline(always)]
	pub fn table_length(&self) -> u32 {
		self.table_length * core::mem::size_of::<u32>() as u32
	}

	#[inline(always)]
	pub fn offset_table_length(&self) -> u32 {
		self.location_table_offset() - self.offset_table_offset()
	}

	#[inline(always)]
	pub fn location_table_length(&self) -> u32 {
		self.locations_size
	}

	#[inline(always)]
	pub fn string_table_length(&self) -> u32 {
		self.strings_size
	}
}
