use crate::classes;
use crate::objects::reference::{ClassInstanceRef, Reference};
use crate::symbols::Symbol;

use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{LazyLock, RwLock};

use byte_slice_cast::{AsByteSlice, AsSliceOf};
use common::int_types::{u1, u2};
use common::traits::PtrType;
use jni::env::JniEnv;
use jni::sys::{jbyte, jint};

include_generated!("native/java/lang/def/String.definitions.rs");
include_generated!("native/java/lang/def/String.constants.rs");

pub fn intern(_env: JniEnv, this: Reference /* java.lang.String */) -> Reference /* java.lang.String */
{
	if this.is_null() {
		return Reference::null();
	}

	let string = this.extract_class();

	let hash = classes::java::lang::String::hash(string.get());
	let hash_is_zero = classes::java::lang::String::hashIsZero(string.get());
	if hash != 0 || hash_is_zero {
		if let Some(interned_string) = lookup(StringHash(hash)) {
			return Reference::class(interned_string);
		}

		// Otherwise something's off, recompute the hash...
	}

	let coder = classes::java::lang::String::coder(string.get());
	let value_field = classes::java::lang::String::value(string.get());
	let value = value_field.get();

	let value = value.as_slice::<jbyte>();
	let value_unsigned = value.as_byte_slice();

	let computed_hash;
	if value_unsigned.is_empty() {
		classes::java::lang::String::set_hash(string.get_mut(), 0);
		classes::java::lang::String::set_hashIsZero(string.get_mut(), true);
		computed_hash = StringHash(0);
	} else {
		let hash = match coder {
			LATIN1 => <&[u1] as StringHashDerivable<&[u1]>>::hash(&value_unsigned),
			UTF16 => <&[u2] as StringHashDerivable<&[u2]>>::hash(
				&value_unsigned.as_slice_of::<u2>().unwrap(),
			),
			_ => panic!("Invalid string coder `{coder}`"),
		};

		classes::java::lang::String::set_hash(string.get_mut(), hash.0 as jint);
		classes::java::lang::String::set_hashIsZero(string.get_mut(), false);
		computed_hash = hash;
	}

	Reference::class(do_intern(computed_hash, string))
}

// TODO: This is controlled by a cli argument
// Compact strings are enabled by default
pub(crate) const COMPACT_STRINGS: bool = true;

// TODO: Need to wipe the string pool when the instances fall out of scope
static STRING_POOL: LazyLock<RwLock<HashMap<StringHash, ClassInstanceRef>>> =
	LazyLock::new(|| RwLock::new(HashMap::new()));

fn lookup(hash: StringHash) -> Option<ClassInstanceRef> {
	if let Some(entry) = STRING_POOL.read().unwrap().get(&hash) {
		return Some(entry.clone());
	}

	None
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct StringHash(i32);

pub trait StringHashDerivable<T> {
	fn hash(value: &T) -> StringHash;
}

impl<'a> StringHashDerivable<&'a str> for &'a str {
	fn hash(value: &Self) -> StringHash {
		let mut h = 0;
		for b in value.as_bytes() {
			h = (31_u32.wrapping_mul(h)) + (*b as u32);
		}
		StringHash(h as i32)
	}
}
impl<'a> StringHashDerivable<&'a [u1]> for &'a [u1] {
	fn hash(value: &Self) -> StringHash {
		let mut h = 0;
		for b in value.iter() {
			h = (31_u32.wrapping_mul(h)) + (*b as u32);
		}
		StringHash(h as i32)
	}
}
impl<'a> StringHashDerivable<&'a [u2]> for &'a [u2] {
	fn hash(value: &Self) -> StringHash {
		let mut h = 0;
		for b in value.iter() {
			h = (31_u32.wrapping_mul(h)) + (*b as u32);
		}
		StringHash(h as i32)
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
		let string = classes::java::lang::String::new(symbol);
		do_intern(hash, string)
	}
}

fn do_intern(hash: StringHash, string: ClassInstanceRef) -> ClassInstanceRef {
	// There's a chance that a string with `hash` already exists in the pool, so now that we computed
	// the hash on string, we can return either the existing string or the new one.
	STRING_POOL
		.write()
		.unwrap()
		.entry(hash)
		.or_insert(string)
		.clone()
}
