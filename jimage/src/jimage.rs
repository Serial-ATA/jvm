use crate::{ImageStrings, JImageHeader, JImageLocation};

use std::io::Read;

use common::endian::Endian;
use common::int_types::{u1, u4, u8};

#[ouroboros::self_referencing(pub_extras)]
#[derive(Debug)]
pub struct JImage {
	pub endian: Endian,       // Endian handler
	pub header: JImageHeader, // Image header
	pub data: Vec<u1>,        // The entire JImage's data
	#[borrows(data)]
	#[covariant]
	pub index: crate::JImageIndex<'this>, // Information related to resource lookup
}

impl JImage {
	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.hpp#LL436-L440C6
	/// Compute number of bytes in image file index.
	#[inline(always)]
	pub fn index_size(&self) -> usize {
		let header = self.borrow_header();
		core::mem::size_of::<JImageHeader>()
			+ header.table_length() * 2
			+ header.locations_size as usize
			+ header.strings_size as usize
	}

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.hpp#L504
	/// Return first address of resource data.
	#[inline(always)]
	pub fn get_data_address(&self) -> usize {
		self.index_size()
	}

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
			let location = JImageLocation::new_opt_(data);

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

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.cpp#L523
	/// Return the resource for the supplied location offset.
	pub fn get_resource(&self, offset: u4, uncompressed_data: &mut [u1]) {
		// Get address of first byte of location attribute stream.
		let data = self.get_location_offset_data(offset);
		// Expand location attributes.
		let location = JImageLocation::new_opt_(data);
		// Read the data
		self.get_resource_from_location(&location, uncompressed_data);
	}

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.cpp#L533
	/// Return the resource for the supplied location.
	pub fn get_resource_from_location(
		&self,
		location: &JImageLocation,
		uncompressed_data: &mut [u1],
	) {
		// Retrieve the byte offset and size of the resource.
		let offset = location.get_attribute(JImageLocation::ATTRIBUTE_OFFSET as u1);
		let uncompressed_size =
			location.get_attribute(JImageLocation::ATTRIBUTE_UNCOMPRESSED as u1);
		let compressed_size = location.get_attribute(JImageLocation::ATTRIBUTE_COMPRESSED as u1);

		let data_start = self.get_data_address() + offset as usize;

		// If the resource is not compressed.
		if compressed_size == 0 {
			// Read bytes from offset beyond the image index.
			let mut data = &self.borrow_data()[data_start..data_start + uncompressed_size as usize];
			assert!(
				(&mut data).read_exact(uncompressed_data).is_ok(),
				"error reading from image or short read"
			);
			return;
		}

		// We have to decompress the data
		let mut compressed_data = Box::<[u1]>::from(
			&self.borrow_data()[data_start..data_start + compressed_size as usize],
		);
		// Get image string table.
		let strings = ImageStrings(self.borrow_index().string_bytes);
		// Decompress resource.
		super::decompressor::decompress_resource(
			&mut compressed_data,
			uncompressed_data,
			uncompressed_size,
			strings,
			Endian::Little, // TODO
		);
	}
}
