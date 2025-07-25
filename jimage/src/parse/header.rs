use crate::error::{Error, Result};
use crate::header::{JIMAGE_MAGIC, JIMAGE_MAGIC_INVERTED, JImageHeader};

use std::io::Read;

use common::endian::Endian;

// The header contains information related to identification and description of
// contents.
//
//         +-------------------------+
//         |   Magic (0xCAFEDADA)    |
//         +------------+------------+
//         | Major Vers | Minor Vers |
//         +------------+------------+
//         |          Flags          |
//         +-------------------------+
//         |      Resource Count     |
//         +-------------------------+
//         |       Table Length      |
//         +-------------------------+
//         |      Attributes Size    |
//         +-------------------------+
//         |       Strings Size      |
//         +-------------------------+
//
// Magic - means of identifying validity of the file.  This avoids requiring a
//         special file extension.
// Major vers, minor vers - differences in version numbers indicate structural
//                          changes in the image.
// Flags - various image wide flags (future).
// Resource count - number of resources in the file.
// Table length - the length of lookup tables used in the index.
// Attributes size - number of bytes in the region used to store location attribute
//                   streams.
// Strings size - the size of the region used to store strings used by the
//                index and meta data.
pub(crate) fn read_header<R>(reader: &mut R) -> Result<(JImageHeader, Endian)>
where
	R: Read,
{
	let mut endian = Endian::native();

	let magic = endian.read_u4(reader)?;
	match magic {
		// The image was created with the target endianness, nothing to do
		JIMAGE_MAGIC => {},
		// The image was created on a platform with opposite endianness
		JIMAGE_MAGIC_INVERTED => endian = endian.invert(),
		_ => return Err(Error::InvalidMagic),
	}

	let version = endian.read_u4(reader)?;

	let major_version = version >> 16;
	let minor_version = version & 0xFFFF;

	assert_eq!(
		major_version, 1,
		"Unsupported major version: {}",
		major_version
	);
	assert_eq!(
		minor_version, 0,
		"Unsupported minor version: {}",
		minor_version
	);

	let flags = endian.read_u4(reader)?;

	let resource_count = endian.read_u4(reader)?;
	let table_length = endian.read_u4(reader)?;
	let locations_size = endian.read_u4(reader)?;
	let strings_size = endian.read_u4(reader)?;

	let header = JImageHeader {
		__magic: magic,
		version,
		flags,
		resource_count,
		table_length,
		locations_size,
		strings_size,
	};

	Ok((header, endian))
}
