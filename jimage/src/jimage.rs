use crate::{ImageStrings, JImageLocation};

use common::int_types::{u1, u4, u8};

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
	pub data: Vec<u1>,                       // The entire JImage's data
	#[borrows(data)]
	#[covariant]
	pub index: crate::JImageIndex<'this>, // Information related to resource lookup
}

impl JImage {
	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.hpp#L545
	/// Return location attribute stream at offset.
	#[inline(always)]
	pub fn get_location_offset_data(&self, offset: u4) -> Option<&[u1]> {
		assert!(
			offset < self.borrow_header().locations_size,
			"offset exceeds location attributes size"
		);

		if offset != 0 {
			return Some(&self.borrow_index().location_bytes[offset as usize..]);
		}

		None
	}

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.hpp#L552
	/// Return location attribute stream for location i.
	#[inline(always)]
	pub fn get_location_data(&self, index: u4) -> Option<&[u1]> {
		self.get_location_offset_data(self.get_location_offset(index))
	}

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.hpp#L557
	/// Return the location offset for index
	#[inline(always)]
	pub fn get_location_offset(&self, index: u4) -> u4 {
		assert!(
			index < self.borrow_header().table_length,
			"index exceeds location count"
		);
		self.borrow_index().offsets_table[index as usize]
	}

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.cpp#L447
	/// Find the location attributes associated with the path.
	/// Returns true if the location is found, false otherwise.
	pub fn find_location(&self, path: &str, location: &mut JImageLocation) -> bool {
		// Locate the entry in the index perfect hash table.
		let index = ImageStrings::find(
			*self.borrow_endian(),
			path,
			self.borrow_index().redirects_table,
		);

		// If is found.
		if let Some(index) = index {
			// Get address of first byte of location attribute stream.
			let data = self.get_location_data(index as u4);

			// Expand location attributes.
			if let Some(data) = data {
				location.set_data(data);
			}

			// Make sure result is not a false positive.
			return self.verify_location(location, path);
		}

		false
	}

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.cpp#L464
	/// Find the location index and size associated with the path.
	/// Returns the location index and size if the location is found, `None` otherwise.
	pub fn find_location_index(&self, path: &str, size: &mut u8) -> Option<u4> {
		// Locate the entry in the index perfect hash table.
		let index = ImageStrings::find(
			*self.borrow_endian(),
			path,
			self.borrow_index().redirects_table,
		);

		// If found.
		if let Some(index) = index {
			// Get address of first byte of location attribute stream.
			let offset = self.get_location_offset(index as u4);
			let data = self.get_location_offset_data(offset);

			// Expand location attributes.
			let location;
			if let Some(data) = data {
				location = JImageLocation::new_with_data(data);
			} else {
				location = JImageLocation::new();
			}

			// Make sure result is not a false positive.
			if self.verify_location(&location, path) {
				*size = location.get_attribute(JImageLocation::ATTRIBUTE_UNCOMPRESSED as u1);
				return Some(offset);
			}
		}

		// not found
		None
	}

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.cpp#L484
	/// Verify that a found location matches the supplied path.
	#[rustfmt::skip]
	pub fn verify_location(&self, location: &JImageLocation, path: &str) -> bool {
		// Manage the image string table.
		let strings = ImageStrings(self.borrow_index().string_bytes);

		// Get module name string.
		let module =
			location.get_attribute_string(JImageLocation::ATTRIBUTE_MODULE as u4, &strings);

		let mut path_iter = path.bytes();

		// If module string is not empty.
		if !module.is_empty() {
			// Compare '/module/'
			if path_iter.next() != Some(b'/') { return false; }
			if !path_iter.by_ref().eq(module.iter().copied()) { return false; }
			if path_iter.next() != Some(b'/') { return false; }
		}

		// Get parent (package) string
		let parent =
			location.get_attribute_string(JImageLocation::ATTRIBUTE_PARENT as u4, &strings);

		// If parent string is not empty string.
		if !parent.is_empty() {
			// Compare 'parent/'
			if !path_iter.by_ref().eq(parent.iter().copied()) { return false; }
			if path_iter.next() != Some(b'/') { return false; }
		}

		// Get base name string.
		let base = location.get_attribute_string(JImageLocation::ATTRIBUTE_BASE as u4, &strings);

		// Compare with base name.
		if !path_iter.by_ref().eq(base.iter().copied()) { return false; }

		// Get extension string.
		let extension = location.get_attribute_string(JImageLocation::ATTRIBUTE_EXTENSION as u4, &strings);

		// If extension is not empty.
		if !extension.is_empty() {
			// Compare '.extension'
			if path_iter.next() != Some(b'.') { return false; }
			if !path_iter.by_ref().eq(extension.iter().copied()) { return false; }
		}

		// True only if complete match and no more characters.
		path_iter.next().is_none()
	}
}
