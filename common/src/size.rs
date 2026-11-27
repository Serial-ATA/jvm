/// Utilities for parsing byte counts in multiple units
use std::str::FromStr;

/// A byte size, parsed from a unit string (e.g., `64m`)
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct ByteSize(pub usize);

pub enum Unit {
	Bytes,
	Kilobytes,
	Megabytes,
	Gigabytes,
}

impl Unit {
	pub fn scale(self) -> usize {
		match self {
			Unit::Bytes => 1,
			Unit::Kilobytes => 1000,
			Unit::Megabytes => 1000000,
			Unit::Gigabytes => 1000000000,
		}
	}
}

impl FromStr for ByteSize {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let unit = match s.chars().last() {
			Some('k' | 'K') => Unit::Kilobytes,
			Some('m' | 'M') => Unit::Megabytes,
			Some('g' | 'G') => Unit::Gigabytes,
			Some(c) if c.is_ascii_digit() => Unit::Bytes,
			_ => return Err(()),
		};

		// Safe to slice, the unit char was verified above
		let digits = &s[..s.len() - 1];
		if !digits.chars().all(|c| c.is_ascii_digit()) {
			return Err(());
		};

		let count = digits
			.as_bytes()
			.iter()
			.fold(0_usize, |product, c| product * 10 + usize::from(*c - b'0'));

		Ok(ByteSize(count.saturating_mul(unit.scale())))
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn parses_byte_sizes() {
		for (raw, bytes) in [("1", 1), ("1k", 1000), ("1m", 1000000), ("1g", 1000000000)] {
			let size = ByteSize::from_str(raw).unwrap();
			assert_eq!(size.0, bytes);
		}
	}
}
