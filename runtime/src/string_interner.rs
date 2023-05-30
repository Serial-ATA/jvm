use crate::class_instance::{ArrayContent, ArrayInstance, ClassInstance, Instance};
use crate::classpath::classloader::ClassLoader;
use crate::reference::{ClassInstanceRef, Reference};

use std::collections::HashMap;
use std::ptr::slice_from_raw_parts;
use std::sync::{Arc, RwLock};

use byte_slice_cast::AsSliceOf;
use common::int_types::{s1, s4, u1, u2};
use common::traits::PtrType;
use instructions::Operand;
use once_cell::sync::Lazy;

// Possible coders for Strings
const CODER_LATIN1: s4 = 0;
const CODER_UTF16: s4 = 1;

// Compact strings are enabled by default
const COMPACT_STRINGS: bool = true;

static STRING_POOL: Lazy<RwLock<HashMap<Vec<u1>, ClassInstanceRef>>> =
	Lazy::new(|| RwLock::new(HashMap::new()));

pub struct StringInterner;

// TODO: Need to wipe the string pool when the instances fall out of scope
impl StringInterner {
	pub fn get_java_string(raw: &[u1]) -> ClassInstanceRef {
		if let Some(interned) = STRING_POOL.read().unwrap().get(raw) {
			return Arc::clone(interned);
		}

		Self::intern_string(raw)
	}

	pub fn intern_string(raw: &[u1]) -> ClassInstanceRef {
		// TODO: Error handling
		let java_string_class = ClassLoader::lookup_class(b"java/lang/String")
			.expect("java.lang.String should be loaded at this point");
		let byte_array_class = ClassLoader::Bootstrap.load(b"[B").unwrap();

		let mut is_latin1 = false;
		if COMPACT_STRINGS {
			is_latin1 = str_is_latin1(raw);
		}

		let encoded_str;
		if is_latin1 {
			encoded_str = latin1_encode(raw);
		} else {
			encoded_str = utf_16_encode(raw);
		}

		let reference_to_byte_array = Operand::Reference(Reference::Array(ArrayInstance::new(
			byte_array_class,
			ArrayContent::Byte(encoded_str),
		)));

		let new_java_string_instance = ClassInstance::new(Arc::clone(&java_string_class));

		// Set `private byte[] value`
		new_java_string_instance.get_mut().put_field_value0(
			crate::globals::field_offsets::string_value_field_offset(),
			reference_to_byte_array,
		);

		// Set `private final byte coder`
		let coder = if is_latin1 { CODER_LATIN1 } else { CODER_UTF16 };
		new_java_string_instance.get_mut().put_field_value0(
			crate::globals::field_offsets::string_coder_field_offset(),
			Operand::Int(coder),
		);

		// TODO: Make this less of a mess
		let ret = Arc::clone(&new_java_string_instance);
		STRING_POOL
			.write()
			.unwrap()
			.insert(raw.to_vec(), new_java_string_instance);
		ret
	}

	pub fn rust_string_from_java_string(class: ClassInstanceRef) -> String {
		let string_value_field = class
			.get()
			.get_field_value0(crate::globals::field_offsets::string_value_field_offset());
		let string_coder_field = class
			.get()
			.get_field_value0(crate::globals::field_offsets::string_coder_field_offset())
			.expect_int();

		let char_array = string_value_field.expect_reference().extract_array();
		let chars = char_array.get().elements.expect_byte();

		let unsigned_chars =
			unsafe { &*slice_from_raw_parts(chars.as_ptr().cast::<u8>(), chars.len()) };
		match string_coder_field {
			CODER_LATIN1 => String::from_utf8_lossy(unsigned_chars).into_owned(),
			CODER_UTF16 => String::from_utf16_lossy(unsigned_chars.as_slice_of::<u2>().unwrap()),
			_ => unreachable!(),
		}
	}
}

fn str_is_latin1(raw: &[u1]) -> bool {
	let mut prev = 0;
	for byte in raw {
		// 0x80 denotes a multibyte sequence, but could also be a valid Latin-1 character
		if *byte == 0x80 && prev <= 0xC3 {
			return false;
		}
		prev = *byte;
	}

	true
}

fn utf_16_encode(raw: &[u1]) -> Box<[s1]> {
	// TODO: More efficient conversion
	let string = unsafe { std::str::from_utf8_unchecked(raw) };
	let utf16_encoded_bytes = string.encode_utf16().collect::<Box<[u16]>>();
	bytemuck::cast_slice(&utf16_encoded_bytes)
		.to_vec()
		.into_boxed_slice()
}

fn latin1_encode(raw: &[u1]) -> Box<[s1]> {
	let re_aligned_bytes: &[s1] = bytemuck::cast_slice(raw);
	re_aligned_bytes.to_vec().into_boxed_slice()
}
