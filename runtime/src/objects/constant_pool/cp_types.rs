use super::entry::ResolvedEntry;
use crate::native::java::lang::invoke::MethodHandleNatives;
use crate::native::java::lang::String::StringInterner;
use crate::objects::array::{Array, ObjectArrayInstance};
use crate::objects::boxing::Boxable;
use crate::objects::class::Class as ClassObj;
use crate::objects::constant_pool::ConstantPool;
use crate::objects::field::Field;
use crate::objects::method::Method;
use crate::objects::reference::Reference;
use crate::symbols::{sym, Symbol};
use crate::thread::exceptions::{throw, Throws};
use crate::thread::JavaThread;
use crate::{classes, java_call};

use classfile::constant_pool::types::{
	raw as raw_types, CpEntry, LoadableConstantPoolValueInner, ReferenceEntry, ReferenceKind,
};
use classfile::MethodDescriptor;
use common::int_types::{s4, s8, u1, u2};
use common::traits::PtrType;
use instructions::Operand;

/// A constant pool entry of any type
pub enum Entry {
	Class(<Class as EntryType>::Resolved),
	Integer(<Integer as EntryType>::Resolved),
	Double(<Double as EntryType>::Resolved),
	Float(<Float as EntryType>::Resolved),
	Long(<Long as EntryType>::Resolved),
	ClassName(<ClassName as EntryType>::Resolved),
	ConstantUtf8(<ConstantUtf8 as EntryType>::Resolved),
	FieldRef(<FieldRef as EntryType>::Resolved),
	MethodRef(<MethodRef as EntryType>::Resolved),
	String(<String as EntryType>::Resolved),
	MethodHandle(<MethodHandle as EntryType>::Resolved),
	MethodType(u32),
}

/// A trait for types that can be stored in the constant pool.

pub trait EntryType: sealed::Sealed {
	/// The final type an entry will resolve to.
	type Resolved;
	type RawEntryType: for<'a> CpEntry<'a>;

	/// Convert the `ResolvedEntry` to the final type.
	#[doc(hidden)]
	fn resolved_entry(entry: ResolvedEntry) -> Self::Resolved;

	/// Resolve the entry at the given index in the constant pool.
	///
	/// # Errors
	///
	/// Certain entry resolutions, such as [`Class`]es and [`Field`]s can throw. Other entries, such
	/// as [`Integer`] cannot, and will always return [`Throws::Ok`].
	#[doc(hidden)]
	fn resolve(
		class: &'static ClassObj,
		cp: &super::ConstantPool,
		index: u2,
	) -> Throws<ResolvedEntry>;

	/// The actual resolution logic
	#[doc(hidden)]
	fn resolve_with(
		class: &'static ClassObj,
		cp: &super::ConstantPool,
		index: u2,
		value: <Self::RawEntryType as CpEntry>::Entry,
	) -> Throws<ResolvedEntry>;
}

pub struct Class;

impl EntryType for Class {
	type Resolved = &'static ClassObj;
	type RawEntryType = raw_types::RawClassName;

	#[inline]
	fn resolved_entry(entry: ResolvedEntry) -> Self::Resolved {
		unsafe { entry.class }
	}

	fn resolve(
		class: &'static ClassObj,
		cp: &super::ConstantPool,
		index: u2,
	) -> Throws<ResolvedEntry> {
		let class_name = cp.raw().get::<Self::RawEntryType>(index);
		Self::resolve_with(class, cp, index, class_name)
	}

	fn resolve_with(
		class: &'static ClassObj,
		cp: &ConstantPool,
		_: u2,
		value: <Self::RawEntryType as CpEntry>::Entry,
	) -> Throws<ResolvedEntry> {
		let name = unsafe { cp.resolve_entry_with::<ConstantUtf8>(value.name_index, value.name)? };

		let class = class.loader().load(name)?;
		Throws::Ok(ResolvedEntry { class })
	}
}

pub struct Integer;

impl EntryType for Integer {
	type Resolved = s4;
	type RawEntryType = raw_types::Integer;

	#[inline]
	fn resolved_entry(entry: ResolvedEntry) -> Self::Resolved {
		unsafe { entry.integer }
	}

