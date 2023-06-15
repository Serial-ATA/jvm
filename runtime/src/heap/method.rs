use super::reference::ClassRef;
use crate::classpath::classloader::ClassLoader;
use crate::reference::MethodRef;

use std::fmt::Debug;
use std::sync::Arc;

use classfile::accessflags::MethodAccessFlags;
use classfile::{Code, FieldType, LineNumber, MethodDescriptor, MethodInfo};
use common::int_types::{s4, u1};
use common::traits::PtrType;

#[derive(Clone, PartialEq)]
pub struct Method {
	pub class: ClassRef,
	pub access_flags: MethodAccessFlags,
	pub name: Vec<u1>,
	pub descriptor: Vec<u1>,
	pub parameter_count: u1,
	pub line_number_table: Vec<LineNumber>,
	pub code: Code,
	pub is_intrinsic: bool, // TODO: This can be done better
}

impl Method {
	pub fn new(class: ClassRef, method_info: &MethodInfo) -> MethodRef {
		let constant_pool = Arc::clone(&class.unwrap_class_instance().constant_pool);

		let access_flags = method_info.access_flags;

		let name_index = method_info.name_index;
		let name = constant_pool.get_constant_utf8(name_index).to_vec();

		let descriptor_index = method_info.descriptor_index;
		let descriptor_bytes = constant_pool.get_constant_utf8(descriptor_index).to_vec();

		let parameter_count: u1 = MethodDescriptor::parse(&mut &descriptor_bytes[..])
			.unwrap() // TODO: Error handling
			.parameters
			.len()
			.try_into()
			.unwrap();

		let line_number_table = method_info
			.get_line_number_table_attribute()
			.unwrap_or_default();
		let code = method_info.get_code_attribute().unwrap_or_default();

		let is_intrinsic = method_info.is_intrinsic_candidate(constant_pool);

		let method = Self {
			class,
			access_flags,
			name,
			descriptor: descriptor_bytes,
			parameter_count,
			line_number_table,
			code,
			is_intrinsic,
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
		let parsed_descriptor = MethodDescriptor::parse(&mut &self.descriptor[..]).unwrap(); // TODO: Error handling
		match &*parsed_descriptor.parameters {
			[FieldType::Array(arr_ty)] => match &**arr_ty {
				FieldType::Object(ref obj) if &**obj == b"java/lang/Object" => {},
				_ => return false,
			},
			_ => return false,
		}

		//     It has the ACC_VARARGS and ACC_NATIVE flags set.
		is_polymorphic |= self.access_flags.is_varargs() && self.is_native();

		is_polymorphic
	}

	/// Whether this method can override the provided instance method ([§5.4.3.3](https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-5.html#jvms-5.4.5))
	pub fn can_override(&self, other: &Method) -> bool {
		// An instance method mC can override another instance method mA iff all of the following are true:

		// mC has the same name and descriptor as mA.
		//
		// mC is not marked ACC_PRIVATE.
		//
		// One of the following is true:
		//
		//     mA is marked ACC_PUBLIC.
		//
		//     mA is marked ACC_PROTECTED.
		//
		//     mA is marked neither ACC_PUBLIC nor ACC_PROTECTED nor ACC_PRIVATE, and either:
		//
		//         (a) the declaration of mA appears in the same run-time package as the declaration of mC, or
		//         (b) if mA is declared in a class A and mC is declared in a class C, then there exists a method mB declared in a class B
		//             such that C is a subclass of B and B is a subclass of A and mC can override mB and mB can override mA.

		(self.name == other.name && self.descriptor == other.descriptor)
			&& !self.is_private()
			&& (other.is_public()
				|| other.is_protected()
				|| (!other.is_private() && other.class.shares_package_with(self.class.get())))
	}

	pub fn is_native(&self) -> bool {
		self.access_flags.is_native()
	}

	pub fn is_public(&self) -> bool {
		self.access_flags.is_public()
	}

	pub fn is_private(&self) -> bool {
		self.access_flags.is_private()
	}

	pub fn is_protected(&self) -> bool {
		self.access_flags.is_protected()
	}

	pub fn is_static(&self) -> bool {
		self.access_flags.is_static()
	}

	pub fn is_abstract(&self) -> bool {
		self.access_flags.is_abstract()
	}

	pub fn is_default(&self) -> bool {
		self.class.is_interface() && (!self.is_abstract() && !self.is_public())
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
