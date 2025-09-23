pub mod types;
use types::{
	ConstantPoolEntryError, CpEntry, LoadableConstantPoolValue, LoadableConstantPoolValueInner,
};

use crate::error::ClassFileParseError;

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

	pub fn get<'a, T: CpEntry<'a>>(
		&'a self,
		index: u2,
	) -> Result<T::Entry, ConstantPoolEntryError> {
		T::get(self, index)
	}

	/// Same as [`Self::get()`], but will panic on errors
	///
	/// This is used in the runtime post-verification, when indices and types are known to be valid.
	pub fn expect<'a, T: CpEntry<'a>>(&'a self, index: u2) -> T::Entry {
		T::get(self, index).expect("")
	}

	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.4-310
	pub fn get_loadable_entry(
		&self,
		index: u2,
	) -> Result<LoadableConstantPoolValue<'_>, ConstantPoolEntryError> {
		if index > (self.len() as u2) {
			return Err(ConstantPoolEntryError::OutOfBounds(index));
		}

		let constant = &self[index];

		let value;
		match constant {
			ConstantPoolValueInfo::Integer { bytes } => {
				value = types::raw::Integer::handle(self, *bytes)
					.map(LoadableConstantPoolValueInner::Integer)?;
			},
			ConstantPoolValueInfo::Float { bytes } => {
				value = types::raw::Float::handle(self, *bytes)
					.map(LoadableConstantPoolValueInner::Float)?;
			},
			ConstantPoolValueInfo::Long {
				high_bytes,
				low_bytes,
			} => {
				value = types::raw::Long::handle(self, (*high_bytes, *low_bytes))
					.map(LoadableConstantPoolValueInner::Long)?;
			},
			ConstantPoolValueInfo::Double {
				high_bytes,
				low_bytes,
			} => {
				value = types::raw::Double::handle(self, (*high_bytes, *low_bytes))
					.map(LoadableConstantPoolValueInner::Double)?;
			},
			ConstantPoolValueInfo::Class { name_index } => {
				value = types::raw::RawClassName::handle(self, *name_index)
					.map(LoadableConstantPoolValueInner::Class)?;
			},
			ConstantPoolValueInfo::String { string_index } => {
				value = types::raw::RawString::handle(self, *string_index)
					.map(LoadableConstantPoolValueInner::String)?;
			},
			ConstantPoolValueInfo::MethodHandle {
				reference_kind,
				reference_index,
			} => {
				value =
					types::raw::RawMethodHandle::handle(self, (*reference_kind, *reference_index))
						.map(LoadableConstantPoolValueInner::MethodHandle)?;
			},
			ConstantPoolValueInfo::MethodType { descriptor_index } => {
				value = types::raw::RawMethodType::handle(self, *descriptor_index)
					.map(LoadableConstantPoolValueInner::MethodType)?;
			},
			ConstantPoolValueInfo::Dynamic {
				bootstrap_method_attr_index,
				name_and_type_index,
			} => {
				value = types::raw::RawDynamic::handle(
					self,
					(*bootstrap_method_attr_index, *name_and_type_index),
				)
				.map(LoadableConstantPoolValueInner::Dynamic)?;
			},
			c => {
				return Err(ConstantPoolEntryError::NotLoadable {
					index,
					tag: c.tag(),
				});
			},
		}

		Ok(LoadableConstantPoolValue { index, value })
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
	Dynamic            = 17,
    InvokeDynamic      = 18,
	Module             = 19,
	Package            = 20,
}

impl TryFrom<u1> for ConstantPoolTag {
	type Error = ClassFileParseError;

	fn try_from(value: u1) -> Result<Self, ClassFileParseError> {
		match value {
			1 => Ok(ConstantPoolTag::Utf8),
			3 => Ok(ConstantPoolTag::Integer),
			4 => Ok(ConstantPoolTag::Float),
			5 => Ok(ConstantPoolTag::Long),
			6 => Ok(ConstantPoolTag::Double),
			7 => Ok(ConstantPoolTag::Class),
			8 => Ok(ConstantPoolTag::String),
			9 => Ok(ConstantPoolTag::FieldRef),
			10 => Ok(ConstantPoolTag::MethodRef),
			11 => Ok(ConstantPoolTag::InterfaceMethodref),
			12 => Ok(ConstantPoolTag::NameAndType),
			15 => Ok(ConstantPoolTag::MethodHandle),
			16 => Ok(ConstantPoolTag::MethodType),
			18 => Ok(ConstantPoolTag::InvokeDynamic),
			19 => Ok(ConstantPoolTag::Module),
			20 => Ok(ConstantPoolTag::Package),
			other => Err(ClassFileParseError::BadConstantPoolTag(other)),
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

impl ConstantPoolValueInfo {
	pub fn tag(&self) -> ConstantPoolTag {
		match self {
			ConstantPoolValueInfo::Unusable => ConstantPoolTag::Unusable,
			ConstantPoolValueInfo::Class { .. } => ConstantPoolTag::Class,
			ConstantPoolValueInfo::Fieldref { .. } => ConstantPoolTag::FieldRef,
			ConstantPoolValueInfo::Methodref { .. } => ConstantPoolTag::MethodRef,
			ConstantPoolValueInfo::InterfaceMethodref { .. } => ConstantPoolTag::InterfaceMethodref,
			ConstantPoolValueInfo::String { .. } => ConstantPoolTag::String,
			ConstantPoolValueInfo::Integer { .. } => ConstantPoolTag::Integer,
			ConstantPoolValueInfo::Float { .. } => ConstantPoolTag::Float,
			ConstantPoolValueInfo::Long { .. } => ConstantPoolTag::Long,
			ConstantPoolValueInfo::Double { .. } => ConstantPoolTag::Double,
			ConstantPoolValueInfo::NameAndType { .. } => ConstantPoolTag::NameAndType,
			ConstantPoolValueInfo::Utf8 { .. } => ConstantPoolTag::Utf8,
			ConstantPoolValueInfo::MethodHandle { .. } => ConstantPoolTag::MethodHandle,
			ConstantPoolValueInfo::MethodType { .. } => ConstantPoolTag::MethodType,
			ConstantPoolValueInfo::Dynamic { .. } => ConstantPoolTag::Dynamic,
			ConstantPoolValueInfo::InvokeDynamic { .. } => ConstantPoolTag::InvokeDynamic,
			ConstantPoolValueInfo::Module { .. } => ConstantPoolTag::Module,
			ConstantPoolValueInfo::Package { .. } => ConstantPoolTag::Package,
		}
	}
}