	fn resolve(
		class: &'static ClassObj,
		cp: &super::ConstantPool,
		index: u2,
	) -> Throws<ResolvedEntry> {
		let integer = cp.raw().get::<Self::RawEntryType>(index);
		Self::resolve_with(class, cp, index, integer)
	}

	#[inline]
	fn resolve_with(
		_: &'static ClassObj,
		_: &ConstantPool,
		_: u2,
		value: <Self::RawEntryType as CpEntry>::Entry,
	) -> Throws<ResolvedEntry> {
		Throws::Ok(ResolvedEntry { integer: value })
	}
}

pub struct Double;

impl EntryType for Double {
	type Resolved = f64;
	type RawEntryType = raw_types::Double;

	#[inline]
	fn resolved_entry(entry: ResolvedEntry) -> Self::Resolved {
		unsafe { entry.double }
	}

	fn resolve(
		class: &'static ClassObj,
		cp: &super::ConstantPool,
		index: u2,
	) -> Throws<ResolvedEntry> {
		let double = cp.raw().get::<Self::RawEntryType>(index);
		Self::resolve_with(class, cp, index, double)
	}

	#[inline]
	fn resolve_with(
		_: &'static ClassObj,
		_: &ConstantPool,
		_: u2,
		value: <Self::RawEntryType as CpEntry>::Entry,
	) -> Throws<ResolvedEntry> {
		Throws::Ok(ResolvedEntry { double: value })
	}
}

pub struct Float;

impl EntryType for Float {
	type Resolved = f32;
	type RawEntryType = raw_types::Float;

	#[inline]
	fn resolved_entry(entry: ResolvedEntry) -> Self::Resolved {
		unsafe { entry.float }
	}

	fn resolve(
		class: &'static ClassObj,
		cp: &super::ConstantPool,
		index: u2,
	) -> Throws<ResolvedEntry> {
		let float = cp.raw().get::<Self::RawEntryType>(index);
		Self::resolve_with(class, cp, index, float)
	}

	#[inline]
	fn resolve_with(
		_: &'static ClassObj,
		_: &ConstantPool,
		_: u2,
		value: <Self::RawEntryType as CpEntry>::Entry,
	) -> Throws<ResolvedEntry> {
		Throws::Ok(ResolvedEntry { float: value })
	}
}

pub struct Long;

impl EntryType for Long {
	type Resolved = s8;
	type RawEntryType = raw_types::Long;

	#[inline]
	fn resolved_entry(entry: ResolvedEntry) -> Self::Resolved {
		unsafe { entry.long }
	}

	fn resolve(
		class: &'static ClassObj,
		cp: &super::ConstantPool,
		index: u2,
	) -> Throws<ResolvedEntry> {
		let long = cp.raw().get::<Self::RawEntryType>(index);
		Self::resolve_with(class, cp, index, long)
	}

	#[inline]
	fn resolve_with(
		_: &'static ClassObj,
		_: &ConstantPool,
		_: u2,
		value: <Self::RawEntryType as CpEntry>::Entry,
	) -> Throws<ResolvedEntry> {
		Throws::Ok(ResolvedEntry { long: value })
	}
}

pub struct ClassName;

impl EntryType for ClassName {
	type Resolved = Symbol;
	type RawEntryType = raw_types::RawClassName;

	#[inline]
	fn resolved_entry(entry: ResolvedEntry) -> Self::Resolved {
		unsafe { entry.class_name }
	}

	fn resolve(
		class: &'static ClassObj,
		cp: &super::ConstantPool,
		index: u2,
	) -> Throws<ResolvedEntry> {
		let utf8 = cp.raw().get::<Self::RawEntryType>(index);
		Self::resolve_with(class, cp, index, utf8)
	}

	fn resolve_with(
		_: &'static ClassObj,
		_: &ConstantPool,
		_: u2,
		value: <Self::RawEntryType as CpEntry>::Entry,
	) -> Throws<ResolvedEntry> {
		let class_name = Symbol::intern(&*value.name);
		Throws::Ok(ResolvedEntry { class_name })
	}
}

