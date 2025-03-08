pub mod spec;

use crate::globals::{classes, fields};
use crate::java_call;
use crate::native::java::lang::String::StringInterner;
use crate::native::jni::reference_from_jobject;
use crate::native::method::NativeMethodPtr;
use crate::objects::array::{Array, ObjectArrayInstance};
use crate::objects::class::Class;
use crate::objects::class_instance::ClassInstance;
use crate::objects::constant_pool::cp_types;
use crate::objects::reference::{ObjectArrayInstanceRef, Reference};
use crate::symbols::{sym, Symbol};
use crate::thread::exceptions::Throws;
use crate::thread::JavaThread;

use std::ffi::VaList;
use std::fmt::{Debug, Formatter};
use std::sync::RwLock;

use classfile::accessflags::MethodAccessFlags;
use classfile::attribute::resolved::ResolvedAnnotation;
use classfile::attribute::{Attribute, Code, LineNumber};
use classfile::{FieldType, MethodDescriptor, MethodInfo};
use common::int_types::{s4, u1};
use common::traits::PtrType;
use instructions::Operand;
use jni::sys::{jdouble, jint, jlong, jobject, jvalue};

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

struct ExtraFields {
	parameter_stack_size: usize,
	descriptor_sym: Symbol,
	parameter_count: u1,
	line_number_table: Vec<LineNumber>,
	native_method: RwLock<Option<NativeMethodPtr>>,
}

impl PartialEq for ExtraFields {
	fn eq(&self, other: &Self) -> bool {
		self.parameter_stack_size == other.parameter_stack_size
			&& self.descriptor_sym == other.descriptor_sym
			&& self.parameter_count == other.parameter_count
			&& self.line_number_table == other.line_number_table
	}
}

pub struct Method {
	class: &'static Class,

	pub name: Symbol,
	pub descriptor: MethodDescriptor,
	pub access_flags: MethodAccessFlags,
	attributes: Box<[Attribute]>,

	extra_flags: ExtraFlags,
	extra_fields: ExtraFields,

	pub code: Code,
}

impl PartialEq for Method {
	fn eq(&self, other: &Self) -> bool {
		self.class == other.class
			&& self.access_flags == other.access_flags
			&& self.extra_flags == other.extra_flags
			&& self.extra_fields == other.extra_fields
			&& self.name == other.name
			&& self.code == other.code
	}
}

impl Debug for Method {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}#{}{}",
			self.class.name.as_str(),
			self.name.as_str(),
			self.descriptor_sym().as_str()
		)
	}
}

