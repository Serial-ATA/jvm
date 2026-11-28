use crate::classes;
use crate::native::jni::JniEnvExt;
use crate::native::jni::references::JObjectExt;
use crate::objects::instance::class::ClassInstanceRef;
use crate::objects::reference::Reference;
use crate::symbols::Symbol;

use std::borrow::Cow;
use std::collections::HashMap;
use std::ffi::c_char;
use std::sync::{LazyLock, RwLock};

use ::jni::env::JniEnv;
use ::jni::objects::JString;
use ::jni::sys::jbyte;
use byte_slice_cast::{AsByteSlice, AsSliceOf};
use common::int_types::{s1, u1, u2};
use native_macros::jni_call;

// TODO: This is controlled by a cli argument
// Compact strings are enabled by default
pub(crate) const COMPACT_STRINGS: bool = true;

pub const LATIN1: s1 = 0;
pub const UTF16: s1 = 1;

// TODO: Need to wipe the string pool when the instances fall out of scope
static STRING_POOL: LazyLock<RwLock<HashMap<StringHash, ClassInstanceRef>>> =
	LazyLock::new(|| RwLock::new(HashMap::new()));

fn lookup(hash: StringHash) -> Option<ClassInstanceRef> {
	if let Some(entry) = STRING_POOL.read().unwrap().get(&hash) {
		return Some(*entry);
	}

	None
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct StringHash(i32);

pub trait StringHashDerivable<T> {
	fn string_hash(value: &T) -> StringHash;
}

impl StringHashDerivable<String> for String {
	fn string_hash(value: &String) -> StringHash {
		<&str as StringHashDerivable<&str>>::string_hash(&value.as_str())
	}
}
impl<'a> StringHashDerivable<&'a str> for &'a str {
	fn string_hash(value: &Self) -> StringHash {
		<&[u1] as StringHashDerivable<&[u1]>>::string_hash(&value.as_bytes())
	}
}
impl<'a> StringHashDerivable<Cow<'a, str>> for Cow<'a, str> {
	fn string_hash(value: &Cow<'a, str>) -> StringHash {
		<&str as StringHashDerivable<&str>>::string_hash(&&**value)
	}
}
impl<'a> StringHashDerivable<&'a [u1]> for &'a [u1] {
	fn string_hash(value: &Self) -> StringHash {
		let mut h = 0;
		for b in *value {
			h = (31_u32.wrapping_mul(h)) + u32::from(*b);
		}
		StringHash(h as i32)
	}
}
impl<'a> StringHashDerivable<&'a [u2]> for &'a [u2] {
	fn string_hash(value: &Self) -> StringHash {
		let mut h = 0;
		for b in *value {
			h = (31_u32.wrapping_mul(h)) + u32::from(*b);
		}
		StringHash(h as i32)
	}
}

impl StringHashDerivable<Symbol> for Symbol {
	fn string_hash(value: &Self) -> StringHash {
		<&str as StringHashDerivable<&str>>::string_hash(&value.as_str())
	}
}

pub struct StringInterner<T>(T);

impl<T> StringInterner<T>
where
	T: StringHashDerivable<T>,
	T: Into<Symbol>,
{
	pub fn intern(string: T) -> ClassInstanceRef {
		let hash = <T as StringHashDerivable<T>>::string_hash(&string);

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
	*STRING_POOL.write().unwrap().entry(hash).or_insert(string)
}

#[jni_call]
pub extern "C" fn JVM_InternString(env: JniEnv, string: JString) -> JString {
	let thread = unsafe { env.thread() };

	if string.is_null() {
		return JString::null();
	}

	let string = unsafe { string.to_reference() }
		.expect("should be valid")
		.extract_class();

	let hash = classes::java::lang::String::hash(string);
	let hash_is_zero = classes::java::lang::String::hashIsZero(string);
	if (hash != 0 || hash_is_zero)
		&& let Some(interned_string) = lookup(StringHash(hash))
	{
		let ptr = thread
			.jni_refs()
			.allocate(Reference::class(interned_string));
		return unsafe { JString::from_raw(ptr) };
	}

	// Otherwise something's off, recompute the hash...

	let coder = classes::java::lang::String::coder(string);
	let value = classes::java::lang::String::value(string);

	let value = value.as_slice::<jbyte>();
	let value_unsigned = value.as_byte_slice();

	let computed_hash;
	if value_unsigned.is_empty() {
		classes::java::lang::String::set_hash(string, 0);
		classes::java::lang::String::set_hashIsZero(string, true);
		computed_hash = StringHash(0);
	} else {
		let hash = match coder {
			LATIN1 => <&[u1] as StringHashDerivable<&[u1]>>::string_hash(&value_unsigned),
			UTF16 => <&[u2] as StringHashDerivable<&[u2]>>::string_hash(
				&value_unsigned.as_slice_of::<u2>().unwrap(),
			),
			_ => panic!("Invalid string coder `{coder}`"),
		};

		classes::java::lang::String::set_hash(string, hash.0);
		classes::java::lang::String::set_hashIsZero(string, false);
		computed_hash = hash;
	}

	let ptr = thread
		.jni_refs()
		.allocate(Reference::class(do_intern(computed_hash, string)));
	unsafe { JString::from_raw(ptr) }
}

#[jni_call(no_env, no_strict_types)]
pub extern "C" fn JVM_ReleaseUTF(_utf: *const c_char) {
	todo!()
}
