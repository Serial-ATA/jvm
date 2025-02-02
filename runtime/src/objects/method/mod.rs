pub mod spec;

use crate::java_call;
use crate::native::jni::reference_from_jobject;
use crate::native::method::NativeMethodPtr;
use crate::objects::array::ArrayInstance;
use crate::objects::class::Class;
use crate::objects::constant_pool::cp_types;
use crate::objects::reference::Reference;
use crate::thread::exceptions::Throws;
use crate::thread::JavaThread;

use std::ffi::VaList;
use std::fmt::{Debug, Formatter};
use std::sync::RwLock;

use crate::calls::jcall::JavaCallResult;
use classfile::accessflags::MethodAccessFlags;
use classfile::attribute::resolved::ResolvedAnnotation;
use classfile::attribute::{Code, LineNumber};
use classfile::{FieldType, MethodDescriptor, MethodInfo};
use common::int_types::{s4, u1};
use common::traits::PtrType;
use instructions::Operand;
use jni::sys::{jdouble, jint, jlong, jobject, jvalue};
use symbols::{sym, Symbol};

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
	pub descriptor_sym: Symbol,
	pub descriptor: MethodDescriptor,
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
			&& self.descriptor_sym == other.descriptor_sym
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

		// TODO: Handle throws
		let name_index = method_info.name_index;
		let name = constant_pool
			.get::<cp_types::ConstantUtf8>(name_index)
			.expect("method name should always resolve");

		// TODO: Handle throws
		let descriptor_index = method_info.descriptor_index;
		let descriptor_sym = constant_pool
			.get::<cp_types::ConstantUtf8>(descriptor_index)
			.expect("method descriptor should always resolve");

		let descriptor = MethodDescriptor::parse(&mut descriptor_sym.as_bytes()).unwrap(); // TODO: Error handling

		let parameter_count: u1 = descriptor.parameters.len().try_into().unwrap();

		let line_number_table = method_info
			.get_line_number_table_attribute()
			.unwrap_or_default();
		let code = method_info.get_code_attribute().unwrap_or_default();

		let method = Self {
			class,
			access_flags,
			extra_flags,
			name,
			descriptor_sym,
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

			// TODO: Handle throws here, should take precedence over the current exception?
			let catch_type_class = self
				.class
				.unwrap_class_instance()
				.constant_pool
				.get::<cp_types::Class>(exception_handler.catch_type)
				.unwrap();

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

	pub fn is_var_args(&self) -> bool {
		self.access_flags.is_varargs()
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

// Parsing stuff
impl Method {
	/// Get the `java.lang.invoke.MethodType` for a method descriptor
	///
	/// This requires an initiating [`Class`], as this process may load additional classes.
	///
	/// This will parse the `descriptor` and call `java.lang.invoke.MethodHandleNatives#findMethodHandleType` on
	/// the current thread.
	pub fn method_type_for(class: &'static Class, descriptor: &str) -> Throws<Reference> {
		let descriptor = MethodDescriptor::parse(&mut descriptor.as_bytes()).unwrap(); // TODO: Error handling
		let parameters = ArrayInstance::new_reference(
			descriptor.parameters.len() as s4,
			crate::globals::classes::java_lang_Class(),
		)?;

		for (index, parameter) in descriptor.parameters.iter().enumerate() {
			match parameter {
				FieldType::Byte
				| FieldType::Char
				| FieldType::Double
				| FieldType::Float
				| FieldType::Int
				| FieldType::Long
				| FieldType::Short
				| FieldType::Boolean => {
					let mirror = crate::globals::mirrors::primitive_mirror_for(&parameter);
					parameters
						.get_mut()
						.store(index as s4, Operand::Reference(mirror))?;
				},
				FieldType::Void => {
					panic!("Void parameter"); // TODO: Exception
				},
				FieldType::Object(class_name) => {
					let class = class.loader().load(Symbol::intern(&*class_name))?;
					parameters.get_mut().store(
						index as s4,
						Operand::Reference(Reference::mirror(class.mirror())),
					)?;
				},
				FieldType::Array(_) => todo!("Array parameters"),
			}
		}

		let return_type;
		match descriptor.return_type {
			FieldType::Byte
			| FieldType::Char
			| FieldType::Double
			| FieldType::Float
			| FieldType::Int
			| FieldType::Long
			| FieldType::Short
			| FieldType::Boolean
			| FieldType::Void => {
				return_type =
					crate::globals::mirrors::primitive_mirror_for(&descriptor.return_type);
			},
			FieldType::Object(class_name) => {
				let class = class.loader().load(Symbol::intern(class_name))?;
				return_type = Reference::mirror(class.mirror());
			},
			FieldType::Array(_) => todo!("Array returns"),
		}

		let method_handle_natives_class =
			crate::globals::classes::java_lang_invoke_MethodHandleNatives();

		let find_method_handle_type_method = method_handle_natives_class.resolve_method(
			sym!(findMethodHandleType),
			sym!(findMethodHandleType_signature),
		)?;

		// static java.lang.invoke.MethodHandleNatives#findMethodHandleType(Class rt, Class[] pts) -> MethodType
		let result = java_call!(
			JavaThread::current(),
			find_method_handle_type_method,
			Operand::Reference(return_type),
			Operand::Reference(Reference::array(parameters))
		);

		let method_type;
		match result {
			JavaCallResult::Ok(op) => {
				method_type = op
					.expect("method should return something")
					.expect_reference();
			},
			JavaCallResult::PendingException => {
				JavaThread::current().throw_pending_exception(false);
				todo!();
			},
		}

		Throws::Ok(method_type)
	}
}

// JNI stuff
impl Method {
	pub unsafe fn args_for_c_array(
		&self,
		mut args: *const jvalue,
	) -> Option<Vec<Operand<Reference>>> {
		let mut parameters = Vec::with_capacity(self.parameter_count as usize);

		for parameter in &self.descriptor.parameters {
			let val = unsafe { *args };

			match parameter {
				FieldType::Byte => {
					let val = unsafe { val.b };
					parameters.push(Operand::from(val))
				},
				FieldType::Char => {
					let val = unsafe { val.c };
					parameters.push(Operand::from(val))
				},
				FieldType::Short => {
					let val = unsafe { val.s };
					parameters.push(Operand::from(val))
				},
				FieldType::Int => {
					let val = unsafe { val.i };
					parameters.push(Operand::from(val))
				},

				FieldType::Boolean => {
					let val = unsafe { val.z };
					parameters.push(Operand::from(val));
				},

				FieldType::Long => {
					let val = unsafe { val.j };
					parameters.push(Operand::from(val));
				},
				FieldType::Double => {
					let val = unsafe { val.d };
					parameters.push(Operand::from(val))
				},
				FieldType::Float => {
					let val = unsafe { val.f };
					parameters.push(Operand::from(val))
				},

				FieldType::Object(_) | FieldType::Array(_) => {
					let val = unsafe { val.l };
					let obj = unsafe { reference_from_jobject(val) };
					let Some(obj) = obj else {
						return None;
					};

					parameters.push(Operand::Reference(obj))
				},

				FieldType::Void => unreachable!(),
			}

			unsafe {
				args = args.add(1);
			}
		}

		Some(parameters)
	}

	pub unsafe fn args_for_va_list(&self, mut args: VaList) -> Option<Vec<Operand<Reference>>> {
		let mut parameters = Vec::with_capacity(self.parameter_count as usize);
		for parameter in &self.descriptor.parameters {
			match parameter {
				FieldType::Byte | FieldType::Char | FieldType::Short | FieldType::Int => {
					parameters.push(Operand::from(args.arg::<jint>()))
				},

				FieldType::Boolean => parameters.push(Operand::from(args.arg::<jint>() != 0)),

				FieldType::Long => parameters.push(Operand::from(args.arg::<jlong>())),
				FieldType::Double => parameters.push(Operand::from(args.arg::<jdouble>())),
				FieldType::Float => todo!("float parameter"),

				FieldType::Object(_) | FieldType::Array(_) => {
					// TODO: Is this correct?
					let obj_raw = args.arg::<*mut ()>();
					let obj = unsafe { reference_from_jobject(obj_raw as jobject) };
					let Some(obj) = obj else {
						return None;
					};

					parameters.push(Operand::Reference(obj))
				},

				FieldType::Void => unreachable!(),
			}
		}

		Some(parameters)
	}
}
