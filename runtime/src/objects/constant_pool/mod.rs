pub mod cp_types;
mod entry;

use crate::objects::class::Class;
use cp_types::Entry;

use std::fmt::{Debug, Formatter};
use std::sync::Arc;

use classfile::ConstantPoolValueInfo;
use common::box_slice;
use common::int_types::u2;

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.4

pub struct ConstantPool {
	class: &'static Class,
	entries: Box<[entry::ConstantPoolEntry]>,
	raw: classfile::ConstantPool,
}

impl ConstantPool {
	pub fn new(class: &'static Class, cp: classfile::ConstantPool) -> Self {
		Self {
			class,
			entries: box_slice![entry::ConstantPoolEntry::new(); cp.len()],
			raw: cp,
		}
	}

	pub fn get<T: cp_types::EntryType>(&self, index: u2) -> T::Resolved {
		let entry = &self.entries[index as usize];
		if let Some(resolved) = entry.resolved::<T>() {
			return resolved;
		}

		entry.resolve::<T>(self.class, &self, index)
	}

	pub fn get_any(&self, index: u2) -> Entry {
		let raw = &self.raw[index as usize];
		match raw {
			ConstantPoolValueInfo::Class { .. } => Entry::Class(self.get::<cp_types::Class>(index)),
			ConstantPoolValueInfo::Fieldref { .. } => {
				Entry::FieldRef(self.get::<cp_types::FieldRef>(index))
			},
			ConstantPoolValueInfo::Methodref { .. } => {
				Entry::MethodRef(self.get::<cp_types::MethodRef>(index))
			},
			ConstantPoolValueInfo::String { .. } => {
				Entry::String(self.get::<cp_types::String>(index))
			},
			ConstantPoolValueInfo::Integer { .. } => {
				Entry::Integer(self.get::<cp_types::Integer>(index))
			},
			ConstantPoolValueInfo::Float { .. } => Entry::Float(self.get::<cp_types::Float>(index)),
			ConstantPoolValueInfo::Long { .. } => Entry::Long(self.get::<cp_types::Long>(index)),
			ConstantPoolValueInfo::Double { .. } => {
				Entry::Double(self.get::<cp_types::Double>(index))
			},
			ConstantPoolValueInfo::Utf8 { .. } => {
				Entry::ConstantUtf8(self.get::<cp_types::ConstantUtf8>(index))
			},
			ConstantPoolValueInfo::MethodHandle { .. }
			| ConstantPoolValueInfo::MethodType { .. }
			| ConstantPoolValueInfo::InterfaceMethodref { .. }
			| ConstantPoolValueInfo::InvokeDynamic { .. }
			| ConstantPoolValueInfo::Module { .. }
			| ConstantPoolValueInfo::Package { .. } => todo!(),
			ConstantPoolValueInfo::NameAndType { .. } => {
				unreachable!("NameAndType entries should not be resolved directly")
			},
			ConstantPoolValueInfo::Unusable => unreachable!(),
		}
	}

	pub(super) fn raw(&self) -> &classfile::ConstantPool {
		&self.raw
	}
}

impl Debug for ConstantPool {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.raw.iter()).finish()
	}
}