pub struct ConstantUtf8;

impl EntryType for ConstantUtf8 {
	type Resolved = Symbol;
	type RawEntryType = raw_types::RawConstantUtf8;

	#[inline]
	fn resolved_entry(entry: ResolvedEntry) -> Self::Resolved {
		unsafe { entry.constant_utf8 }
	}

	fn resolve(
		class: &'static ClassObj,
		cp: &super::ConstantPool,
		index: u2,
	) -> Throws<ResolvedEntry> {
		let utf8_raw = cp.raw().get::<Self::RawEntryType>(index);
		Self::resolve_with(class, cp, index, utf8_raw)
	}

	fn resolve_with(
		_: &'static ClassObj,
		_: &ConstantPool,
		_: u2,
		value: <Self::RawEntryType as CpEntry>::Entry,
	) -> Throws<ResolvedEntry> {
		let utf8 = Symbol::intern(&*value);
		Throws::Ok(ResolvedEntry {
			constant_utf8: utf8,
		})
	}
}

pub struct NameAndType;

impl EntryType for NameAndType {
	type Resolved = (Symbol, Symbol);
	type RawEntryType = raw_types::RawNameAndType;

	fn resolved_entry(entry: ResolvedEntry) -> Self::Resolved {
		unsafe { entry.name_and_type }
	}

	fn resolve(class: &'static ClassObj, cp: &ConstantPool, index: u2) -> Throws<ResolvedEntry> {
		let name_and_type_raw = cp.raw().get::<Self::RawEntryType>(index);
		Self::resolve_with(class, cp, index, name_and_type_raw)
	}

	fn resolve_with(
		_: &'static ClassObj,
		cp: &ConstantPool,
		_: u2,
		value: <Self::RawEntryType as CpEntry>::Entry,
	) -> Throws<ResolvedEntry> {
		let name_sym =
			unsafe { cp.resolve_entry_with::<ConstantUtf8>(value.name_index, value.name)? };
		let ty_sym = unsafe {
			cp.resolve_entry_with::<ConstantUtf8>(value.descriptor_index, value.descriptor)?
		};
		Throws::Ok(ResolvedEntry {
			name_and_type: (name_sym, ty_sym),
		})
	}
}

pub struct InvokeDynamic;

#[derive(Copy, Clone, Debug)]
pub struct InvokeDynamicEntry {
	pub method: &'static Method,
	pub appendix: Option<&'static Reference>,
}

impl EntryType for InvokeDynamic {
	type Resolved = InvokeDynamicEntry;
	type RawEntryType = raw_types::RawInvokeDynamic;

	#[inline]
	fn resolved_entry(entry: ResolvedEntry) -> Self::Resolved {
		unsafe { entry.invoke_dynamic }.clone()
	}

	fn resolve(
		class: &'static ClassObj,
		cp: &super::ConstantPool,
		index: u2,
	) -> Throws<ResolvedEntry> {
		let raw_invoke_dynamic = cp.raw().get::<Self::RawEntryType>(index);
		Self::resolve_with(class, cp, index, raw_invoke_dynamic)
	}

