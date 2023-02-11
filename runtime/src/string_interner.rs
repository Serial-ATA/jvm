use crate::class_instance::{ArrayContent, ArrayInstance, ClassInstance, Instance};
use crate::classpath::classloader::ClassLoader;
use crate::reference::{ClassInstanceRef, Reference};

use std::collections::HashMap;
use std::ptr::slice_from_raw_parts;
use std::sync::Arc;

use byte_slice_cast::AsSliceOf;
use common::int_types::{s1, u1, u2};
use common::traits::PtrType;
use instructions::Operand;
use once_cell::sync::Lazy;

static STRING_POOL: Lazy<HashMap<Vec<u1>, ClassInstanceRef>> = Lazy::new(HashMap::new);

pub struct StringInterner;

// TODO: Need to wipe the string pool when the instances fall out of scope
impl StringInterner {
	pub fn get_java_string(raw: &[u1]) -> ClassInstanceRef {
		if let Some(interned) = STRING_POOL.get(raw) {
			return Arc::clone(interned);
		}

		Self::intern_string(raw)
	}

	pub fn intern_string(raw: &[u1]) -> ClassInstanceRef {
		// TODO: Error handling
		let java_string_class = ClassLoader::lookup_class(b"java/lang/String")
			.expect("java.lang.String should be loaded at this point");
		let byte_array_class = ClassLoader::Bootstrap.load(b"[B").unwrap();

		// TODO: More efficient conversion
		let string = unsafe { std::str::from_utf8_unchecked(raw) };
		let utf16_encoded_bytes = string.encode_utf16().collect::<Box<[u16]>>();
		let re_aligned_bytes: &[s1] = bytemuck::cast_slice(&utf16_encoded_bytes);

		let reference_to_byte_array = Operand::Reference(Reference::Array(ArrayInstance::new(
			byte_array_class,
			ArrayContent::Byte(re_aligned_bytes.to_vec().into_boxed_slice()),
		)));

		let new_java_string_instance = ClassInstance::new(Arc::clone(&java_string_class));

		// Set `private byte[] value`
		new_java_string_instance.get_mut().put_field_value0(
			crate::globals::string_value_field_offset(),
			reference_to_byte_array,
		);

		new_java_string_instance
	}

	pub fn rust_string_from_java_string(class: ClassInstanceRef) -> String {
		let string_value_field = class
			.get()
			.get_field_value0(crate::globals::string_value_field_offset());
		let char_array = string_value_field.expect_reference().extract_array();
		let chars = char_array.get().elements.expect_byte();

		let unsigned_chars =
			unsafe { &*slice_from_raw_parts(chars.as_ptr().cast::<u8>(), chars.len()) };
		String::from_utf16_lossy(unsigned_chars.as_slice_of::<u2>().unwrap())
	}
}
