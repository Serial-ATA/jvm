#[derive(Copy, Clone, Debug)]
pub enum Endian {
	Little,
	Big,
}

impl Endian {
	pub fn invert(self) -> Self {
		match self {
			Self::Little => Self::Big,
			Self::Big => Self::Little,
		}
	}

	pub fn is_target(self) -> bool {
		match self {
			Self::Little => cfg!(target_endian = "little"),
			Self::Big => cfg!(target_endian = "big"),
		}
	}
}

#[ouroboros::self_referencing(pub_extras)]
#[derive(Debug)]
pub struct JImage {
	pub endian: Endian,                      // Endian handler
	pub header: crate::header::JImageHeader, // Image header
	pub data: Vec<u8>,                       // The entire JImage's data
	#[borrows(data)]
	#[not_covariant]
	pub index: crate::JImageIndex<'this>, // Information related to resource lookup
}
