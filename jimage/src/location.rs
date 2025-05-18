use crate::error::Result;
use crate::{ImageStrings, JImage};

use std::fmt::{Debug, Formatter};

use common::int_types::{u1, u4, u8};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AttributeKind {
	/// End of attribute stream marker
	End = 0,
	/// String table offset of module name
	Module = 1,
	/// String table offset of resource path parent
	Parent = 2,
	/// String table offset of resource path base
	Base = 3,
	/// String table offset of resource path extension
	Extension = 4,
	/// Container byte offset of resource
	Offset = 5,
	/// In image byte size of the compressed resource
	Compressed = 6,
	/// In memory byte size of the uncompressed resource
	Uncompressed = 7,
}

impl AttributeKind {
	const VARIANTS_COUNT: usize = 8;
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
struct Attribute(u1);

impl Attribute {
	const NULL: Self = Self(0);

	pub fn kind(&self) -> AttributeKind {
		match self.0 >> 3 {
			0 => AttributeKind::End,
			1 => AttributeKind::Module,
			2 => AttributeKind::Parent,
			3 => AttributeKind::Base,
			4 => AttributeKind::Extension,
			5 => AttributeKind::Offset,
			6 => AttributeKind::Compressed,
			7 => AttributeKind::Uncompressed,
			kind => unreachable!("Invalid JImage attribute kind: {kind}"),
		}
	}

	pub fn len(&self) -> u1 {
		(self.0 & 0x7) + 1
	}

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.hpp#L264
	/// Return the attribute length.
	#[inline(always)]
	fn value_from(&self, data: &[u1]) -> u8 {
		let mut value = 0;
		for i in 0..self.len() {
			value <<= 8;
			value |= u8::from(data[i as usize]);
		}

		value
	}
}

// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.hpp#L233
pub struct JImageLocation<'a> {
	attribute_values: [u8; AttributeKind::VARIANTS_COUNT],
	image: &'a JImage,
}

impl<'a> Debug for JImageLocation<'a> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("JImageLocation")
			.field("attributes", &self.attribute_values)
			.finish()
	}
}

impl<'a> JImageLocation<'a> {
	pub fn new(image: &'a JImage) -> Self {
		Self {
			attribute_values: [0; AttributeKind::VARIANTS_COUNT],
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
	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.hpp#L300
	/// Retrieve an attribute string value from the inflated array.
	#[inline(always)]
	pub fn get_attribute_string(
		&self,
		kind: AttributeKind,
		strings: ImageStrings<'a>,
	) -> Option<&'a [u1]> {
		let string_offset = self.attribute_value(kind) as u4;
		if string_offset == 0 {
			return None;
		}

		Some(strings.get(string_offset))
	}

	/// Retrieve the full name of the location
	///
	/// # Errors
	/// * The location contains a non UTF-8 attribute
	pub fn full_name(&self, modules_prefix: bool) -> Result<String> {
		let mut name = String::new();

		if let Some(module) = self.module() {
			let module_str = module?;

			if modules_prefix {
				name.push_str("/modules");
			}

			name.push('/');
			name.push_str(module_str);
			name.push('/');
		}

		if let Some(parent) = self.parent() {
			name.push_str(parent?);
			name.push('/');
		}

		if let Some(base) = self.base() {
			name.push_str(base?);
		}

		if let Some(extension) = self.extension() {
			name.push('.');
			name.push_str(extension?);
		}

		Ok(name)
	}

	/// Retrieve the string at `ATTRIBUTE_MODULE`
	///
	/// # Errors
	/// * The string at the offset is non UTF-8
	pub fn module(&self) -> Option<Result<&str>> {
		let strings = ImageStrings(self.image.index.string_bytes());

		let string = self.get_attribute_string(AttributeKind::Module, strings)?;
		match core::str::from_utf8(string) {
			Ok(s) => Some(Ok(s)),
			Err(e) => Some(Err(e.into())),
		}
	}

	/// Retrieve the string at `ATTRIBUTE_PARENT`
	///
	/// # Errors
	/// * The string at the offset is non UTF-8
	pub fn parent(&self) -> Option<Result<&str>> {
		let strings = ImageStrings(self.image.index.string_bytes());

		let string = self.get_attribute_string(AttributeKind::Parent, strings)?;
		match core::str::from_utf8(string) {
			Ok(s) => Some(Ok(s)),
			Err(e) => Some(Err(e.into())),
		}
	}

	/// Retrieve the string at `ATTRIBUTE_BASE`
	///
	/// # Errors
	/// * The string at the offset is non UTF-8
	pub fn base(&self) -> Option<Result<&str>> {
		let strings = ImageStrings(self.image.index.string_bytes());

		let string = self.get_attribute_string(AttributeKind::Base, strings)?;
		match core::str::from_utf8(string) {
			Ok(s) => Some(Ok(s)),
			Err(e) => Some(Err(e.into())),
		}
	}

	/// Retrieve the string at `ATTRIBUTE_EXTENSION`
	///
	/// # Errors
	/// * The string at the offset is non UTF-8
	pub fn extension(&self) -> Option<Result<&str>> {
		let strings = ImageStrings(self.image.index.string_bytes());

		let string = self.get_attribute_string(AttributeKind::Extension, strings)?;
		match core::str::from_utf8(string) {
			Ok(s) => Some(Ok(s)),
			Err(e) => Some(Err(e.into())),
		}
	}

	/// Retrieve the `ATTRIBUTE_OFFSET`
	pub fn content_offset(&self) -> u8 {
		self.attribute_value(AttributeKind::Offset)
	}

	/// Retrieve the `ATTRIBUTE_COMPRESSED`
	pub fn compressed_size(&self) -> u8 {
		self.attribute_value(AttributeKind::Compressed)
	}

	/// Retrieve the `ATTRIBUTE_UNCOMPRESSED`
	pub fn uncompressed_size(&self) -> u8 {
		self.attribute_value(AttributeKind::Uncompressed)
	}
}

impl JImageLocation<'_> {
	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.cpp#L125
	/// Inflates the attribute stream into individual values stored in the long
	/// array _attributes. This allows an attribute value to be quickly accessed by
	/// direct indexing.  Unspecified values default to zero (from constructor.)
	fn set_data(&mut self, data: &[u1]) {
		// Deflate the attribute stream into an array of attributes.
		// Repeat until end header is found.
		let mut i = 0;
		loop {
			// Extract kind from header byte.
			let header_byte = data[i];
			let attr = Attribute(header_byte);
			if attr.kind() == AttributeKind::End {
				return;
			}

			// Read value (most significant first.)
			self.attribute_values[attr.kind() as usize] = attr.value_from(&data[i + 1..]);
			i += attr.len() as usize + 1;
		}
	}

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.hpp#L294
	/// Retrieve an attribute value from the inflated array.
	#[inline(always)]
	fn attribute_value(&self, kind: AttributeKind) -> u8 {
		self.attribute_values[kind as usize]
	}
}
