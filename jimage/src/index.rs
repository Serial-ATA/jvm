#[derive(Clone, Debug)]
pub struct JImageIndex<'a> {
	pub redirects_table: &'a [i32], // Perfect hash redirect table
	pub offsets_table: &'a [u32],   // Location offset table
	pub location_bytes: &'a [u8],   // Location attributes
	pub string_bytes: &'a [u8],     // String table
}
