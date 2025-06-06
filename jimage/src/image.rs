use crate::error::Result;
use crate::{ImageStrings, JImageHeader, JImageLocation};

use common::box_slice;
use common::endian::Endian;
use common::int_types::{u1, u4, u8};
use std::io::Read;

#[derive(Debug)]
pub struct JImage {
	pub(crate) endian: Endian,            // Endian handler
	pub(crate) header: JImageHeader,      // Image header
	pub(crate) index: crate::JImageIndex, // Information related to resource lookup
	pub(crate) resources: Box<[u1]>,      // The actual resource data
}

impl JImage {
	pub fn resources(&self) -> ResourceIter<'_> {
		ResourceIter {
			image: self,
			max: self.header.table_length() as u4,
			pos: 0,
		}
	}
}

pub struct Resource<'a> {
	module: &'a str,
	version: &'a str,
	parent: Option<&'a str>,
	base: Option<&'a str>,
	extension: Option<&'a str>,
}

pub struct ResourceIter<'a> {
	image: &'a JImage,
	max: u4,
	pos: u4,
}

impl<'a> Iterator for ResourceIter<'a> {
	type Item = Resource<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		loop {
			if self.pos >= self.max {
				return None;
			}

			let cur = self.pos;
			self.pos += 1;

			let Some(location_data) = self.image.get_location_data(cur) else {
				break;
			};

			let location = JImageLocation::new_with_data(self.image, location_data);
			let Some(Ok(module)) = location.module() else {
				continue;
			};

			if module == "modules" || module == "packages" {
				continue;
			}
		}

		None
	}
}

impl JImage {
	pub fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
		crate::parse::parse(reader)
	}

	pub fn header(&self) -> JImageHeader {
		self.header
	}

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.cpp#L447
	/// Find the location attributes associated with the path.
	pub fn find_location(&self, path: &str) -> Option<JImageLocation<'_>> {
		// Locate the entry in the index perfect hash table.
		let index = ImageStrings::find(self.endian, path, self.index.redirects_table());

		// If is found.
		if let Some(index) = index {
			// Get address of first byte of location attribute stream.
			let data = self.get_location_data(index as u4);

			// Expand location attributes.
			let location = JImageLocation::new_opt_(self, data);

			// Make sure result is not a false positive.
			if self.verify_location(&location, path) {
				return Some(location);
			}
		}

		None
	}

	// TODO: https://github.com/openjdk/jdk/blob/62a033ecd7058f4a4354ebdcd667b3d7991e1f3d/src/java.base/share/native/libjimage/jimage.cpp#L102
	pub fn find_resource(&self, module_name: &str, name: &str) -> Option<(u4, u8)> {
		// TBD: assert!(module_name.len() > 0, "module name must be non-empty");
		if name.is_empty() {
			// `name` must be non-empty
			//
			// libjimage makes this an assertion, doesn't really seem necessary.
			return None;
		}

		let fullpath = format!("/{}/{}", module_name, name);
		self.find_location_index(&fullpath)
	}

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.cpp#L523
	/// Return the resource for the supplied location offset.
	pub fn get_resource(&self, offset: u4) -> Result<Box<[u1]>> {
		// Get address of first byte of location attribute stream.
		let data = self.get_location_offset_data(offset);
		// Expand location attributes.
		let location = JImageLocation::new_opt_(self, data);
		// Read the data
		let uncompressed_data = self.get_resource_from_location(&location)?;

		Ok(uncompressed_data)
	}

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.cpp#L533
	/// Return the resource for the supplied location.
	pub fn get_resource_from_location(&self, location: &JImageLocation<'_>) -> Result<Box<[u1]>> {
		// Retrieve the byte offset and size of the resource.
		let offset = location.content_offset() as usize;
		let uncompressed_size = location.uncompressed_size();
		let compressed_size = location.compressed_size();

		let mut uncompressed_data = box_slice![0u8; uncompressed_size];

		// If the resource is not compressed.
		if compressed_size == 0 {
			// Read bytes from offset beyond the image index.
			let mut data = &self.resources[offset..offset + uncompressed_size as usize];
			assert!(
				(&mut data).read_exact(&mut *uncompressed_data).is_ok(),
				"error reading from image or short read"
			);
			return Ok(uncompressed_data);
		}

		// We have to decompress the data
		let mut compressed_data = &self.resources[offset..offset + compressed_size as usize];
		// Get image string table.
		let strings = ImageStrings(self.index.string_bytes());
		// Decompress resource.
		super::decompressor::decompress_resource(
			&mut compressed_data,
			&mut *uncompressed_data,
			uncompressed_size,
			strings,
			self.endian,
		)?;

		Ok(uncompressed_data)
	}

	/// Return a sorted collection of all paths to valid locations
	///
	/// # Errors
	/// * A location has a non UTF-8 attribute
	pub fn get_entry_names(&self) -> Result<Vec<String>> {
		let offsets = self.index.offsets_table();

		let mut names = Vec::with_capacity(offsets.len());
		for offset in offsets.iter().copied() {
			if offset > 0 {
				let data = self.get_location_offset_data(offset);
				let location = JImageLocation::new_opt_(self, data);
				let name = location.full_name(false)?;
				names.push(name);
			}
		}

		names.sort();
		Ok(names)
	}

	pub fn is_tree_info_resource(path: &str) -> bool {
		path.starts_with("/packages") || path.starts_with("/modules")
	}
}

