use common::int_types::{s4, u1, u4};

#[derive(Clone, Debug)]
pub struct JImageIndex<'a> {
	pub redirects_table: &'a [s4], // Perfect hash redirect table
	pub offsets_table: &'a [u4],   // Location offset table
	pub location_bytes: &'a [u1],  // Location attributes
	pub string_bytes: &'a [u1],    // String table
}