	fn resolve_with(
		class: &'static ClassObj,
		cp: &ConstantPool,
		_: u2,
		value: <Self::RawEntryType as CpEntry>::Entry,
	) -> Throws<ResolvedEntry> {
		let (name, descriptor) = unsafe {
			cp.resolve_entry_with::<NameAndType>(value.name_and_type_index, value.name_and_type)?
		};

		let name_arg = StringInterner::intern(name);
		let descriptor_str = descriptor.as_str();

		let type_arg;
		if descriptor_str.starts_with('(') {
			type_arg = Method::method_type_for(class, descriptor_str)?;
		} else {
			todo!()
		}

		let appendix = ObjectArrayInstance::new(1, crate::globals::classes::java_lang_Object())?;

		let Some(bootstrap_methods) = class.bootstrap_methods() else {
			panic!("No bootstrap methods found"); // TODO?
		};

		let bootstrap_method = &bootstrap_methods[value.bootstrap_method_attr_index as usize];
		let bsm_handle = cp.get::<MethodHandle>(bootstrap_method.method_handle_index)?;

		let static_args_obj = ObjectArrayInstance::new(
			bootstrap_method.arguments.len() as s4,
			crate::globals::classes::java_lang_Object(),
		)?;

		let thread = JavaThread::current();
		for (index, arg) in bootstrap_method.arguments.iter().enumerate() {
			let r;
			match arg.value {
				LoadableConstantPoolValueInner::Integer(val) => r = val.into_box(thread)?,
				LoadableConstantPoolValueInner::Float(val) => r = val.into_box(thread)?,
				LoadableConstantPoolValueInner::Long(val) => r = val.into_box(thread)?,
				LoadableConstantPoolValueInner::Double(val) => r = val.into_box(thread)?,
				LoadableConstantPoolValueInner::Class(_) => todo!("Class static argument"),
				LoadableConstantPoolValueInner::String(ref val) => {
					let sym = Symbol::intern(val);
					r = Reference::class(StringInterner::intern(sym));
				},
				LoadableConstantPoolValueInner::MethodHandle(_) => {
					r = cp.get::<MethodHandle>(arg.index)?;
				},
				LoadableConstantPoolValueInner::MethodType(ref val) => {
					let sym = Symbol::intern(val);
					r = Method::method_type_for(class, sym.as_str())?;
				},
				LoadableConstantPoolValueInner::Dynamic(_) => todo!("Dynamic static argument"),
			}

			// SAFETY: We just created the array, we know that none of the indices will be out of bounds
			unsafe { static_args_obj.get_mut().store_unchecked(index, r) };
		}

		let link_call_site_method = crate::globals::classes::java_lang_invoke_MethodHandleNatives()
			.resolve_method(sym!(linkCallSite), sym!(linkCallSite_signature))?;

		let result = java_call!(
			thread,
			link_call_site_method,
			Operand::Reference(Reference::mirror(class.mirror())),
			Operand::Reference(bsm_handle),
			Operand::Reference(Reference::class(name_arg)),
			Operand::Reference(type_arg),
			Operand::Reference(Reference::object_array(static_args_obj)),
			Operand::Reference(Reference::object_array(appendix.clone())),
		);

		if thread.has_pending_exception() {
			return Throws::PENDING_EXCEPTION;
		}

		let member_name = result
			.expect("method should return something")
			.expect_reference();

		'invalid: {
			if member_name.is_null() {
				break 'invalid;
			}

			let resolved_method_name =
				classes::java::lang::invoke::MemberName::method(member_name.extract_class().get());
			if resolved_method_name.is_null() {
				break 'invalid;
			}

			let vmtarget = classes::java::lang::invoke::ResolvedMethodName::vmtarget(
				resolved_method_name.extract_class().get(),
			);
			let Some(target) = vmtarget else {
				break 'invalid;
			};

			let appendix = appendix.get().get(0)?;
			let appendix_opt = if appendix.is_null() {
				None
			} else {
				let leaked: &'static Reference = Box::leak(Box::new(appendix));
				Some(leaked)
			};

			return Throws::Ok(ResolvedEntry {
				invoke_dynamic: InvokeDynamicEntry {
					method: target,
					appendix: appendix_opt,
				},
			});
		}

		// TODO: The same LinkageError should be thrown for subsequent calls
		throw!(@DEFER LinkageError, "MethodHandleNatives produced a bad value");
	}
}

pub struct MethodHandle;

impl EntryType for MethodHandle {
	type Resolved = Reference;
	type RawEntryType = raw_types::RawMethodHandle;

	#[inline]
	fn resolved_entry(entry: ResolvedEntry) -> Self::Resolved {
		unsafe { entry.method_handle }.clone()
	}

