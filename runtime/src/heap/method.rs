use super::reference::ClassRef;
use crate::calls::CallStub;
use crate::classpath::classloader::ClassLoader;
use crate::reference::MethodRef;

use std::ffi::c_void;
use std::fmt::Debug;
use std::sync::Arc;

use classfile::accessflags::MethodAccessFlags;
use classfile::{Code, LineNumber, MethodDescriptor, MethodInfo};
use common::int_types::{s4, u1};
use symbols::Symbol;

#[derive(Clone, PartialEq)]
pub struct Method {
	pub class: ClassRef,
	pub access_flags: MethodAccessFlags,
	pub name: Symbol,
	pub descriptor: Symbol,
	pub parameter_count: u1,
	pub line_number_table: Vec<LineNumber>,
	pub code: Code,
	pub is_intrinsic: bool, // TODO: This can be done better
	native_invoker: *const c_void,
	native_method: *const c_void,
}

impl Method {
	pub fn new(class: ClassRef, method_info: &MethodInfo) -> MethodRef {
		let constant_pool = Arc::clone(&class.unwrap_class_instance().constant_pool);

		let access_flags = method_info.access_flags;

		let name_index = method_info.name_index;
		let name = constant_pool.get_constant_utf8(name_index);

		let descriptor_index = method_info.descriptor_index;
		let descriptor_bytes = constant_pool.get_constant_utf8(descriptor_index);

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

		let is_intrinsic = method_info.is_intrinsic_candidate(Arc::clone(&constant_pool));

		let method = Self {
			class,
			access_flags,
			name: Symbol::intern_bytes(name),
			descriptor: Symbol::intern_bytes(&descriptor_bytes),
			parameter_count,
			line_number_table,
			code,
			is_intrinsic,
			native_invoker: std::ptr::null(),
			native_method: std::ptr::null(),
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
				.load(Symbol::intern_bytes(catch_type_class_name))
				.expect("catch_type should be available");

			if catch_type_class == class || catch_type_class.is_subclass_of(Arc::clone(&class)) {
				return Some(exception_handler.handler_pc as isize);
			}
		}

		None
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

	pub fn native_invoker(&mut self) -> *const c_void {
		if self.native_invoker.is_null() {
			self.native_invoker =
				CallStub::for_descriptor(self.descriptor.as_str(), self.is_static());
		}

		self.native_invoker
	}

	pub fn native_method(&mut self) -> *const c_void {
		if self.native_method.is_null() {
			// TODO
		}

		unimplemented!("Native method setting");
		self.native_method
	}

	pub fn set_native_method(&mut self, func: *const c_void) {
		self.native_method = func;
	}
}

impl Debug for Method {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
		f.debug_struct("Method")
			.field("class", &self.class)
			.field("access_flags", &self.access_flags)
			.field("name", &self.name.as_str())
			.field("descriptor", &self.descriptor.as_str())
			.field("parameter_count", &self.parameter_count)
			.field("code_len", &self.code.code.len())
			.finish()
	}
}
