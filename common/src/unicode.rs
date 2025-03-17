//! Java modified UTF-8 encoding support

use alloc::borrow::Cow;

#[derive(Debug)]
pub enum Error {
	ContainsNull(usize),
	UnexpectedByte {
		byte: u8,
		index: usize,
		expected: Option<u8>,
	},
	ExpectedContinuation {
		found: u8,
		index: usize,
	},
	UnexpectedEnd,
}

impl core::fmt::Display for Error {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::ContainsNull(index) => write!(f, "string contains null byte at index {index}"),
			Self::UnexpectedByte {
				byte,
				index,
				expected,
			} => {
				if let Some(expected) = expected {
					write!(
						f,
						"unexpected byte 0x{byte:02x} at index {index} (expected 0x{expected:02x})"
					)
				} else {
					write!(f, "unexpected byte 0x{byte:02x} at index {index}")
				}
			},
			Self::ExpectedContinuation { found, index } => {
				write!(
					f,
					"expected continuation byte at index {index}, found 0x{found:02x}"
				)
			},
			Self::UnexpectedEnd => write!(f, "unexpected end of string"),
		}
	}
}

impl core::error::Error for Error {}

/// Encode a UTF-8 string into Java modified UTF-8
///
/// Implemented from the spec [here](https://docs.oracle.com/en/java/javase/23/docs/specs/jni/types.html#modified-utf-8-strings)
///
/// NOTE: This only takes strings, as to not have to do any UTF-8 validation.
pub fn encode(input: &str) -> Cow<'_, [u8]> {
	#[derive(Copy, Clone)]
	enum ReplacementNeeded {
		Null,
		SixByte(u16, u16),
	}

	impl ReplacementNeeded {
		fn write_to(self, buf: &mut Vec<u8>) {
			fn utf8_surrogate_to_mutf8(surrogate: u16) -> [u8; 3] {
				assert!((0xD800..=0xDFFF).contains(&surrogate));
				// 1110xxxx 10xxxxxx 10xxxxxx
				[
					0b11100000 | ((surrogate >> 12) & 0b1111) as u8,
					0b1000_0000 | ((surrogate >> 6) & 0b11_1111) as u8,
					0b1000_0000 | (surrogate & 0b111111) as u8,
				]
			}

			match self {
				ReplacementNeeded::Null => buf.extend([0b11000000, 0b10000000]),
				ReplacementNeeded::SixByte(high, low) => {
					buf.extend(utf8_surrogate_to_mutf8(high));
					buf.extend(utf8_surrogate_to_mutf8(low));
				},
			}
		}

		fn utf8_size(self) -> usize {
			match self {
				ReplacementNeeded::Null => 1,
				ReplacementNeeded::SixByte(..) => 4,
			}
		}
	}

	struct Scanner<'a> {
		input: &'a [u8],
		continue_from: Option<usize>,
	}

	impl<'a> Scanner<'a> {
		fn new(input: &'a [u8]) -> Self {
			Self {
				input,
				continue_from: None,
			}
		}

		fn scan(&mut self) -> (Option<ReplacementNeeded>, &[u8]) {
			if let Some(index) = self.continue_from.take() {
				self.input = &self.input[index..];
			}

			let mut index = 0;
			loop {
				let Some(b) = self.input.get(index).copied() else {
					break;
				};

				match b {
                    // All of the following require no changes:

                    // ASCII, excluding null
                    0x01..=0x7F
                    // 2-byte sequences
                    | 0xC2..=0xDF
                    // 3-byte sequences
                    | 0xE0..=0xEF => {
                        index += 1;
                        continue;
                    }

                    // Nulls are extended to 2 bytes
                    0x00 => return self.fail(ReplacementNeeded::Null, index),

                    // 4-byte sequences are extended to 6 bytes
                    0xF0.. => {
                        // SAFETY: Heavy reliance on the fact that str is valid UTF-8.
                        let c = unsafe {
                            let seq = self.input.get_unchecked(index..index + 4);
                            str::from_utf8_unchecked(seq).chars().next()
                        };

                        let c = (c.unwrap() as u32) - 0x10000;

                        let high = ((c >> 10) as u16) | 0xD800;
                        let low = ((c & 0x3FF) as u16) | 0xDC00;

                        return self.fail(ReplacementNeeded::SixByte(high, low), index);
                    }
                    0x80..=0xC1 => unreachable!("Invalid UTF-8"),
                }
			}

			// Remaining input is valid
			(None, self.input)
		}

		fn fail(
			&mut self,
			replacement_needed: ReplacementNeeded,
			pos: usize,
		) -> (Option<ReplacementNeeded>, &[u8]) {
			self.continue_from = Some(pos + replacement_needed.utf8_size());
			(Some(replacement_needed), &self.input[..pos])
		}
	}

	let mut scanner = Scanner::new(input.as_bytes());

	let (Some(replacement), valid_portion) = scanner.scan() else {
		return Cow::Borrowed(input.as_bytes());
	};

	let mut ret = Vec::with_capacity(input.len() + 2);
	ret.extend(valid_portion);
	replacement.write_to(&mut ret);

	loop {
		match scanner.scan() {
			(None, valid_portion) => {
				ret.extend(valid_portion);
				break;
			},
			(Some(replacement), valid_portion) => {
				ret.extend(valid_portion);
				replacement.write_to(&mut ret);
			},
		}
	}

	Cow::from(ret)
}

