use crate::globals::{classes, fields};
use crate::objects::array::{ArrayContent, ArrayInstance};
use crate::objects::class_instance::ClassInstance;
use crate::objects::instance::Instance;
use crate::objects::reference::{ClassInstanceRef, Reference};
use crate::symbols::Symbol;

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ptr::slice_from_raw_parts;
use std::sync::{Arc, LazyLock, RwLock};

use byte_slice_cast::{AsByteSlice, AsSliceOf};
use common::int_types::{s1, s2, s4, u1, u2};
use common::traits::PtrType;
use instructions::Operand;
use jni::env::JniEnv;
use jni::sys::jint;

include_generated!("native/java/lang/def/String.definitions.rs");
include_generated!("native/java/lang/def/String.constants.rs");

pub fn intern(_env: JniEnv, this: Reference /* java.lang.String */) -> Reference /* java.lang.String */
{
	if this.is_null() {
		return Reference::null();
	}

	let string = this.extract_class();

	let hash = fields::java_lang_String::hash(string.get());
	let hash_is_zero = fields::java_lang_String::hashIsZero(string.get());
	if hash != 0 || hash_is_zero {
		if let Some(interned_string) = lookup(StringHash(hash as u64)) {
			return Reference::class(interned_string);
		}

		// Otherwise something's off, recompute the hash...
	}

	let coder = fields::java_lang_String::coder(string.get());
	let value_field = fields::java_lang_String::value(string.get()).extract_array();
	let value = value_field.get();

	let value = value.elements.expect_byte();
	let value_unsigned = value.as_byte_slice();

	let computed_hash;
	if value_unsigned.is_empty() {
		fields::java_lang_String::set_hash(string.get_mut(), 0);
		fields::java_lang_String::set_hashIsZero(string.get_mut(), true);
		computed_hash = StringHash(0);
	} else {
		let hash = match coder {
			LATIN1 => <&[u1] as StringHashDerivable<&[u1]>>::hash(&value_unsigned),
			UTF16 => <&[u2] as StringHashDerivable<&[u2]>>::hash(
				&value_unsigned.as_slice_of::<u2>().unwrap(),
			),
			_ => panic!("Invalid string coder `{coder}`"),
		};

		fields::java_lang_String::set_hash(string.get_mut(), hash.0 as jint);
		fields::java_lang_String::set_hashIsZero(string.get_mut(), false);
		computed_hash = hash;
	}

	Reference::class(do_intern(computed_hash, value_unsigned, false))
}

// TODO: This is controlled by a cli argument
// Compact strings are enabled by default
const COMPACT_STRINGS: bool = true;

// TODO: Need to wipe the string pool when the instances fall out of scope
static STRING_POOL: LazyLock<RwLock<HashMap<StringHash, ClassInstanceRef>>> =
	LazyLock::new(|| RwLock::new(HashMap::new()));

fn lookup(hash: StringHash) -> Option<ClassInstanceRef> {
	if let Some((_, entry)) = STRING_POOL
		.read()
		.unwrap()
		.raw_entry()
		.from_hash(hash.0, |k| k == &hash)
	{
		return Some(entry.clone());
	}

	None
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
struct StringHash(u64);

pub trait StringHashDerivable<T> {
	fn hash(value: &T) -> StringHash;
}

impl<'a> StringHashDerivable<&'a str> for &'a str {
	fn hash(value: &Self) -> StringHash {
		let mut h = 0;
		for b in value.chars() {
			h = (31 * h) + (b as u64);
		}
		StringHash(h)
	}
}
impl<'a> StringHashDerivable<&'a [u1]> for &'a [u1] {
	fn hash(value: &Self) -> StringHash {
		let mut h = 0;
		for b in value.iter() {
			h = (31 * h) + (*b as u64);
		}
		StringHash(h)
	}
}
impl<'a> StringHashDerivable<&'a [u2]> for &'a [u2] {
	fn hash(value: &Self) -> StringHash {
		let mut h = 0;
		for b in value.iter() {
			h = (31 * h) + (*b as u64);
		}
		StringHash(h)
	}
}

