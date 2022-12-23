use byte_slice_cast::AsSliceOf;
use common::endian::Endian;
use common::int_types::{s4, u1, u4};
use jimage::{JImageHeader, JImageIndex};

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
// Attribute Data - Bytes representing compact attribute data for locations. (See
//                  comments in ImageLocation.)
// Strings - Collection of zero terminated UTF-8 strings used by the index and
//           image meta data.  Each string is accessed by offset.  Each string is
//           unique.  Offset zero is reserved for the empty string.
pub(crate) fn read_index(data: &[u1], header: JImageHeader, endian: Endian) -> JImageIndex<'_> {
	if !endian.is_target() {
		panic!("Non-target index table reading is not implemented");
	}

	let redirect_table_offset = header.redirect_table_offset();
	let redirects_table =
		&data[redirect_table_offset..redirect_table_offset + header.table_length()];

	let redirects_table = redirects_table.as_slice_of::<s4>().unwrap();

	let offset_table_offset = header.offset_table_offset();
	let offsets_table =
		&data[offset_table_offset..offset_table_offset + header.offset_table_length()];

	let offsets_table = offsets_table.as_slice_of::<u4>().unwrap();

	let location_table_offset = header.location_table_offset();
	let location_bytes = &data
		[location_table_offset..location_table_offset + header.location_table_length() as usize];

	let string_table_offset = header.string_table_offset();
	let string_bytes =
		&data[string_table_offset..string_table_offset + header.string_table_length() as usize];

	JImageIndex {
		redirects_table,
		offsets_table,
		location_bytes,
		string_bytes,
	}
}
