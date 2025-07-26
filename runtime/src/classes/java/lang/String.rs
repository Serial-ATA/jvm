use crate::native::java::lang::String::LATIN1;
use crate::objects::instance::Instance;
use crate::objects::instance::array::{PrimitiveArrayInstance, PrimitiveArrayInstanceRef};
use crate::objects::instance::class::{ClassInstance, ClassInstanceRef};
use crate::objects::instance::object::Object;
use crate::objects::reference::Reference;
use crate::symbols::Symbol;
use crate::{globals, native};

use std::borrow::Cow;
use std::ptr::slice_from_raw_parts;

use byte_slice_cast::AsSliceOf;
use classfile::FieldType;
use common::int_types::u2;
use instructions::Operand;
use jni::sys::{jboolean, jbyte, jint};

pub trait IntoJavaStringInternable: sealed::Sealed {
	const IS_UTF8: bool;

	fn byte_slice(&self) -> &[u8];
}

mod sealed {
	use super::*;

	pub trait Sealed {}

	impl Sealed for Cow<'_, str> {}
	impl Sealed for &str {}
	impl Sealed for String {}
	impl Sealed for Symbol {}
	impl Sealed for &[u16] {}
}

impl IntoJavaStringInternable for Cow<'_, str> {
	const IS_UTF8: bool = true;

	fn byte_slice(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl IntoJavaStringInternable for &str {
	const IS_UTF8: bool = true;

	fn byte_slice(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl IntoJavaStringInternable for String {
	const IS_UTF8: bool = true;

	fn byte_slice(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl IntoJavaStringInternable for Symbol {
	// * All `Symbols` are valid UTF-8
	const IS_UTF8: bool = true;

	fn byte_slice(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl IntoJavaStringInternable for &[u16] {
	const IS_UTF8: bool = false;

	fn byte_slice(&self) -> &[u8] {
		bytemuck::cast_slice(self)
	}
}

/// Create a new `java.lang.String` instance
///
/// This will automatically handle compacting of Latin-1 strings if enabled.
pub fn new<T>(content: T) -> ClassInstanceRef
where
	T: IntoJavaStringInternable,
{
	fn inner(string: &[u8], is_utf8: bool) -> ClassInstanceRef {
		let mut is_latin1 = false;
		if is_utf8 && native::java::lang::String::COMPACT_STRINGS {
			is_latin1 = str_is_latin1(string);
		}

		let encoded_str;
		if is_latin1 {
			let byte_slice: &[jbyte] = bytemuck::cast_slice(string);
			encoded_str = byte_slice.to_vec().into_boxed_slice();
		} else {
			if is_utf8 {
				// SAFETY: &[u8] and &str have the same layout
				let string: &str = unsafe { std::mem::transmute(string) };
				// TODO: More efficient conversion
				let utf16_encoded_bytes = string.encode_utf16().collect::<Box<[u16]>>();
				encoded_str = bytemuck::cast_slice(&utf16_encoded_bytes)
					.to_vec()
					.into_boxed_slice()
			} else {
				// Otherwise, the source is a UTF-16 encoded string (hopefully)
				assert_eq!(string.len() % 2, 0);
				let byte_slice: &[jbyte] = bytemuck::cast_slice(string);
				encoded_str = byte_slice.to_vec().into_boxed_slice();
			}
		}

		let new_java_string_instance = ClassInstance::new(globals::classes::java_lang_String());

		// Set `private byte[] value`
		set_value(
			new_java_string_instance,
			Reference::array(PrimitiveArrayInstance::new::<jbyte>(encoded_str)),
		);

		// Set `private final byte coder`
		if is_latin1 {
			set_coder(
				new_java_string_instance,
				native::java::lang::String::LATIN1.into(),
			);
		} else {
			set_coder(
				new_java_string_instance,
				native::java::lang::String::UTF16.into(),
			);
		};

		new_java_string_instance
	}

	let string = content.byte_slice();
	inner(string, T::IS_UTF8)
}

/// Create a new `java.lang.String` instance with the given hash
///
/// Same as [`new()`], but also sets the hash field. This should only be used in string interning.
pub fn new_with_hash<T>(content: T, hash: jint) -> ClassInstanceRef
where
	T: IntoJavaStringInternable,
{
	let java_string = new(content);
	set_hash(java_string, hash);
	java_string
}

fn str_is_latin1(string: &[u8]) -> bool {
	let mut prev = 0;
	for byte in string {
		// 0x80 denotes a multibyte sequence, but could also be a valid Latin-1 character
		if (*byte & 0xC0) == 0x80 && prev <= 0xC3 {
			return false;
		}
		prev = *byte;
	}

	true
}

/// Extract a [`String`] from the contents of a `java.lang.String` instance
pub fn extract(instance: ClassInstanceRef) -> String {
	let value = value(instance);
	let value = value.as_slice::<jbyte>();

	let coder = coder(instance);

	// SAFETY: &[i8] and &[u8] have the same layout
	let unsigned_chars =
		unsafe { &*slice_from_raw_parts(value.as_ptr().cast::<u8>(), value.len()) };
	match coder {
		native::java::lang::String::LATIN1 => String::from_utf8_lossy(unsigned_chars).into_owned(),
		native::java::lang::String::UTF16 => {
			String::from_utf16_lossy(unsigned_chars.as_slice_of::<u2>().unwrap())
		},
		_ => panic!("Invalid string coder `{coder}`"),
	}
}

/// Get the length of a `java.lang.String` **in characters**
///
/// To get the length of the string in *bytes*, just get the length of the [`value()`].
pub fn length(this: ClassInstanceRef) -> usize {
	assert_eq!(this.class(), globals::classes::java_lang_String());

	let value_instance = value(this);
	let value = value_instance.as_slice::<jbyte>();
	if coder(this) == LATIN1 {
		return value.len();
	}

	assert_eq!(
		value.len() & 1,
		0,
		"UTF-16 strings must have an even length"
	);
	value.len() >> 1
}

/// `java.lang.String#value` field
pub fn value(instance: ClassInstanceRef) -> PrimitiveArrayInstanceRef {
	instance
		.get_field_value0(value_field_index())
		.expect_reference()
		.extract_primitive_array()
}

pub fn set_value(instance: ClassInstanceRef, value: Reference) {
	instance.put_field_value0(value_field_index(), Operand::Reference(value))
}

/// `java.lang.String#coder` field
pub fn coder(instance: ClassInstanceRef) -> jbyte {
	instance.get_field_value0(coder_field_index()).expect_int() as jbyte
}

pub fn set_coder(instance: ClassInstanceRef, value: jbyte) {
	instance.put_field_value0(coder_field_index(), Operand::Int(value as jint))
}

/// `java.lang.String#hash` field
pub fn hash(instance: ClassInstanceRef) -> jint {
	instance.get_field_value0(hash_field_index()).expect_int()
}

pub fn set_hash(instance: ClassInstanceRef, value: jint) {
	instance.put_field_value0(hash_field_index(), Operand::Int(value))
}

/// `java.lang.String#hashIsZero` field
pub fn hashIsZero(instance: ClassInstanceRef) -> jboolean {
	instance
		.get_field_value0(hashIsZero_field_index())
		.expect_int()
		!= 0
}

pub fn set_hashIsZero(instance: ClassInstanceRef, value: jboolean) {
	instance.put_field_value0(hashIsZero_field_index(), Operand::Int(value as jint))
}

crate::classes::field_module! {
	@CLASS java_lang_String;

	@FIELDSTART
	/// `java.lang.String#value` field offset
	///
	/// Expected type: `jByteArray`
	@FIELD value: FieldType::Array(val) if **val == FieldType::Byte,
	/// `java.lang.String#coder` field offset
	///
	/// Expected type: `jbyte`
	@FIELD coder: FieldType::Byte,
	/// `java.lang.String#hash` field offset
	///
	/// Expected type: `jint`
	@FIELD hash: FieldType::Integer,
	/// `java.lang.String#hashIsZero` field offset
	///
	/// Expected type: `jboolean`
	@FIELD hashIsZero: FieldType::Boolean,
}