impl JImage {
	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.hpp#L545
	/// Return location attribute stream at offset.
	#[inline(always)]
	fn get_location_offset_data(&self, offset: u4) -> Option<&[u1]> {
		assert!(
			offset < self.header.location_table_length(),
			"offset exceeds location attributes size"
		);

		if offset != 0 {
			return Some(&self.index.location_bytes()[offset as usize..]);
		}

		None
	}

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.hpp#L552
	/// Return location attribute stream for location i.
	#[inline(always)]
	fn get_location_data(&self, index: u4) -> Option<&[u1]> {
		self.get_location_offset_data(self.get_location_offset(index))
	}

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.hpp#L557
	/// Return the location offset for index
	#[inline(always)]
	fn get_location_offset(&self, index: u4) -> u4 {
		assert!(
			index < self.header.table_length() as u4,
			"index exceeds location count"
		);
		self.index.offsets_table()[index as usize]
	}

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.cpp#L464
	/// Find the location index and size associated with the path.
	/// Returns the location index and size if the location is found, `None` otherwise.
	fn find_location_index(&self, path: &str) -> Option<(u4, u8)> {
		// Locate the entry in the index perfect hash table.
		let index = ImageStrings::find(self.endian, path, self.index.redirects_table());

		// If found.
		if let Some(index) = index {
			// Get address of first byte of location attribute stream.
			let offset = self.get_location_offset(index as u4);
			let data = self.get_location_offset_data(offset);

			// Expand location attributes.
			let location = JImageLocation::new_opt_(self, data);

			// Make sure result is not a false positive.
			if self.verify_location(&location, path) {
				let size = location.uncompressed_size();
				return Some((offset, size));
			}
		}

		// not found
		None
	}

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.cpp#L484
	/// Verify that a found location matches the supplied path.
    #[rustfmt::skip]
	fn verify_location(&self, location: &JImageLocation<'_>, path: &str) -> bool {
		let mut path_iter = path.bytes();

		// If module string is not empty.
		if let Some(module) = location.module() {
			let Ok(module) = module else {
				return false
			};

			// Compare '/module/'
			if path_iter.next() != Some(b'/') { return false; }
			if !path_iter.by_ref().take(module.len()).eq(module.as_bytes().iter().copied()) { return false; }
			if path_iter.next() != Some(b'/') { return false; }
		}

		// If parent string is not empty string.
		if let Some(parent) = location.parent() {
			let Ok(parent) = parent else {
				return false
			};

			// Compare 'parent/'
			if !path_iter.by_ref().take(parent.len()).eq(parent.as_bytes().iter().copied()) { return false; }
			if path_iter.next() != Some(b'/') { return false; }
		}

		// Compare with base name.
		if let Some(base) = location.base() {
			let Ok(base) = base else {
				return false
			};

			if !path_iter.by_ref().take(base.len()).eq(base.as_bytes().iter().copied()) { return false; }
		}

		// If extension is not empty.
		if let Some(extension) = location.extension() {
			let Ok(extension) = extension else {
				return false
			};

			// Compare '.extension'
			if path_iter.next() != Some(b'.') { return false; }
			if !path_iter.by_ref().take(extension.len()).eq(extension.as_bytes().iter().copied()) { return false; }
		}

		// True only if complete match and no more characters.
		path_iter.next().is_none()
	}
}