/// Decode a Java modified UTF-8 string into a UTF-8 string
pub fn decode(input: &[u8]) -> Result<Cow<'_, str>, Error> {
	#[inline]
	fn verify_contination(b: u8, index: usize) -> Result<(), Error> {
		if b & (!CONT_MASK) != 0b1000_0000 {
			return Err(Error::ExpectedContinuation { found: b, index });
		}

		Ok(())
	}

	if let Ok(s) = str::from_utf8(input) {
		return Ok(Cow::Borrowed(s));
	}

	let mut s = String::with_capacity(input.len());

	let mut i = 0;
	loop {
		let Some(b) = input.get(i).copied() else {
			return Ok(Cow::Owned(s));
		};

		i += 1;

		match b {
			0x00 => return Err(Error::ContainsNull(i)),

			// ASCII, excluding null
			0x01..=0x7F => s.push(b as char),

			// Handle nulls
			0xC0 => {
				let Some(next) = input.get(i).copied() else {
					return Err(Error::UnexpectedEnd);
				};

				verify_contination(next, i)?;
				i += 1;

				if next != 0x80 {
					return Err(Error::UnexpectedByte {
						byte: next,
						index: i,
						expected: Some(0x80),
					});
				}

				s.push('\0');
			},

			b => {
				let width = utf8_char_width(b);
				let Some(next) = input.get(i).copied() else {
					return Err(Error::UnexpectedEnd);
				};

				verify_contination(next, i)?;
				i += 1;

				match width {
					2 => {
						s.extend([b as char, next as char]);
					},
					3 => {
						let Some(next2) = input.get(i).copied() else {
							return Err(Error::UnexpectedEnd);
						};

						verify_contination(next2, i)?;
						i += 1;

						match (b, next) {
							// Valid UTF-8, nothing extra to do
							(0xE0, 0xA0..=0xBF)
							| (0xE1..=0xEC, 0x80..=0xBF)
							| (0xED, 0x80..=0x9F)
							| (0xEE..=0xEF, 0x80..=0xBF) => s.extend([b as char, next as char, next2 as char]),
							(0xED, 0xA0..=0xAF) => {
								let Some(next3) = input.get(i).copied() else {
									return Err(Error::UnexpectedEnd);
								};

								if next3 != 0xED {
									return Err(Error::UnexpectedByte {
										byte: next3,
										index: i,
										expected: Some(0xED),
									});
								}

								i += 1;

								let Some(next4) = input.get(i).copied() else {
									return Err(Error::UnexpectedEnd);
								};

								verify_contination(next4, i)?;
								i += 1;

								if !(0xB0..=0xBF).contains(&next4) {
									return Err(Error::UnexpectedByte {
										byte: next4,
										index: i,
										expected: None,
									});
								}

								let Some(next5) = input.get(i).copied() else {
									return Err(Error::UnexpectedEnd);
								};

								verify_contination(next5, i)?;
								i += 1;

								let high =
									mutf8_surrogate_to_utf8(((next as u16) << 8) | next2 as u16);
								let low =
									mutf8_surrogate_to_utf8(((next4 as u16) << 8) | next5 as u16);

								let c = 0x10000 + (high - 0xD800) * 0x400 + (low - 0xDC00);
								assert!((0x10000..=0x10FFFF).contains(&c));

								// SAFETY: We just verified that this is a valid UTF-8 surrogate pair
								let v = unsafe { s.as_mut_vec() };
								v.extend([
									0b1111_0000u8 | ((c >> 18) & 0b111) as u8,
									0b1000_0000 | ((c >> 12) & 0b11_1111) as u8,
									0b1000_0000 | ((c >> 6) & 0b11_1111) as u8,
									0b1000_0000 | (c & 0b11_1111) as u8,
								])
							},
							_ => {
								return Err(Error::UnexpectedByte {
									byte: next,
									index: i,
									expected: None,
								})
							},
						}
					},
					_ => {
						return Err(Error::UnexpectedByte {
							byte: b,
							index: i,
							expected: None,
						})
					},
				}
			},
		}
	}
}

/// Converts a modified UTF-8 surrogate pair into a UTF-8 code point
#[inline]
fn mutf8_surrogate_to_utf8(surrogate: u16) -> u32 {
	0xD000u32
		| (((surrogate >> 8) & CONT_MASK as u16) as u32) << 6
		| (surrogate & CONT_MASK as u16) as u32
}

// https://github.com/rust-lang/rust/blob/master/library/core/src/str/validations.rs#L254
// https://tools.ietf.org/html/rfc3629
const UTF8_CHAR_WIDTH: &[u8; 256] = &[
	// 1  2  3  4  5  6  7  8  9  A  B  C  D  E  F
	1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 0
	1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 1
	1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 2
	1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 3
	1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 4
	1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 5
	1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 6
	1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, // 7
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 8
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 9
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // A
	0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // B
	0, 0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // C
	2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, // D
	3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, // E
	4, 4, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // F
];

/// Given a first byte, determines how many bytes are in this UTF-8 character.
#[inline]
pub const fn utf8_char_width(b: u8) -> usize {
	UTF8_CHAR_WIDTH[b as usize] as usize
}

const CONT_MASK: u8 = 0b0011_1111;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn with_nulls() {
		let s = "hello\0world";
		let encoded = encode(s);

		assert_eq!(*encoded, *b"hello\xC0\x80world");

		let decoded = decode(&encoded).unwrap();
		assert_eq!(decoded, s);
	}

	#[test]
	fn with_three_byte() {
		let s = "hello\u{0800}world";
		let encoded = encode(s);

		assert_eq!(*encoded, *b"hello\xE0\xA0\x80world");

		let decoded = decode(&encoded).unwrap();
		assert_eq!(decoded, s);
	}

	#[test]
	fn with_four_byte() {
		let s = "hello\u{10000}world";
		let encoded = encode(s);

		assert_eq!(*encoded, *b"hello\xED\xA0\x80\xED\xB0\x80world");

		let decoded = decode(&encoded).unwrap();

		assert_eq!(decoded, s);
	}
}
