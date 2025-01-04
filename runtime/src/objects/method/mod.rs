pub mod spec;

use crate::native::NativeMethodPtr;
use crate::objects::class::Class;
use crate::objects::constant_pool::cp_types;

use std::fmt::{Debug, Formatter};
use std::sync::RwLock;

use classfile::accessflags::MethodAccessFlags;
use classfile::{Code, LineNumber, MethodDescriptor, MethodInfo, ResolvedAnnotation};
use common::int_types::{s4, u1};
use symbols::Symbol;

#[derive(Default, PartialEq, Eq, Debug)]
struct ExtraFlags {
	caller_sensitive: bool,
	intrinsic: bool,
}

impl ExtraFlags {
	fn from_annotations(annotations: impl Iterator<Item = ResolvedAnnotation>) -> Self {
		const CALLER_SENSITIVE_TYPE: &str = "Ljdk/internal/reflect/CallerSensitive;";
		const INTRINSIC_CANDIDATE_TYPE: &str = "Ljdk/internal/vm/annotation/IntrinsicCandidate;";

		let mut ret = Self::default();

		for annotation in annotations {
			match &*annotation.name {
				CALLER_SENSITIVE_TYPE => ret.caller_sensitive = true,
				INTRINSIC_CANDIDATE_TYPE => ret.intrinsic = true,
				_ => {},
			}
		}

		ret
	}
}

pub struct Method {
	class: &'static Class,
	pub access_flags: MethodAccessFlags,
	extra_flags: ExtraFlags,
	pub name: Symbol,
	pub descriptor: Symbol,
	pub parameter_count: u1,
	pub line_number_table: Vec<LineNumber>,
	pub code: Code,
	native_method: RwLock<Option<NativeMethodPtr>>,
}

impl PartialEq for Method {
	fn eq(&self, other: &Self) -> bool {
		self.class == other.class
			&& self.access_flags == other.access_flags
			&& self.extra_flags == other.extra_flags
			&& self.name == other.name
			&& self.descriptor == other.descriptor
			&& self.parameter_count == other.parameter_count
			&& self.line_number_table == other.line_number_table
			&& self.code == other.code
	}
}

impl Debug for Method {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}#{}", self.class.name.as_str(), self.name.as_str())
	}
}

impl Method {
	/// Create a new `Method` instance
	///
	/// NOTE: This will leak the `Method` and return a reference. It is important that this only
	///       be called once per method. It should never be used outside of class loading.
	pub(super) fn new(class: &'static Class, method_info: &MethodInfo) -> &'static mut Self {
		let constant_pool = class.constant_pool().unwrap();

		let access_flags = method_info.access_flags;

		let extra_flags;
		match method_info.runtime_visible_annotations(constant_pool.raw()) {
			Some(annotations) => {
				extra_flags = ExtraFlags::from_annotations(annotations);
			},
			None => {
				extra_flags = ExtraFlags::default();
			},
		}

		let name_index = method_info.name_index;
		let name = constant_pool.get::<cp_types::ConstantUtf8>(name_index);

		let descriptor_index = method_info.descriptor_index;
		let descriptor = constant_pool.get::<cp_types::ConstantUtf8>(descriptor_index);

		let parameter_count: u1 = MethodDescriptor::parse(&mut descriptor.as_bytes())
			.unwrap() // TODO: Error handling
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
			extra_flags,
			name,
			descriptor,
			parameter_count,
			line_number_table,
			code,
			native_method: RwLock::new(None),
		};

		Box::leak(Box::new(method))
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
	pub fn find_exception_handler(&self, class: &'static Class, pc: isize) -> Option<isize> {
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

			let catch_type_class = self
				.class
				.unwrap_class_instance()
				.constant_pool
				.get::<cp_types::Class>(exception_handler.catch_type);

			if catch_type_class == class || catch_type_class.is_subclass_of(class) {
				return Some(exception_handler.handler_pc as isize);
			}
		}

		None
	}

	pub fn native_method(&self) -> Option<NativeMethodPtr> {
		assert!(self.is_native());
		let native_method = self.native_method.read().unwrap();
		*native_method
	}

	pub fn set_native_method(&self, func: NativeMethodPtr) {
		let mut lock = self.native_method.write().unwrap();
		*lock = Some(func);
	}
}

// Getters
impl Method {
	#[inline]
	pub fn class(&self) -> &'static Class {
		self.class
	}
}

// Flags
impl Method {
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

	/// Whether the method has the @CallerSensitive annotation
	pub fn is_caller_sensitive(&self) -> bool {
		self.extra_flags.caller_sensitive
	}

	pub fn is_stack_walk_ignored(&self) -> bool {
		if self
			.class
			.is_subclass_of(crate::globals::classes::jdk_internal_reflect_MethodAccessorImpl())
		{
			return true;
		}

		false
	}
}
