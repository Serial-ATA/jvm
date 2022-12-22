use crate::ImageStrings;

use common::int_types::{u1, u4, u8};

// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.hpp#L233
pub struct JImageLocation {
	attributes: [u8; Self::ATTRIBUTE_COUNT as usize],
}

impl JImageLocation {
	pub fn new() -> Self {
		Self {
			attributes: [0; Self::ATTRIBUTE_COUNT as usize],
		}
	}

	pub fn new_with_data(data: &[u1]) -> Self {
		let mut ret = Self::new();
		ret.set_data(data);
		ret
	}

	pub(crate) fn new_opt_(data: Option<&[u1]>) -> Self {
		match data {
			Some(data) => Self::new_with_data(data),
			None => Self::new(),
		}
	}
}

#[rustfmt::skip]
impl JImageLocation {
	pub const ATTRIBUTE_END         : u8 = 0; // End of attribute stream marker
	pub const ATTRIBUTE_MODULE      : u8 = 1; // String table offset of module name
	pub const ATTRIBUTE_PARENT      : u8 = 2; // String table offset of resource path parent
	pub const ATTRIBUTE_BASE        : u8 = 3; // String table offset of resource path base
	pub const ATTRIBUTE_EXTENSION   : u8 = 4; // String table offset of resource path extension
	pub const ATTRIBUTE_OFFSET      : u8 = 5; // Container byte offset of resource
	pub const ATTRIBUTE_COMPRESSED  : u8 = 6; // In image byte size of the compressed resource
	pub const ATTRIBUTE_UNCOMPRESSED: u8 = 7; // In memory byte size of the uncompressed resource
	pub const ATTRIBUTE_COUNT       : u8 = 8; // Number of attribute kinds
}

impl JImageLocation {
	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.hpp#L252
	/// Return the attribute value number of bytes.
	#[inline(always)]
	pub fn attribute_length(data: u1) -> u1 {
		(data & 0x7) + 1
	}

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.hpp#L257
	/// Return the attribute kind.
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
	/// Return the attribute length.
	#[inline(always)]
	pub fn attribute_value(data: &[u1], length: u1) -> u8 {
		assert!(
			(1..=8).contains(&length),
			"Invalid JImage attribute value length: {}",
			length
		);

		let mut value = 0;
		for i in 0..length {
			value <<= 8;
			value |= u8::from(data[i as usize]);
		}

		value
	}

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.cpp#L125
	/// Inflates the attribute stream into individual values stored in the long
	/// array _attributes. This allows an attribute value to be quickly accessed by
	/// direct indexing.  Unspecified values default to zero (from constructor.)
	pub fn set_data(&mut self, mut data: &[u1]) {
		// Deflate the attribute stream into an array of attributes.
		// Repeat until end header is found.
		for header_byte in data {
			// Extract kind from header byte.
			let kind = Self::attribute_kind(*header_byte);
			assert!(
				u8::from(kind) < Self::ATTRIBUTE_COUNT,
				"invalid image location attribute"
			);

			// Extract length of data (in bytes).
			let n = Self::attribute_length(*header_byte);

			// Read value (most significant first.)
			self.attributes[kind as usize] = Self::attribute_value(&data[1..], n);

			data = &data[n as usize..];
		}
	}

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.cpp#L143
	/// Zero all attribute values.
	pub fn clear_data(&mut self) {
		// Set defaults to zero.
		self.attributes.fill(0);
	}

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.hpp#L294
	/// Retrieve an attribute value from the inflated array.
	#[inline(always)]
	pub fn get_attribute(&self, kind: u1) -> u8 {
		assert!(
			Self::ATTRIBUTE_END < u8::from(kind) && u8::from(kind) < Self::ATTRIBUTE_COUNT,
			"invalid attribute kind"
		);
		self.attributes[kind as usize]
	}

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.hpp#L300
	/// Retrieve an attribute string value from the inflated array.
	#[inline(always)]
	pub fn get_attribute_string<'a>(&self, kind: u4, strings: &'a ImageStrings<'_>) -> &'a [u1] {
		strings.get(self.get_attribute(kind as u1) as u4)
	}
}