impl Method {
	/// Create a new `Method` instance
	///
	/// NOTE: This will leak the `Method` and return a reference. It is important that this only
	///       be called once per method. It should never be used outside of class loading.
	pub(super) fn new(class: &'static Class, method_info: MethodInfo) -> &'static mut Self {
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

		let mut parameter_stack_size = descriptor
			.parameters
			.iter()
			.map(|ty| ty.stack_size() as usize)
			.sum();
		if !access_flags.is_static() {
			parameter_stack_size += 1;
		}

		let parameter_count: u1 = descriptor.parameters.len().try_into().unwrap();

		let line_number_table = method_info
			.get_line_number_table_attribute()
			.unwrap_or_default();

		let extra_fields = ExtraFields {
			parameter_stack_size,
			descriptor_sym,
			parameter_count,
			line_number_table,
			native_method: RwLock::new(None), // Initialized later (if necessary)
		};

		let code = method_info.get_code_attribute().unwrap_or_default();

		let method = Self {
			class,
			attributes: method_info.attributes,
			access_flags,
			extra_flags,
			extra_fields,
			name,
			descriptor,
			code,
		};

		Box::leak(Box::new(method))
	}

	pub fn get_line_number(&self, pc: isize) -> s4 {
		if self.is_native() {
			return -2;
		}

		if self.extra_fields.line_number_table.is_empty() {
			return -1;
		}

		for line_number in self.extra_fields.line_number_table.iter().copied() {
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

	/// Get the method descriptor as a `Symbol`
	pub fn descriptor_sym(&self) -> Symbol {
		self.extra_fields.descriptor_sym
	}

	/// The number of parameters this method takes
	pub fn parameter_count(&self) -> u1 {
		self.extra_fields.parameter_count
	}

	/// The number of stack slots that the parameters take up
	///
	/// This is necessary, as `long`s and `double`s take up two slots
	///
	/// NOTE: This includes `this` for non-static methods
	pub fn parameter_stack_size(&self) -> usize {
		self.extra_fields.parameter_stack_size
	}

	pub fn native_method(&self) -> Option<NativeMethodPtr> {
		assert!(self.is_native());
		let native_method = self.extra_fields.native_method.read().unwrap();
		*native_method
	}

	pub fn set_native_method(&self, func: NativeMethodPtr) {
		let mut lock = self.extra_fields.native_method.write().unwrap();
		*lock = Some(func);
	}
}

// Getters
impl Method {
	#[inline]
	pub fn class(&self) -> &'static Class {
		self.class
	}

	pub fn external_name(&self) -> String {
		let mut external_name = format!(
			"{} {}.{}(",
			self.descriptor.return_type.as_java_type(),
			self.class.external_name(),
			self.name
		);
		for param in &self.descriptor.parameters {
			external_name.push_str(&param.as_java_type());
		}
		external_name.push(')');

		external_name
	}

	pub fn generic_signature(&self) -> Option<Symbol> {
		self.attributes
			.iter()
			.find_map(|attr| attr.signature())
			.map(|signature_attr| {
				self.class
					.constant_pool()
					.unwrap()
					.get::<cp_types::ConstantUtf8>(signature_attr.signature_index)
					.expect("resolution of method signatures should not fail")
			})
	}

	pub fn exception_types(&self) -> Throws<ObjectArrayInstanceRef> {
		let Some(exceptions) = self.attributes.iter().find_map(|attr| attr.exceptions()) else {
			return ObjectArrayInstance::new(0, classes::java_lang_Class());
		};

		let constant_pool = self.class.constant_pool().unwrap();

		let array = ObjectArrayInstance::new(
			exceptions.exception_index_table.len() as jint,
			classes::java_lang_Class(),
		)?;

		for (index, exception_class_index) in exceptions.exception_index_table.iter().enumerate() {
			let class = constant_pool.get::<cp_types::Class>(*exception_class_index)?;

			// SAFETY: The array is known to have the correct length
			unsafe {
				array
					.get_mut()
					.store_unchecked(index, Reference::mirror(class.mirror()))
			}
		}

		Throws::Ok(array)
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

	pub fn is_final(&self) -> bool {
		self.access_flags.is_final()
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
		let parameters = Self::parameter_types_array(class, &descriptor)?;

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

		let thread = JavaThread::current();

		// static java.lang.invoke.MethodHandleNatives#findMethodHandleType(Class rt, Class[] pts) -> MethodType
		let result = java_call!(
			thread,
			find_method_handle_type_method,
			Operand::Reference(return_type),
			Operand::Reference(Reference::object_array(parameters))
		);

		if thread.has_pending_exception() {
			return Throws::PENDING_EXCEPTION;
		}

		let method_type = result
			.expect("method should return something")
			.expect_reference();

		Throws::Ok(method_type)
	}

	/// Create a `java.lang.Class[]` of this method's parameter types
	///
	/// This takes a `Class`, as this process may load classes
	fn parameter_types_array(
		class: &'static Class,
		descriptor: &MethodDescriptor,
	) -> Throws<ObjectArrayInstanceRef> {
		let parameters = ObjectArrayInstance::new(
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
					parameters.get_mut().store(index as s4, mirror)?;
				},
				FieldType::Void => {
					panic!("Void parameter"); // TODO: Exception
				},
				FieldType::Object(class_name) => {
					let class = class.loader().load(Symbol::intern(&*class_name))?;
					parameters
						.get_mut()
						.store(index as s4, Reference::mirror(class.mirror()))?;
				},
				parameter @ FieldType::Array(_) => {
					let name = parameter.as_signature();
					let array_class = class.loader().load(Symbol::intern(&*name))?;
					parameters
						.get_mut()
						.store(index as s4, Reference::mirror(array_class.mirror()))?;
				},
			}
		}

		Throws::Ok(parameters)
	}

	/// Create a `java.lang.reflect.Constructor` instance for this method
	pub fn as_reflect_constructor(&self) -> Throws<Reference> {
		assert!(
			self.name == sym!(object_initializer_name) || self.name == sym!(class_initializer_name)
		);

		let constructor = ClassInstance::new(classes::java_lang_reflect_Constructor());

		// The slot is the method's position in the vtable
		let slot = self
			.class
			.vtable()
			.iter()
			.position(|m| m == self)
			.expect("a method must be present in a class vtable");

		let parameter_types = Self::parameter_types_array(self.class, &self.descriptor)?;
		let exception_types = self.exception_types()?;

		fields::java_lang_reflect_Constructor::set_clazz(
			constructor.get_mut(),
			Reference::mirror(self.class().mirror()),
		);
		fields::java_lang_reflect_Constructor::set_slot(constructor.get_mut(), slot as jint);
		fields::java_lang_reflect_Constructor::set_parameterTypes(
			constructor.get_mut(),
			Reference::object_array(parameter_types),
		);
		fields::java_lang_reflect_Constructor::set_exceptionTypes(
			constructor.get_mut(),
			Reference::object_array(exception_types),
		);
		fields::java_lang_reflect_Constructor::set_modifiers(
			constructor.get_mut(),
			self.access_flags.as_u2() as jint,
		);
		if let Some(generic_signature) = self.generic_signature() {
			let signature = StringInterner::intern(generic_signature);
			fields::java_lang_reflect_Constructor::set_signature(
				constructor.get_mut(),
				Reference::class(signature),
			);
		}
		// // TODO
		// fields::java_lang_reflect_Constructor::set_annotations(
		// 	constructor.get_mut(),
		// 	Reference::null(),
		// );
		// // TODO
		// fields::java_lang_reflect_Constructor::set_parameterAnnotations(
		// 	constructor.get_mut(),
		// 	Reference::null(),
		// );

		Throws::Ok(Reference::class(constructor))
	}
}

// JNI stuff
impl Method {
	pub unsafe fn args_for_c_array(
		&self,
		mut args: *const jvalue,
	) -> Option<Vec<Operand<Reference>>> {
		let mut parameters = Vec::with_capacity(self.parameter_stack_size());

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
					parameters.push(Operand::Empty);
				},
				FieldType::Double => {
					let val = unsafe { val.d };
					parameters.push(Operand::from(val));
					parameters.push(Operand::Empty);
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
		let mut parameters = Vec::with_capacity(self.parameter_count() as usize);
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
