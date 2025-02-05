pub mod types;

use crate::constant_pool::types::{
	CpEntry, LoadableConstantPoolValue, LoadableConstantPoolValueInner,
};

use std::fmt::{Debug, Formatter};
use std::ops::{Deref, Index};

use common::int_types::{u1, u2, u4};

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.4

#[derive(PartialEq, Clone)]
#[repr(transparent)]
pub struct ConstantPool {
	inner: Vec<ConstantPoolValueInfo>,
}

impl ConstantPool {
	pub fn with_capacity(capacity: usize) -> Self {
		Self {
			inner: Vec::with_capacity(capacity),
		}
	}

	pub fn push(&mut self, value: ConstantPoolValueInfo) {
		self.inner.push(value);
	}

	pub fn into_inner(self) -> Vec<ConstantPoolValueInfo> {
		self.inner
	}

	pub fn get<'a, T: CpEntry<'a>>(&'a self, index: u2) -> T::Entry {
		T::get(self, index)
	}

	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.4-310
	pub fn get_loadable_entry(&self, index: u2) -> LoadableConstantPoolValue {
		let constant = &self[index];

		let value = match constant {
			ConstantPoolValueInfo::Integer { bytes } => {
				LoadableConstantPoolValueInner::Integer(types::raw::Integer::handle(self, *bytes))
			},
			ConstantPoolValueInfo::Float { bytes } => {
				LoadableConstantPoolValueInner::Float(types::raw::Float::handle(self, *bytes))
			},
			ConstantPoolValueInfo::Long {
				high_bytes,
				low_bytes,
			} => LoadableConstantPoolValueInner::Long(types::raw::Long::handle(
				self,
				(*high_bytes, *low_bytes),
			)),
			ConstantPoolValueInfo::Double {
				high_bytes,
				low_bytes,
			} => LoadableConstantPoolValueInner::Double(types::raw::Double::handle(
				self,
				(*high_bytes, *low_bytes),
			)),
			ConstantPoolValueInfo::Class { name_index } => LoadableConstantPoolValueInner::Class(
				types::raw::RawClassName::handle(self, *name_index),
			),
			ConstantPoolValueInfo::String { string_index } => {
				LoadableConstantPoolValueInner::String(types::raw::RawString::handle(
					self,
					*string_index,
				))
			},
			ConstantPoolValueInfo::MethodHandle {
				reference_kind,
				reference_index,
			} => LoadableConstantPoolValueInner::MethodHandle(types::raw::RawMethodHandle::handle(
				self,
				(*reference_kind, *reference_index),
			)),
			ConstantPoolValueInfo::MethodType { descriptor_index } => {
				LoadableConstantPoolValueInner::MethodType(types::raw::RawMethodType::handle(
					self,
					*descriptor_index,
				))
			},
			ConstantPoolValueInfo::Dynamic {
				bootstrap_method_attr_index,
				name_and_type_index,
			} => LoadableConstantPoolValueInner::Dynamic(types::raw::RawDynamic::handle(
				self,
				(*bootstrap_method_attr_index, *name_and_type_index),
			)),
			_ => panic!("Expected a loadable constant pool entry"),
		};

		LoadableConstantPoolValue { index, value }
	}
}

impl Index<u2> for ConstantPool {
	type Output = ConstantPoolValueInfo;

	fn index(&self, index: u2) -> &Self::Output {
		&self.inner[index as usize]
	}
}

impl Index<usize> for ConstantPool {
	type Output = ConstantPoolValueInfo;

	fn index(&self, index: usize) -> &Self::Output {
		&self.inner[index]
	}
}

impl Debug for ConstantPool {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.inner.iter()).finish()
	}
}

impl Deref for ConstantPool {
	type Target = [ConstantPoolValueInfo];

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.4-140
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
#[rustfmt::skip]
pub enum ConstantPoolTag {
	Unusable, /* Used when storing longs/doubles (https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.4.5) */
	Utf8               = 1,
	Integer            = 3,
	Float              = 4,
	Long               = 5,
	Double             = 6,
	Class              = 7,
	String             = 8,
	FieldRef           = 9,
	MethodRef          = 10,
	InterfaceMethodref = 11,
	NameAndType        = 12,
	MethodHandle       = 15,
	MethodType         = 16,
	InvokeDynamic      = 18,
	Module             = 19,
	Package            = 20,
}

impl From<u1> for ConstantPoolTag {
	fn from(value: u1) -> Self {
		match value {
			1 => ConstantPoolTag::Utf8,
			3 => ConstantPoolTag::Integer,
			4 => ConstantPoolTag::Float,
			5 => ConstantPoolTag::Long,
			6 => ConstantPoolTag::Double,
			7 => ConstantPoolTag::Class,
			8 => ConstantPoolTag::String,
			9 => ConstantPoolTag::FieldRef,
			10 => ConstantPoolTag::MethodRef,
			11 => ConstantPoolTag::InterfaceMethodref,
			12 => ConstantPoolTag::NameAndType,
			15 => ConstantPoolTag::MethodHandle,
			16 => ConstantPoolTag::MethodType,
			18 => ConstantPoolTag::InvokeDynamic,
			19 => ConstantPoolTag::Module,
			20 => ConstantPoolTag::Package,
			_ => unreachable!(),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstantPoolValueInfo {
	Unusable,
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.4.1
	Class {
		name_index: u2,
	},
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.4.2
	Fieldref {
		class_index: u2,
		name_and_type_index: u2,
	},
	Methodref {
		class_index: u2,
		name_and_type_index: u2,
	},
	InterfaceMethodref {
		class_index: u2,
		name_and_type_index: u2,
	},
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.4.3
	String {
		string_index: u2,
	},
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.4.4
	Integer {
		bytes: u4,
	},
	Float {
		bytes: u4,
	},
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.4.5
	Long {
		high_bytes: u4,
		low_bytes: u4,
	},
	Double {
		high_bytes: u4,
		low_bytes: u4,
	},
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.4.6
	NameAndType {
		name_index: u2,
		descriptor_index: u2,
	},
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.4.7
	Utf8 {
		length: u2,
		bytes: Box<[u1]>,
	},
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.4.8
	MethodHandle {
		reference_kind: u1,
		reference_index: u2,
	},
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.4.9
	MethodType {
		descriptor_index: u2,
	},
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.4.10
	Dynamic {
		bootstrap_method_attr_index: u2,
		name_and_type_index: u2,
	},
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.4.10
	InvokeDynamic {
		bootstrap_method_attr_index: u2,
		name_and_type_index: u2,
	},
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.4.11
	Module {
		name_index: u2,
	},
	// // https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.4.12
	Package {
		name_index: u2,
	},
}
