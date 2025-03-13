use super::cp_types;
use crate::objects::class::Class;
use crate::objects::constant_pool::cp_types::{InvokeDynamicEntry, MethodEntry};
use crate::objects::field::Field;
use crate::objects::reference::Reference;
use crate::symbols::Symbol;
use crate::thread::exceptions::Throws;

use std::cell::UnsafeCell;
use std::sync::Mutex;

use classfile::constant_pool::types::CpEntry;
use common::int_types::{s4, s8, u2};

#[derive(Copy, Clone)]
pub(super) union ResolvedEntry {
	pub(super) integer: s4,
	pub(super) double: f64,
	pub(super) float: f32,
	pub(super) long: s8,
	pub(super) class: &'static Class,
	pub(super) class_name: Symbol,
	pub(super) name_and_type: (Symbol, Symbol),
	pub(super) constant_utf8: Symbol,
	pub(super) field_ref: &'static Field,
	pub(super) invoke_dynamic: InvokeDynamicEntry,
	pub(super) method_ref: MethodEntry,
	pub(super) method_handle: &'static Reference,
	pub(super) string: Symbol,
}

pub(super) struct ConstantPoolEntry {
	failed: UnsafeCell<bool>,
	resolved: UnsafeCell<Option<ResolvedEntry>>,
	_resolution_lock: Mutex<()>,
}

impl ConstantPoolEntry {
	pub(super) fn new() -> Self {
		Self {
			failed: UnsafeCell::new(false),
			resolved: UnsafeCell::new(None),
			_resolution_lock: Mutex::new(()),
		}
	}

	pub(super) fn resolved<T: cp_types::EntryType>(&self) -> Option<T::Resolved> {
		match self.resolved_field() {
			Some(resolved) => Some(T::resolved_entry(resolved)),
			None => None,
		}
	}

	pub(super) fn resolve<T: cp_types::EntryType>(
		&self,
		class: &'static Class,
		cp: &super::ConstantPool,
		index: u2,
	) -> Throws<T::Resolved> {
		let _guard = self
			._resolution_lock
			.try_lock()
			.expect("re-entrant resolution");

		// Some other thread beat us here
		if let Some(resolved) = self.resolved::<T>() {
			return Throws::Ok(resolved);
		}

		if self.failed() {
			panic!("Resolution error"); // TODO: Exception
		}

		let resolved = T::resolve(class, cp, index)?;
		unsafe {
			self.set_resolved(resolved);
		}

		Throws::Ok(T::resolved_entry(resolved))
	}

	pub(super) unsafe fn resolve_with<T: cp_types::EntryType>(
		&self,
		class: &'static Class,
		cp: &super::ConstantPool,
		index: u2,
		value: <T::RawEntryType as CpEntry>::Entry,
	) -> Throws<T::Resolved> {
		let resolved = T::resolve_with(class, cp, index, value)?;
		unsafe {
			self.set_resolved(resolved);
		}

		Throws::Ok(T::resolved_entry(resolved))
	}

	unsafe fn set_resolved(&self, resolved: ResolvedEntry) {
		unsafe { *self.resolved.get() = Some(resolved) };
	}

	fn set_failed(&self) {
		unsafe { *self.failed.get() = true };
	}

	fn failed(&self) -> bool {
		unsafe { *self.failed.get() }
	}

	fn resolved_field(&self) -> Option<ResolvedEntry> {
		unsafe { *self.resolved.get() }
	}
}
