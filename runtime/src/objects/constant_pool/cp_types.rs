use super::entry::ResolvedEntry;
use crate::objects::class::Class as ClassObj;
use crate::objects::field::Field;
use crate::objects::method::Method;

use common::int_types::{s4, s8, u2};
use symbols::Symbol;

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
pub trait EntryType: sealed::Sealed {
	type Resolved;

	#[doc(hidden)]
	fn resolved_entry(entry: ResolvedEntry) -> Self::Resolved;

	/// Resolve the entry at the given index in the constant pool.
	///
	/// # Panics
	///
	/// This will panic in the event of any resolution failure. There should never be a case
	/// where a *valid* constant pool entry cannot be resolved.
	#[doc(hidden)]
	fn resolve(class: &'static ClassObj, cp: &super::ConstantPool, index: u2) -> ResolvedEntry;
}

pub struct Class;

impl EntryType for Class {
	type Resolved = &'static ClassObj;

	#[inline]
	fn resolved_entry(entry: ResolvedEntry) -> Self::Resolved {
		unsafe { entry.class }
	}

	fn resolve(class: &'static ClassObj, cp: &super::ConstantPool, index: u2) -> ResolvedEntry {
		let entry = ClassName::resolve(class, cp, index);
		let name = ClassName::resolved_entry(entry);

		let class = class.loader.load(name).unwrap();
		ResolvedEntry { class }
	}
}

pub struct Integer;

impl EntryType for Integer {
	type Resolved = s4;

	#[inline]
	fn resolved_entry(entry: ResolvedEntry) -> Self::Resolved {
		unsafe { entry.integer }
	}

	fn resolve(_: &'static ClassObj, cp: &super::ConstantPool, index: u2) -> ResolvedEntry {
		let integer = cp.raw().get_integer(index);
		ResolvedEntry { integer }
	}
}

pub struct Double;

impl EntryType for Double {
	type Resolved = f64;

	#[inline]
	fn resolved_entry(entry: ResolvedEntry) -> Self::Resolved {
		unsafe { entry.double }
	}

	fn resolve(_: &'static ClassObj, cp: &super::ConstantPool, index: u2) -> ResolvedEntry {
		let double = cp.raw().get_double(index);
		ResolvedEntry { double }
	}
}

pub struct Float;

impl EntryType for Float {
	type Resolved = f32;

	#[inline]
	fn resolved_entry(entry: ResolvedEntry) -> Self::Resolved {
		unsafe { entry.float }
	}

	fn resolve(_: &'static ClassObj, cp: &super::ConstantPool, index: u2) -> ResolvedEntry {
		let float = cp.raw().get_float(index);
		ResolvedEntry { float }
	}
}

pub struct Long;

impl EntryType for Long {
	type Resolved = s8;

	#[inline]
	fn resolved_entry(entry: ResolvedEntry) -> Self::Resolved {
		unsafe { entry.long }
	}

	fn resolve(_: &'static ClassObj, cp: &super::ConstantPool, index: u2) -> ResolvedEntry {
		let long = cp.raw().get_long(index);
		ResolvedEntry { long }
	}
}

pub struct ClassName;

impl EntryType for ClassName {
	type Resolved = Symbol;

	#[inline]
	fn resolved_entry(entry: ResolvedEntry) -> Self::Resolved {
		unsafe { entry.class_name }
	}

	fn resolve(_: &'static ClassObj, cp: &super::ConstantPool, index: u2) -> ResolvedEntry {
		let utf8 = cp.raw().get_class_name(index);

		let class_name = Symbol::intern_bytes(utf8);
		ResolvedEntry { class_name }
	}
}

pub struct ConstantUtf8;

impl EntryType for ConstantUtf8 {
	type Resolved = Symbol;

	#[inline]
	fn resolved_entry(entry: ResolvedEntry) -> Self::Resolved {
		unsafe { entry.constant_utf8 }
	}

	fn resolve(_: &'static ClassObj, cp: &super::ConstantPool, index: u2) -> ResolvedEntry {
		let utf8_raw = cp.raw().get_constant_utf8(index);

		let utf8 = Symbol::intern_bytes(utf8_raw);
		ResolvedEntry {
			constant_utf8: utf8,
		}
	}
}

pub struct FieldRef;

impl EntryType for FieldRef {
	type Resolved = &'static Field;

	#[inline]
	fn resolved_entry(entry: ResolvedEntry) -> Self::Resolved {
		unsafe { entry.field_ref }
	}

	fn resolve(class: &'static ClassObj, cp: &super::ConstantPool, index: u2) -> ResolvedEntry {
		let (class_index, name_and_type_index) = cp.raw().get_field_ref(index);

		let class_entry = Class::resolve(class, cp, class_index);
		let class = Class::resolved_entry(class_entry);

		let (name_index, descriptor_index) = cp.raw().get_name_and_type(name_and_type_index);

		let name_entry = ConstantUtf8::resolve(class, cp, name_index);
		let name = ConstantUtf8::resolved_entry(name_entry);

		let descriptor_entry = ConstantUtf8::resolve(class, cp, descriptor_index);
		let descriptor = ConstantUtf8::resolved_entry(descriptor_entry);

		let field = class.resolve_field(name, descriptor).unwrap();
		ResolvedEntry { field_ref: field }
	}
}

pub struct MethodRef;

impl EntryType for MethodRef {
	type Resolved = &'static Method;

	#[inline]
	fn resolved_entry(entry: ResolvedEntry) -> Self::Resolved {
		unsafe { entry.method_ref }
	}

	fn resolve(class: &'static ClassObj, cp: &super::ConstantPool, index: u2) -> ResolvedEntry {
		let (is_interface, class_index, name_and_type_index) = cp.raw().get_method_ref(index);

		let class_entry = Class::resolve(class, cp, class_index);
		let class = Class::resolved_entry(class_entry);

		let (name_index, descriptor_index) = cp.raw().get_name_and_type(name_and_type_index);

		let name_entry = ConstantUtf8::resolve(class, cp, name_index);
		let name = ConstantUtf8::resolved_entry(name_entry);

		let descriptor_entry = ConstantUtf8::resolve(class, cp, descriptor_index);
		let descriptor = ConstantUtf8::resolved_entry(descriptor_entry);

		let method_ref = ClassObj::resolve_method(class, is_interface, name, descriptor).unwrap();
		ResolvedEntry { method_ref }
	}
}

pub struct String;

impl EntryType for String {
	type Resolved = Symbol;

	#[inline]
	fn resolved_entry(entry: ResolvedEntry) -> Self::Resolved {
		unsafe { entry.string }
	}

	fn resolve(_: &'static ClassObj, cp: &super::ConstantPool, index: u2) -> ResolvedEntry {
		let string_raw = cp.raw().get_string(index);

		let string = Symbol::intern_bytes(string_raw);
		ResolvedEntry { string }
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
	impl Sealed for ConstantUtf8 {}
	impl Sealed for FieldRef {}
	impl Sealed for MethodRef {}
	impl Sealed for String {}
}
