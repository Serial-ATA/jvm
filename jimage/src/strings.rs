use crate::Endian;

use std::cmp::Ordering;

use common::int_types::{s4, u4};

pub struct ImageStrings;

impl ImageStrings {
	const HASH_MULTIPLIER: s4 = 0x0100_0193;

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.cpp#L59
	pub fn hash_code(string: &str, seed: s4) -> s4 {
		assert!(seed > 0, "invariant");

		let mut useed = seed as u4;
		for byte in string.bytes() {
			useed = (useed * Self::HASH_MULTIPLIER as u4) ^ u4::from(byte);
		}

		(useed & 0x07FF_FFFF) as s4
	}

	// https://github.com/openjdk/jdk/blob/f56285c3613bb127e22f544bd4b461a0584e9d2a/src/java.base/share/native/libjimage/imageFile.cpp#L75
	// TODO: Use endian when endian switching is implemented
	pub fn find(_endian: Endian, name: &str, redirect: &[s4]) -> Option<s4> {
		// If the table is empty, then short cut.
		let length = redirect.len() as u4;
		if length == 0 {
			return None;
		}

		// Compute the basic perfect hash for name.
		let mut hash_code = Self::hash_code(name, Self::HASH_MULTIPLIER) as u4;

		// Modulo table size.
		let index = hash_code % length;

		// Get redirect entry.
		//   value == 0 then not found
		//   value < 0 then -1 - value is true index
		//   value > 0 then value is seed for recomputing hash.
		let value = redirect[index as usize];

		// if recompute is required.
		match value.cmp(&0) {
			Ordering::Greater => {
				// Entry collision value, need to recompute hash.
				hash_code = ImageStrings::hash_code(name, value) as u4;
				// Modulo table size.
				Some((hash_code % length) as s4)
			},
			// Compute direct index.
			Ordering::Less => Some(-1 - value),
			// No entry found.
			Ordering::Equal => None,
		}
	}
}
