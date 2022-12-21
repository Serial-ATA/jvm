use crate::class::Class;
use crate::class_instance::{ArrayContent, ArrayInstance, ClassInstance};
use crate::classpath::classloader::ClassLoader;
use crate::reference::{ClassInstanceRef, Reference};
use crate::stack::operand_stack::Operand;
use crate::thread::ThreadRef;

use std::collections::HashMap;
use std::sync::Arc;

use once_cell::sync::Lazy;
use common::int_types::{u1, u2};

static STRING_POOL: Lazy<HashMap<Vec<u1>, ClassInstanceRef>> = Lazy::new(HashMap::new);

pub struct StringInterner;

// TODO: Need to wipe the string pool when the instances fall out of scope
impl StringInterner {
	pub fn get_java_string(raw: &[u1], thread: ThreadRef) -> ClassInstanceRef {
		if let Some(interned) = STRING_POOL.get(raw) {
			return Arc::clone(interned);
		}

		Self::intern_string(raw, thread)
	}

	pub fn intern_string(raw: &[u1], thread: ThreadRef) -> ClassInstanceRef {
		const STRING_CONSTRUCTOR_FROM_CHAR_ARRAY: &[u8] = b"([C)V";

		// TODO: Error handling
		let java_string_class = ClassLoader::Bootstrap.load(b"java/lang/String").unwrap();
		let char_array_class = ClassLoader::Bootstrap.load(b"[C").unwrap();

		// TODO: Actual UTF-16 handling
		let chars_utf16 = raw.iter().map(|b| u2::from(*b)).collect::<Box<[u2]>>();
		let reference_to_char_array = Operand::Reference(Reference::Array(ArrayInstance::new(
			char_array_class,
			ArrayContent::Char(chars_utf16),
		)));

		let new_java_string_instance = ClassInstance::new(Arc::clone(&java_string_class));

		// public String(char[] value)
		let constructor_args = vec![
			Operand::Reference(Reference::Class(Arc::clone(&new_java_string_instance))),
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