	fn resolve(
		class: &'static ClassObj,
		cp: &super::ConstantPool,
		index: u2,
	) -> Throws<ResolvedEntry> {
		let raw_method_handle = cp.raw().get::<Self::RawEntryType>(index);
		Self::resolve_with(class, cp, index, raw_method_handle)
	}

	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-5.html#jvms-5.4.3.5
	fn resolve_with(
		invoking_class: &'static ClassObj,
		cp: &ConstantPool,
		_: u2,
		value: <Self::RawEntryType as CpEntry>::Entry,
	) -> Throws<ResolvedEntry> {
		let callee_class;
		let name;
		let descriptor;

		match value.reference_kind {
			ReferenceKind::GetField
			| ReferenceKind::GetStatic
			| ReferenceKind::PutField
			| ReferenceKind::PutStatic => {
				todo!("MH of kind field");
			},
			ReferenceKind::InvokeVirtual
			| ReferenceKind::NewInvokeSpecial
			| ReferenceKind::InvokeStatic
			| ReferenceKind::InvokeSpecial
			| ReferenceKind::InvokeInterface => {
				let ReferenceEntry::MethodRef(method_ref) = value.reference else {
					panic!("Expected a method reference"); // TODO: Exception and set failure
				};

				callee_class = cp.get::<Class>(method_ref.class_index)?;
				(name, descriptor) = unsafe {
					cp.resolve_entry_with::<NameAndType>(
						method_ref.name_and_type_index,
						method_ref.name_and_type,
					)?
				};
			},
		}

		if name == sym!(class_initializer_name) {
			throw!(@DEFER IllegalArgumentException, "method handles cannot link to class initializer");
		}

		let member_name = MethodHandleNatives::new_member_name(name, descriptor, callee_class)?;

		MethodHandleNatives::resolve_member_name(
			member_name.get_mut(),
			value.reference_kind,
			Some(invoking_class),
			0,
		)?;

		let ty_arg = Method::method_type_for(invoking_class, descriptor.as_str())?;

		let link_method_handle_constant_method =
			crate::globals::classes::java_lang_invoke_MethodHandleNatives().resolve_method(
				sym!(linkMethodHandleConstant),
				sym!(linkMethodHandleConstant_signature),
			)?;

		let thread = JavaThread::current();
		let result = java_call!(
			thread,
			link_method_handle_constant_method,
			Operand::Reference(Reference::mirror(invoking_class.mirror())),
			Operand::Int(value.reference_kind as i32),
			Operand::Reference(Reference::mirror(callee_class.mirror())),
			Operand::Reference(Reference::class(StringInterner::intern(name))),
			Operand::Reference(ty_arg),
		);

		if thread.has_pending_exception() {
			return Throws::PENDING_EXCEPTION;
		}

		let method_handle = result
			.expect("method should return something")
			.expect_reference();

		Throws::Ok(ResolvedEntry {
			method_handle: Box::leak(Box::new(method_handle)),
		})
	}
}

pub struct FieldRef;

impl EntryType for FieldRef {
	type Resolved = &'static Field;
	type RawEntryType = raw_types::RawFieldRef;

	#[inline]
	fn resolved_entry(entry: ResolvedEntry) -> Self::Resolved {
		unsafe { entry.field_ref }
	}

	fn resolve(
		class: &'static ClassObj,
		cp: &super::ConstantPool,
		index: u2,
	) -> Throws<ResolvedEntry> {
		let raw_field_ref = cp.raw().get::<raw_types::RawFieldRef>(index);
		Self::resolve_with(class, cp, index, raw_field_ref)
	}

	fn resolve_with(
		_: &'static ClassObj,
		cp: &ConstantPool,
		_: u2,
		value: <Self::RawEntryType as CpEntry>::Entry,
	) -> Throws<ResolvedEntry> {
		let class = cp.get::<Class>(value.class_index)?;
		let (name, descriptor) = unsafe {
			cp.resolve_entry_with::<NameAndType>(value.name_and_type_index, value.name_and_type)?
		};

		let field = class.resolve_field(name, descriptor)?;
		Throws::Ok(ResolvedEntry { field_ref: field })
	}
}

#[derive(Copy, Clone, Debug)]
pub struct MethodEntry {
	pub method: &'static Method,
	pub descriptor: Symbol,
	/// The number of parameters this method takes
	///
	/// This is needed for variadic methods, saving the effort of parsing the descriptor on
	/// every call.
	pub parameter_count: u1,
	/// The number of stack slots that the parameters take up
	pub parameters_stack_size: u2,
}

