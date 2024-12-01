use common::int_types::{s4, u1, u4};

#[derive(Clone, Debug)]
pub struct JImageIndex {
	redirects_table: Box<[s4]>, // Perfect hash redirect table
	offsets_table: Box<[u4]>,   // Location offset table
	location_bytes: Box<[u1]>,  // Location attributes
	string_bytes: Box<[u1]>,    // String table
}

impl JImageIndex {
	pub(crate) fn new(
		redirects_table: Box<[s4]>,
		offsets_table: Box<[u4]>,
		location_bytes: Box<[u1]>,
		string_bytes: Box<[u1]>,
	) -> Self {
		Self {
			redirects_table,
			offsets_table,
			location_bytes,
			string_bytes,
		}
	}

	pub fn redirects_table(&self) -> &[s4] {
		&self.redirects_table
	}
	pub fn offsets_table(&self) -> &[u4] {
		&self.offsets_table
	}
	pub fn location_bytes(&self) -> &[u1] {
		&self.location_bytes
	}
	pub fn string_bytes(&self) -> &[u1] {
		&self.string_bytes
	}

	/// The length of the entire index, in bytes
	pub fn len(&self) -> usize {
		(self.redirects_table.len() * size_of::<s4>())
			+ (self.offsets_table.len() * size_of::<u4>())
			+ self.location_bytes.len()
			+ self.string_bytes.len()
	}
}
