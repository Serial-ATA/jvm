use super::reference::ClassRef;
use crate::classpath::classloader::ClassLoader;
use crate::reference::MethodRef;

use std::fmt::Debug;
use std::sync::Arc;

use classfile::{Code, FieldType, LineNumber, MethodDescriptor, MethodInfo};
use common::int_types::{s4, u1, u2};
use common::traits::PtrType;

#[derive(Clone, PartialEq)]
pub struct Method {
	pub class: ClassRef,
	pub access_flags: u2,
	pub name: Vec<u1>,
	pub descriptor: Vec<u1>,
	pub parameter_count: u1,
	pub line_number_table: Vec<LineNumber>,
	pub code: Code,
}

#[rustfmt::skip]
impl Method {
	// Access flags
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.6-200-A.1

	pub const ACC_PUBLIC      : u2 = 0x0001; /* Declared public; may be accessed from outside its package. */
	pub const ACC_PRIVATE     : u2 = 0x0002; /* Declared private; accessible only within the defining class and other classes belonging to the same nest (§5.4.4). */
	pub const ACC_PROTECTED   : u2 = 0x0004; /* Declared protected; may be accessed within subclasses. */
	pub const ACC_STATIC      : u2 = 0x0008; /* Declared static. */
	pub const ACC_FINAL       : u2 = 0x0010; /* Declared final; must not be overridden (§5.4.5). */
	pub const ACC_SYNCHRONIZED: u2 = 0x0020; /* Declared synchronized; invocation is wrapped by a monitor use. */ // TODO: This is not respected anywhere
	pub const ACC_BRIDGE      : u2 = 0x0040; /* A bridge method, generated by the compiler. */
	pub const ACC_VARARGS     : u2 = 0x0080; /* Declared with variable number of arguments. */
	pub const ACC_NATIVE      : u2 = 0x0100; /* Declared native; implemented in a language other than the Java programming language. */
	pub const ACC_ABSTRACT    : u2 = 0x0400; /* Declared abstract; no implementation is provided. */
	pub const ACC_STRICT      : u2 = 0x0800; /* In a class file whose major version number is at least 46 and at most 60: Declared strictfp. */
	pub const ACC_SYNTHETIC   : u2 = 0x1000; /* Declared synthetic; not present in the source code. */
}

impl Method {
	pub fn new(class: ClassRef, method_info: &MethodInfo) -> MethodRef {
		let constant_pool = &class.unwrap_class_instance().constant_pool;

		let access_flags = method_info.access_flags;

		let name_index = method_info.name_index;
		let name = constant_pool.get_constant_utf8(name_index).to_vec();

		let descriptor_index = method_info.descriptor_index;
		let descriptor_bytes = constant_pool.get_constant_utf8(descriptor_index).to_vec();

		let parameter_count: u1 = MethodDescriptor::parse(&mut &descriptor_bytes[..])
			.parameters
			.len()
			.try_into()
			.unwrap();

		let line_number_table = method_info
			.get_line_number_table_attribute()
			.unwrap_or_default();
		let code = method_info.get_code_attribute().unwrap_or_default();

		let method = Self {
			class,
			access_flags,
			name,
			descriptor: descriptor_bytes,
			parameter_count,
			line_number_table,
			code,
		};

		MethodRef::new(method)
	}

	pub fn get_line_number(&self, pc: isize) -> s4 {
		if self.line_number_table.is_empty() {
			return -1;
		}

		for line_number in self.line_number_table.iter().copied() {
			if (line_number.start_pc as isize) == pc {
				return line_number.line_number as s4;
			}
		}

		-1
	}

	// https://docs.oracle.com/javase/specs/jvms/se20/html/jvms-2.html#jvms-2.10
	/// Find the exception handler for the given class and pc
	pub fn find_exception_handler(&self, class: ClassRef, pc: isize) -> Option<isize> {
		for exception_handler in &self.code.exception_table {
			let active_range =
				(exception_handler.start_pc as isize)..(exception_handler.end_pc as isize);
			if !active_range.contains(&pc) {
				continue;
			}

			// catch_type of 0 means this handles all exceptions
			if exception_handler.catch_type == 0 {
				return Some(exception_handler.handler_pc as isize);
			}

			let catch_type_class_name = self
				.class
				.unwrap_class_instance()
				.constant_pool
				.get_class_name(exception_handler.catch_type);
			let catch_type_class = ClassLoader::Bootstrap
				.load(catch_type_class_name)
				.expect("catch_type should be available");

			if catch_type_class == class || catch_type_class.is_subclass_of(Arc::clone(&class)) {
				return Some(exception_handler.handler_pc as isize);
			}
		}

		None
	}

	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.9.3
	pub fn is_polymorphic(&self) -> bool {
		const METHODHANDLE_CLASS_NAME: &[u1] = b"java/lang/invoke/MethodHandle";
		const VARHANDLE_CLASS_NAME: &[u1] = b"java/lang/invoke/VarHandle";

		let class = self.class.get();
		let mut is_polymorphic = false;

		// A method is signature polymorphic if all of the following are true:

		//     It is declared in the java.lang.invoke.MethodHandle class or the java.lang.invoke.VarHandle class.
		is_polymorphic |=
			class.name == METHODHANDLE_CLASS_NAME || class.name == VARHANDLE_CLASS_NAME;

		//     It has a single formal parameter of type Object[].
		let parsed_descriptor = MethodDescriptor::parse(&mut &self.descriptor[..]);
		match &*parsed_descriptor.parameters {
			[FieldType::Array(arr_ty)] => match &**arr_ty {
				FieldType::Object(ref obj) if &**obj == b"java/lang/Object" => {},
				_ => return false,
			},
			_ => return false,
		}

		//     It has the ACC_VARARGS and ACC_NATIVE flags set.
		is_polymorphic |= self.access_flags & Method::ACC_VARARGS != 0
			&& self.access_flags & Method::ACC_NATIVE != 0;

		is_polymorphic
	}

	pub fn is_native(&self) -> bool {
		self.access_flags & Method::ACC_NATIVE > 0
	}

	pub fn is_public(&self) -> bool {
		self.access_flags & Method::ACC_PUBLIC > 0
	}

	pub fn is_static(&self) -> bool {
		self.access_flags & Method::ACC_STATIC > 0
	}
}

impl Debug for Method {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		f.debug_struct("Method")
			.field("class", &self.class)
			.field("access_flags", &self.access_flags)
			.field("name", unsafe {
				&std::str::from_utf8_unchecked(&self.name)
			})
			.field("descriptor", unsafe {
				&std::str::from_utf8_unchecked(&self.descriptor)
			})
			.field("parameter_count", &self.parameter_count)
			.field("code_len", &self.code.code.len())
			.finish()
	}
}
