#[cfg(test)]
mod tests;

use std::cmp::Ordering;

use common::endian::Endian;
use common::int_types::{s4, u1, u4};

#[derive(Copy, Clone)]
pub struct ImageStrings<'a>(pub &'a [u1]);

impl<'a> ImageStrings<'a> {
	pub const HASH_MULTIPLIER: s4 = 0x0100_0193;
	const POSITIVE_MASK: u4 = 0x7FFF_FFFF;

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.hpp#L168
	/// Return the UTF-8 string beginning at offset.
	pub fn get(&self, offset: u4) -> &'a [u1] {
		assert!(
			(offset as usize) < self.0.len(),
			"offset exceeds string table size"
		);
		let string_at_offset = &self.0[offset as usize..];
		let terminator_pos = string_at_offset
			.iter()
			.copied()
			.position(|b| b == 0)
			.unwrap();
		&string_at_offset[..terminator_pos]
	}

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.cpp#L59
	pub fn hash_code(string: &str, seed: s4) -> s4 {
		assert!(seed > 0, "invariant");

		let mut useed = seed as u4;
		for byte in string.bytes() {
			useed = (useed.wrapping_mul(Self::HASH_MULTIPLIER as u4)) ^ u4::from(byte);
		}

		(useed & Self::POSITIVE_MASK) as s4
	}

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.cpp#L75
	// TODO: Use endian when endian switching is implemented
	pub fn find(_endian: Endian, name: &str, redirect: &[s4]) -> Option<s4> {
		// If the table is empty, then short cut.
		if redirect.is_empty() {
			return None;
		}

		// Compute the basic perfect hash for name.
		let mut hash_code = Self::hash_code(name, Self::HASH_MULTIPLIER);

		// Modulo table size.
		let index = hash_code % redirect.len() as s4;

		// Get redirect entry.
		//   value == 0 then not found
		//   value < 0 then -1 - value is true index
		//   value > 0 then value is seed for recomputing hash.
		let value = redirect[index as usize];

		// if recompute is required.
		match value.cmp(&0) {
			Ordering::Greater => {
				// Entry collision value, need to recompute hash.
				hash_code = ImageStrings::hash_code(name, value);
				// Modulo table size.
				Some(hash_code % redirect.len() as s4)
			},
			// Compute direct index.
			Ordering::Less => Some(-1 - value),
			// No entry found.
			Ordering::Equal => None,
		}
	}
}