/// A resolved `CONSTANT_Methodref` entry
///
/// This resolves into a tuple of a [`Method`] and a [`Symbol`] representing the method's descriptor.
///
/// The descriptor is necessary for [signature polymorphic] methods, where the method's descriptor
/// is used to determine the types of the parameters at runtime. Otherwise, the descriptor can be
/// grabbed from [`Method::descriptor_sym()`].
///
/// [signature polymorphic]: https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.9.3
pub struct MethodRef;

impl EntryType for MethodRef {
	type Resolved = MethodEntry;
	type RawEntryType = raw_types::RawMethodRef;

	#[inline]
	fn resolved_entry(entry: ResolvedEntry) -> Self::Resolved {
		unsafe { entry.method_ref }
	}

	fn resolve(
		class: &'static ClassObj,
		cp: &super::ConstantPool,
		index: u2,
	) -> Throws<ResolvedEntry> {
		let raw_method_ref = cp.raw().get::<Self::RawEntryType>(index);
		Self::resolve_with(class, cp, index, raw_method_ref)
	}

	fn resolve_with(
		_: &'static ClassObj,
		cp: &ConstantPool,
		_: u2,
		value: <Self::RawEntryType as CpEntry>::Entry,
	) -> Throws<ResolvedEntry> {
		let class = unsafe { cp.resolve_entry_with::<Class>(value.class_index, value.class)? };
		let (name, descriptor) = unsafe {
			cp.resolve_entry_with::<NameAndType>(value.name_and_type_index, value.name_and_type)?
		};

		let method_ref;
		if value.is_interface {
			method_ref = class.resolve_interface_method(name, descriptor)?;
		} else {
			method_ref = class.resolve_method(name, descriptor)?;
		}

		let (parameter_count, parameters_stack_size) = if method_ref.is_var_args() {
			let descriptor = MethodDescriptor::parse(&mut descriptor.as_bytes())
				.expect("an invalid descriptor shouldn't make it this far");

			let stack_size = descriptor
				.parameters
				.iter()
				.map(|ty| ty.stack_size() as u2)
				.sum();
			(descriptor.parameters.len() as u1, stack_size)
		} else {
			let stack_size = method_ref
				.descriptor
				.parameters
				.iter()
				.map(|ty| ty.stack_size() as u2)
				.sum();
			(method_ref.parameter_count(), stack_size)
		};

		let entry = MethodEntry {
			method: method_ref,
			descriptor,
			parameter_count,
			parameters_stack_size,
		};

		Throws::Ok(ResolvedEntry { method_ref: entry })
	}
}

pub struct String;

impl EntryType for String {
	type Resolved = Symbol;
	type RawEntryType = raw_types::RawString;

	#[inline]
	fn resolved_entry(entry: ResolvedEntry) -> Self::Resolved {
		unsafe { entry.string }
	}

	fn resolve(
		class: &'static ClassObj,
		cp: &super::ConstantPool,
		index: u2,
	) -> Throws<ResolvedEntry> {
		let string_raw = cp.raw().get::<raw_types::RawString>(index);
		Self::resolve_with(class, cp, index, string_raw)
	}

	fn resolve_with(
		_: &'static ClassObj,
		_: &ConstantPool,
		_: u2,
		value: <Self::RawEntryType as CpEntry>::Entry,
	) -> Throws<ResolvedEntry> {
		let string = Symbol::intern(&*value);
		Throws::Ok(ResolvedEntry { string })
	}
}

mod sealed {
	use super::*;

	pub trait Sealed {}

	impl Sealed for Class {}
	impl Sealed for Integer {}
	impl Sealed for Double {}
	impl Sealed for Float {}
	impl Sealed for Long {}
	impl Sealed for ClassName {}
	impl Sealed for NameAndType {}
	impl Sealed for ConstantUtf8 {}
	impl Sealed for InvokeDynamic {}
	impl Sealed for MethodHandle {}
	impl Sealed for FieldRef {}
	impl Sealed for MethodRef {}
	impl Sealed for String {}
}
