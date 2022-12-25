use crate::class::Class;
use crate::class_instance::{ArrayContent, ArrayInstance};
use crate::classpath::classloader::ClassLoader;
use crate::reference::{MirrorInstanceRef, Reference};
use crate::thread::ThreadRef;

use std::collections::HashMap;
use std::sync::Arc;

use common::int_types::{u1, u2};
use instructions::Operand;
use once_cell::sync::Lazy;

static STRING_POOL: Lazy<HashMap<Vec<u1>, MirrorInstanceRef>> = Lazy::new(HashMap::new);

pub struct StringInterner;

// TODO: Need to wipe the string pool when the instances fall out of scope
impl StringInterner {
	pub fn get_java_string(raw: &[u1], thread: ThreadRef) -> MirrorInstanceRef {
		if let Some(interned) = STRING_POOL.get(raw) {
			return Arc::clone(interned);
		}

		Self::intern_string(raw, thread)
	}

	pub fn intern_string(raw: &[u1], thread: ThreadRef) -> MirrorInstanceRef {
		const STRING_CONSTRUCTOR_FROM_CHAR_ARRAY: &[u1] = b"([C)V";

		// TODO: Error handling
		let java_string_class = ClassLoader::lookup_class(b"java/lang/String")
			.expect("java.lang.String should be loaded at this point");
		let char_array_class = ClassLoader::Bootstrap.load(b"[C").unwrap();

		// TODO: Actual UTF-16 handling
		let chars_utf16 = raw.iter().map(|b| u2::from(*b)).collect::<Box<[u2]>>();
		let reference_to_char_array = Operand::Reference(Reference::Array(ArrayInstance::new(
			char_array_class,
			ArrayContent::Char(chars_utf16),
		)));

		let new_java_string_instance = Class::create_mirrored(Arc::clone(&java_string_class));

		// public String(char[] value)
		let constructor_args = vec![
			Operand::Reference(Reference::Mirror(Arc::clone(&new_java_string_instance))),
			reference_to_char_array,
		];

		Class::construct(
			java_string_class,
			thread,
			STRING_CONSTRUCTOR_FROM_CHAR_ARRAY,
			constructor_args,
		);

		new_java_string_instance
	}
}
