use common::int_types::{u1, u8};

// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.hpp#L233
#[rustfmt::skip]
#[repr(u8)]
pub enum JImageLocation {
	ATTRIBUTE_END          = 0, // End of attribute stream marker
	ATTRIBUTE_MODULE       = 1, // String table offset of module name
	ATTRIBUTE_PARENT       = 2, // String table offset of resource path parent
	ATTRIBUTE_BASE         = 3, // String table offset of resource path base
	ATTRIBUTE_EXTENSION    = 4, // String table offset of resource path extension
	ATTRIBUTE_OFFSET       = 5, // Container byte offset of resource
	ATTRIBUTE_COMPRESSED   = 6, // In image byte size of the compressed resource
	ATTRIBUTE_UNCOMPRESSED = 7, // In memory byte size of the uncompressed resource
	ATTRIBUTE_COUNT        = 8, // Number of attribute kinds
}

impl JImageLocation {
	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.hpp#L252
	#[inline(always)]
	pub fn attribute_length(data: u1) -> u1 {
		(data & 0x7) + 1
	}

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.hpp#L257
	#[inline(always)]
	pub fn attribute_kind(data: u1) -> u1 {
		let kind = data >> 3;
		assert!(
			kind < Self::ATTRIBUTE_COUNT as u1,
			"Invalid JImage attribute kind: {}",
			data
		);
		kind
	}

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.hpp#L264
	#[inline(always)]
	pub fn attribute_value(data: &[u1], length: u1) -> u8 {
		assert!(
			(1..=8).contains(&length),
			"Invalid JImage attribute value length: {}",
			length
		);

		let mut value = 0u64;
		for i in 0..length {
			value <<= 8;
			value |= u64::from(data[i as usize]);
		}

		value
	}
}
