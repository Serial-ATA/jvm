use crate::error::Result;
use crate::{ImageStrings, JImage};

use std::fmt::{Debug, Formatter};

use common::int_types::{u1, u4, u8};

#[rustfmt::skip]
pub mod attr {
	use common::int_types::u8;
	
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

// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.hpp#L233
pub struct JImageLocation<'a> {
	attributes: [u8; attr::ATTRIBUTE_COUNT as usize],
	image: &'a JImage,
}

impl<'a> Debug for JImageLocation<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JImageLocation")
			.field("attributes", &self.attributes)
			.finish()
	}
}

impl<'a> JImageLocation<'a> {
	pub fn new(image: &'a JImage) -> Self {
		Self {
			attributes: [0; attr::ATTRIBUTE_COUNT as usize],
			image,
		}
	}

	pub fn new_with_data(image: &'a JImage, data: &[u1]) -> Self {
		let mut ret = Self::new(image);
		ret.set_data(data);
		ret
	}

	pub(crate) fn new_opt_(image: &'a JImage, data: Option<&[u1]>) -> Self {
		match data {
			Some(data) => Self::new_with_data(image, data),
			None => Self::new(image),
		}
	}
}

impl<'a> JImageLocation<'a> {
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
			kind < attr::ATTRIBUTE_COUNT as u1,
			"Invalid JImage attribute kind: {}",
			kind
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
	pub fn set_data(&mut self, data: &[u1]) {
		// Deflate the attribute stream into an array of attributes.
		// Repeat until end header is found.
		let mut i = 0;
		loop {
			// Extract kind from header byte.
			let header_byte = data[i];
			let kind = Self::attribute_kind(header_byte);
			if u8::from(kind) == attr::ATTRIBUTE_END {
				return;
			}

			// Extract length of data (in bytes).
			let n = Self::attribute_length(header_byte);

			// Read value (most significant first.)
			self.attributes[kind as usize] = Self::attribute_value(&data[i + 1..], n);
			i += n as usize + 1;
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
			attr::ATTRIBUTE_END < u8::from(kind) && u8::from(kind) < attr::ATTRIBUTE_COUNT,
			"invalid attribute kind"
		);
		self.attributes[kind as usize]
	}

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.hpp#L300
	/// Retrieve an attribute string value from the inflated array.
	#[inline(always)]
	pub fn get_attribute_string(&self, kind: u4, strings: ImageStrings<'a>) -> &'a [u1] {
		strings.get(self.get_attribute(kind as u1) as u4)
	}

	/// Retrieve the full name of the location
	///
	/// # Errors
	/// * The location contains a non UTF-8 attribute
	pub fn get_full_name(&self, modules_prefix: bool) -> Result<String> {
		let mut name = String::new();

		let module_offset = self.get_attribute(attr::ATTRIBUTE_MODULE as u1);
		if module_offset > 0 {
			if modules_prefix {
				name.push_str("/modules");
			}

			name.push('/');
			name.push_str(self.get_module()?);
			name.push('/');
		}

		let parent_offset = self.get_attribute(attr::ATTRIBUTE_PARENT as u1);
		if parent_offset > 0 {
			name.push_str(self.get_parent()?);
			name.push('/');
		}

		name.push_str(self.get_base()?);

		let extension_offset = self.get_attribute(attr::ATTRIBUTE_EXTENSION as u1);
		if extension_offset > 0 {
			name.push('.');
			name.push_str(self.get_extension()?);
		}

		Ok(name)
	}

	/// Retrieve the string at `ATTRIBUTE_MODULE`
	///
	/// # Errors
	/// * The string at the offset is non UTF-8
	pub fn get_module(&self) -> Result<&str> {
		let strings = ImageStrings(self.image.borrow_index().string_bytes);
		Ok(core::str::from_utf8(self.get_attribute_string(
			attr::ATTRIBUTE_MODULE as u4,
			strings,
		))?)
	}

	/// Retrieve the string at `ATTRIBUTE_PARENT`
	///
	/// # Errors
	/// * The string at the offset is non UTF-8
	pub fn get_parent(&self) -> Result<&str> {
		let strings = ImageStrings(self.image.borrow_index().string_bytes);
		Ok(core::str::from_utf8(self.get_attribute_string(
			attr::ATTRIBUTE_PARENT as u4,
			strings,
		))?)
	}

	/// Retrieve the string at `ATTRIBUTE_BASE`
	///
	/// # Errors
	/// * The string at the offset is non UTF-8
	pub fn get_base(&self) -> Result<&str> {
		let strings = ImageStrings(self.image.borrow_index().string_bytes);
		Ok(core::str::from_utf8(self.get_attribute_string(
			attr::ATTRIBUTE_BASE as u4,
			strings,
		))?)
	}

	/// Retrieve the string at `ATTRIBUTE_EXTENSION`
	///
	/// # Errors
	/// * The string at the offset is non UTF-8
	pub fn get_extension(&self) -> Result<&str> {
		let strings = ImageStrings(self.image.borrow_index().string_bytes);
		Ok(core::str::from_utf8(self.get_attribute_string(
			attr::ATTRIBUTE_EXTENSION as u4,
			strings,
		))?)
	}

	/// Retrieve the `ATTRIBUTE_OFFSET`
	pub fn get_content_offset(&self) -> u8 {
		self.get_attribute(attr::ATTRIBUTE_OFFSET as u1)
	}

	/// Retrieve the `ATTRIBUTE_COMPRESSED`
	pub fn get_compressed_size(&self) -> u8 {
		self.get_attribute(attr::ATTRIBUTE_COMPRESSED as u1)
	}

	/// Retrieve the `ATTRIBUTE_UNCOMPRESSED`
	pub fn get_uncompressed_size(&self) -> u8 {
		self.get_attribute(attr::ATTRIBUTE_UNCOMPRESSED as u1)
	}
}