impl StringHashDerivable<Symbol> for Symbol {
	fn hash(value: &Self) -> StringHash {
		<&str as StringHashDerivable<&str>>::hash(&value.as_str())
	}
}

pub struct StringInterner<T>(T);

impl<T> StringInterner<T>
where
	T: StringHashDerivable<T>,
	T: Into<Symbol>,
{
	pub fn intern(string: T) -> ClassInstanceRef {
		let hash = <T as StringHashDerivable<T>>::hash(&string);

		if let Some(entry) = lookup(hash) {
			return entry;
		}

		let symbol: Symbol = string.into();
		do_intern(hash, symbol.as_bytes(), true)
	}
}

fn do_intern(hash: StringHash, string: &[u8], is_utf8_symbol: bool) -> ClassInstanceRef {
	let mut is_latin1 = false;
	if COMPACT_STRINGS {
		is_latin1 = str_is_latin1(string);
	}

	let encoded_str;
	if is_latin1 {
		let byte_slice: &[s1] = bytemuck::cast_slice(string);
		encoded_str = byte_slice.to_vec().into_boxed_slice();
	} else {
		if is_utf8_symbol {
			// SAFETY:
			// * All `Symbols` are valid UTF-8
			// * &[u8] and &str have the same layout
			let string: &str = unsafe { std::mem::transmute(string) };
			// TODO: More efficient conversion
			let utf16_encoded_bytes = string.encode_utf16().collect::<Box<[u16]>>();
			encoded_str = bytemuck::cast_slice(&utf16_encoded_bytes)
				.to_vec()
				.into_boxed_slice()
		} else {
			// Otherwise, the source is a UTF-16 encoded string (hopefully)
			assert!(string.len() % 2 == 0);
			let byte_slice: &[s1] = bytemuck::cast_slice(string);
			encoded_str = byte_slice.to_vec().into_boxed_slice();
		}
	}

	let new_java_string_instance = ClassInstance::new(classes::java_lang_String());

	// Set `private byte[] value`
	fields::java_lang_String::set_value(
		new_java_string_instance.get_mut(),
		Reference::array(ArrayInstance::new(
			classes::byte_array(),
			ArrayContent::Byte(encoded_str),
		)),
	);

	// Set `private final byte coder`
	let coder = if is_latin1 { LATIN1 } else { UTF16 };
	fields::java_lang_String::set_coder(new_java_string_instance.get_mut(), coder.into());

	// TODO: Make this less of a mess
	let ret = Arc::clone(&new_java_string_instance);
	STRING_POOL
		.write()
		.unwrap()
		.entry(hash)
		.or_insert(new_java_string_instance);
	ret
}

pub fn rust_string_from_java_string(class: ClassInstanceRef) -> String {
	let value = fields::java_lang_String::value(class.get());
	let coder = fields::java_lang_String::coder(class.get());

	let char_array = value.extract_array();
	let chars = char_array.get().elements.expect_byte();

	let unsigned_chars =
		unsafe { &*slice_from_raw_parts(chars.as_ptr().cast::<u8>(), chars.len()) };
	match coder {
		LATIN1 => String::from_utf8_lossy(unsigned_chars).into_owned(),
		UTF16 => String::from_utf16_lossy(unsigned_chars.as_slice_of::<u2>().unwrap()),
		_ => panic!("Invalid string coder `{coder}`"),
	}
}

fn str_is_latin1(string: &[u8]) -> bool {
	let mut prev = 0;
	for byte in string {
		// 0x80 denotes a multibyte sequence, but could also be a valid Latin-1 character
		if *byte == 0x80 && prev <= 0xC3 {
			return false;
		}
		prev = *byte;
	}

	true
}
