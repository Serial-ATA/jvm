pub mod cp_types;
use cp_types::Entry;
mod entry;
pub use entry::ResolvedEntry;

use crate::objects::class::ClassPtr;
use crate::thread::exceptions::Throws;

use std::fmt::{Debug, Formatter};

use classfile::constant_pool::ConstantPoolValueInfo;
use classfile::constant_pool::types::CpEntry;
use common::box_slice;
use common::int_types::u2;

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.4

/// The runtime constant pool for a class
///
/// This provides two things:
///
/// * A type-safe interface to the constant pool
/// * A cache of resolved constant pool entries
///
/// The cache will ensure that each entry is only resolved once, and that the same instance is
/// returned each time.
///
/// This is important for types like `Class` which are loaded once and then shared between all
/// instances of the class.
pub struct ConstantPool {
	class: ClassPtr,
	entries: Box<[entry::ConstantPoolEntry]>,
	raw: classfile::constant_pool::ConstantPool,
}

impl ConstantPool {
	pub fn new(class: ClassPtr, cp: classfile::constant_pool::ConstantPool) -> Self {
		Self {
			class,
			entries: box_slice![entry::ConstantPoolEntry::new(); cp.len()],
			raw: cp,
		}
	}

	// TODO: Ideally have an entry api, so that the caller can check whether the value is resolved, and if not, resolve it.
	//       Having `get` potentially throw isn't very good.

	/// Get a constant pool entry of a specific type
	///
	/// See [`ConstantPool`] for notes on resolution.
	pub fn get<T: cp_types::EntryType>(&self, index: u2) -> Throws<T::Resolved> {
		let entry = &self.entries[index as usize];
		if let Some(resolved) = entry.resolved::<T>() {
			return Throws::Ok(resolved);
		}

		entry.resolve::<T>(self.class, &self, index)
	}

	/// Overwrite the entry at `index` with a new value
	///
	/// This should be used sparingly.
	///
	/// # Safety
	///
	/// The caller must ensure that the entry at `index` is of type `T`. Otherwise, the behavior is
	/// undefined.
	pub unsafe fn overwrite<T: cp_types::EntryType>(&self, index: u2, value: ResolvedEntry) {
		let entry = &self.entries[index as usize];
		unsafe { entry.set_resolved(value) }
	}

	unsafe fn resolve_entry_with<T: cp_types::EntryType>(
		&self,
		index: u2,
		value: <T::RawEntryType as CpEntry>::Entry,
	) -> Throws<T::Resolved> {
		let entry = &self.entries[index as usize];
		unsafe { entry.resolve_with::<T>(self.class, self, index, value) }
	}

	/// Get a constant pool entry of any type
	///
	/// See [`ConstantPool`] for notes on resolution.
	pub fn get_any(&self, index: u2) -> Throws<Entry> {
		let raw = &self.raw[index as usize];
		match raw {
			ConstantPoolValueInfo::Class { .. } => {
				self.get::<cp_types::Class>(index).map(Entry::Class)
			},
			ConstantPoolValueInfo::Fieldref { .. } => {
				self.get::<cp_types::FieldRef>(index).map(Entry::FieldRef)
			},
			ConstantPoolValueInfo::Methodref { .. } => {
				self.get::<cp_types::MethodRef>(index).map(Entry::MethodRef)
			},
			ConstantPoolValueInfo::String { .. } => {
				self.get::<cp_types::String>(index).map(Entry::String)
			},
			ConstantPoolValueInfo::Integer { .. } => {
				self.get::<cp_types::Integer>(index).map(Entry::Integer)
			},
			ConstantPoolValueInfo::Float { .. } => {
				self.get::<cp_types::Float>(index).map(Entry::Float)
			},
			ConstantPoolValueInfo::Long { .. } => {
				self.get::<cp_types::Long>(index).map(Entry::Long)
			},
			ConstantPoolValueInfo::Double { .. } => {
				self.get::<cp_types::Double>(index).map(Entry::Double)
			},
			ConstantPoolValueInfo::Utf8 { .. } => self
				.get::<cp_types::ConstantUtf8>(index)
				.map(Entry::ConstantUtf8),
			ConstantPoolValueInfo::MethodHandle { .. }
			| ConstantPoolValueInfo::MethodType { .. }
			| ConstantPoolValueInfo::InterfaceMethodref { .. }
			| ConstantPoolValueInfo::Dynamic { .. }
			| ConstantPoolValueInfo::InvokeDynamic { .. }
			| ConstantPoolValueInfo::Module { .. }
			| ConstantPoolValueInfo::Package { .. } => todo!(),
			ConstantPoolValueInfo::NameAndType { .. } => {
				unreachable!("NameAndType entries should not be resolved directly")
			},
			ConstantPoolValueInfo::Unusable => unreachable!(),
		}
	}

	pub(super) fn raw(&self) -> &classfile::constant_pool::ConstantPool {
		&self.raw
	}
}

impl Debug for ConstantPool {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.raw.iter()).finish()
	}
}
