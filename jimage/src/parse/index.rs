use crate::error::{Error, Result};
use crate::header::JImageHeader;
use crate::index::JImageIndex;

use std::io::Read;

use common::box_slice;
use common::endian::Endian;
use common::traits::JavaEndianAwareRead;

// The following is the format of the index;
//
//         +-------------------+
//         |   Redirect Table  |
//         +-------------------+
//         | Attribute Offsets |
//         +-------------------+
//         |   Attribute Data  |
//         +-------------------+
//         |      Strings      |
//         +-------------------+
//
// Redirect Table - Array of 32-bit signed values representing actions that
//                  should take place for hashed strings that map to that
//                  value.  Negative values indicate no hash collision and can be
//                  quickly converted to indices into attribute offsets.  Positive
//                  values represent a new seed for hashing an index into attribute
//                  offsets.  Zero indicates not found.
// Attribute Offsets - Array of 32-bit unsigned values representing offsets into
//                     attribute data.  Attribute offsets can be iterated to do a
//                     full survey of resources in the image.  Offset of zero
//                     indicates no attributes.
// Attribute Data - Bytes representing compact attribute data for locations. (See comments in ImageLocation.)
// Strings - Collection of zero terminated UTF-8 strings used by the index and
//           image meta data.  Each string is accessed by offset.  Each string is
//           unique.  Offset zero is reserved for the empty string.
pub(crate) fn read_index<R>(
	reader: &mut R,
	header: JImageHeader,
	endian: Endian,
) -> Result<JImageIndex>
where
	R: Read,
{
	if !endian.is_target() {
		panic!("Non-target index table reading is not implemented");
	}

	let redirect_table_length = header.table_length();
	let mut redirects_table = box_slice![0; redirect_table_length];
	endian
		.read_s4_into(reader, &mut redirects_table)
		.map_err(|_| Error::InvalidTableSize)?;

	let offset_table_length = header.table_length();
	let mut offsets_table = box_slice![0; offset_table_length];
	endian
		.read_u4_into(reader, &mut offsets_table)
		.map_err(|_| Error::InvalidTableSize)?;

	let location_table_length = header.location_table_length() as usize;
	let mut location_bytes = box_slice![0; location_table_length];
	reader
		.read_exact(&mut location_bytes)
		.map_err(|_| Error::InvalidTableSize)?;

	let string_table_length = header.string_table_length();
	let mut string_bytes = box_slice![0; string_table_length];
	reader
		.read_exact(&mut string_bytes)
		.map_err(|_| Error::InvalidTableSize)?;

	Ok(JImageIndex::new(
		redirects_table.into(),
		offsets_table.into(),
		location_bytes.into(),
		string_bytes.into(),
	))
}
