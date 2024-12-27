use super::cp_types;
use crate::objects::class::Class;
use crate::objects::field::Field;
use crate::objects::method::Method;

use std::cell::UnsafeCell;
use std::sync::Mutex;

use common::int_types::{s4, s8, u2};
use symbols::Symbol;

#[derive(Copy, Clone)]
pub(super) union ResolvedEntry {
	pub(super) integer: s4,
	pub(super) double: f64,
	pub(super) float: f32,
	pub(super) long: s8,
	pub(super) class: &'static Class,
	pub(super) class_name: Symbol,
	pub(super) constant_utf8: Symbol,
	pub(super) field_ref: &'static Field,
	pub(super) method_ref: &'static Method,
	pub(super) string: Symbol,
}

pub(super) struct ConstantPoolEntry {
	resolved: UnsafeCell<Option<ResolvedEntry>>,
	_resolution_lock: Mutex<()>,
}

impl ConstantPoolEntry {
	pub(super) fn new() -> Self {
		Self {
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
	) -> T::Resolved {
		let _guard = self._resolution_lock.lock().unwrap();

		// Some other thread beat us here
		if self.resolved_field().is_some() {
			return self.resolved::<T>().unwrap();
		}

		// We know that there will be a resolved entry, as `EntryType::resolve` will panic on failure
		let resolved = T::resolve(class, cp, index);
		unsafe {
			*self.resolved.get() = Some(resolved);
		}

		T::resolved_entry(resolved)
	}

	fn resolved_field(&self) -> Option<ResolvedEntry> {
		unsafe { *self.resolved.get() }
	}
}
