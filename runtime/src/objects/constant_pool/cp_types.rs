use super::entry::ResolvedEntry;
use crate::java_call;
use crate::objects::array::ArrayInstance;
use crate::objects::class::Class as ClassObj;
use crate::objects::constant_pool::ConstantPool;
use crate::objects::field::Field;
use crate::objects::method::Method;
use crate::objects::reference::Reference;
use crate::string_interner::StringInterner;
use crate::thread::exceptions::{throw, Throws};
use crate::thread::JavaThread;

use classfile::constant_pool::types::{raw as raw_types, CpEntry, ReferenceEntry, ReferenceKind};
use classfile::{FieldType, MethodDescriptor};
use common::int_types::{s4, s8, u1, u2};
use common::traits::PtrType;
use instructions::Operand;
use symbols::{sym, Symbol};

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
	MethodHandle(u32),
	MethodType(u32),
}

/// A trait for types that can be stored in the constant pool.
#[expect(private_interfaces)]
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

#[expect(private_interfaces)]
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
		index: u2,
		value: <Self::RawEntryType as CpEntry>::Entry,
	) -> Throws<ResolvedEntry> {
		let name = unsafe { cp.resolve_entry_with::<ConstantUtf8>(value.name_index, value.name)? };

		let class = class.loader().load(name)?;
		Throws::Ok(ResolvedEntry { class })
	}
}

pub struct Integer;

#[expect(private_interfaces)]
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

#[expect(private_interfaces)]
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

#[expect(private_interfaces)]
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

#[expect(private_interfaces)]
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

#[expect(private_interfaces)]
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
		let class_name = Symbol::intern_bytes(&*value.name);
		Throws::Ok(ResolvedEntry { class_name })
	}
}

pub struct ConstantUtf8;

#[expect(private_interfaces)]
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
		let utf8 = Symbol::intern_bytes(&*value);
		Throws::Ok(ResolvedEntry {
			constant_utf8: utf8,
		})
	}
}

pub struct NameAndType;

#[expect(private_interfaces)]
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

#[expect(private_interfaces)]
impl EntryType for InvokeDynamic {
	type Resolved = Reference;
	type RawEntryType = raw_types::RawInvokeDynamic;

	#[inline]
	fn resolved_entry(entry: ResolvedEntry) -> Self::Resolved {
		// unsafe { entry.invoke_dynamic }
		todo!()
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
		index: u2,
		value: <Self::RawEntryType as CpEntry>::Entry,
	) -> Throws<ResolvedEntry> {
		let (name, descriptor) = unsafe {
			cp.resolve_entry_with::<NameAndType>(value.name_and_type_index, value.name_and_type)?
		};

		let name_arg = StringInterner::intern_symbol(name);
		let descriptor_str = descriptor.as_str();

		let type_arg;
		if descriptor_str.starts_with('(') {
			type_arg = Method::method_type_for(class, descriptor_str)?;
		} else {
			todo!()
		}

		let appendix =
			ArrayInstance::new_reference(1, crate::globals::classes::java_lang_Object())?;

		let Some(bootstrap_methods) = class.bootstrap_methods() else {
			panic!("No bootstrap methods found"); // TODO?
		};

		let bootstrap_method = &bootstrap_methods[value.bootstrap_method_attr_index as usize];
		let bsm_handle = cp.get::<MethodHandle>(bootstrap_method.method_handle_index)?;

		let link_call_site_method = crate::globals::classes::java_lang_invoke_MethodHandleNatives()
			.resolve_method(sym!(linkCallSite), sym!(linkCallSite_signature))?;

		let result = java_call!(
			JavaThread::current(),
			link_call_site_method,
			Operand::Reference(Reference::mirror(class.mirror())),
			Operand::Reference(bsm_handle),
			Operand::Reference(Reference::class(name_arg)),
			Operand::Reference(type_arg),
			// TODO: /* static args*/,
			Operand::Reference(Reference::array(appendix)),
		)
		.expect("method should return something")
		.expect_reference();

		if result.is_null() {
			throw!(@DEFER LinkageError, "MethodHandleNatives produced a bad value");
		}

		todo!("invoke_dynamic resolution")
	}
}

pub struct MethodHandle;

#[expect(private_interfaces)]
impl EntryType for MethodHandle {
	type Resolved = Reference;
	type RawEntryType = raw_types::RawMethodHandle;

	#[inline]
	fn resolved_entry(entry: ResolvedEntry) -> Self::Resolved {
		// unsafe { entry.method_handle }
		todo!()
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
		match value.reference_kind {
			ReferenceKind::GetField
			| ReferenceKind::GetStatic
			| ReferenceKind::PutField
			| ReferenceKind::PutStatic => {
				let ReferenceEntry::FieldRef(field) = value.reference else {
					panic!("Expected a field reference"); // TODO: Exception and set failure
				};

				let _class = cp.get::<Class>(field.class_index)?;
				let (_name, _descriptor) = unsafe {
					cp.resolve_entry_with::<NameAndType>(
						field.name_and_type_index,
						field.name_and_type,
					)?
				};
				todo!("MH of kind field");
			},
			ReferenceKind::InvokeVirtual
			| ReferenceKind::NewInvokeSpecial
			| ReferenceKind::InvokeStatic
			| ReferenceKind::InvokeSpecial => {
				let ReferenceEntry::MethodRef(method) = value.reference else {
					panic!("Expected a method reference"); // TODO: Exception and set failure
				};

				let class = cp.get::<Class>(method.class_index)?;
				let (name, descriptor) = unsafe {
					cp.resolve_entry_with::<NameAndType>(
						method.name_and_type_index,
						method.name_and_type,
					)?
				};

				let method = class.resolve_method(name, descriptor)?;

				let mut is_valid = true;
				match value.reference_kind {
					ReferenceKind::InvokeSpecial => {
						is_valid = method.class() == invoking_class
							|| class
								.parent_iter()
								.any(|super_class| super_class == invoking_class)
							|| class
								.interfaces
								.iter()
								.any(|interface| *interface == invoking_class)
							|| method.class() == crate::globals::classes::java_lang_Object();
					},
					ReferenceKind::NewInvokeSpecial => {
						is_valid = method.name == sym!(object_initializer_name);
						if method.is_protected() {
							is_valid &= method.class().shares_package_with(invoking_class);
						} else {
							is_valid &= method.class() == class;
						}
					},
					ReferenceKind::InvokeStatic => {
						is_valid = method.is_static();
					},
					ReferenceKind::InvokeVirtual => {
						if method.is_protected()
							&& !method.class().shares_package_with(invoking_class)
						{
							is_valid = method
								.class()
								.parent_iter()
								.any(|super_class| super_class == invoking_class);
						}
					},
					_ => unreachable!(),
				}

				if !is_valid {
					throw!(@DEFER IllegalAccessError);
				}

				if method.is_var_args() {
					todo!("varargs method handle");
				}

				todo!("MH of kind method");
			},
			ReferenceKind::InvokeInterface => {
				let ReferenceEntry::MethodRef(method) = value.reference else {
					panic!("Expected a method reference"); // TODO: Exception and set failure
				};
				todo!("MH of kind interface method");
			},
		}
		todo!("Method handle resolution")
	}
}

pub struct FieldRef;

#[expect(private_interfaces)]
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

pub struct MethodRef;

#[expect(private_interfaces)]
impl EntryType for MethodRef {
	type Resolved = &'static Method;
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

		Throws::Ok(ResolvedEntry { method_ref })
	}
}

pub struct String;

#[expect(private_interfaces)]
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
		let string = Symbol::intern_bytes(&*value);
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
