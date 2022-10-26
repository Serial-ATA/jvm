use std::fmt::{Debug, Formatter};
use std::ops::{Deref, Index};

use common::types::{u1, u2, u4};

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.4

#[derive(PartialEq, Clone)]
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

	pub fn get_class_name(&self, idx: u2) -> &[u1] {
		let constant = &self[idx];

		match constant {
			ConstantPoolValueInfo::Class { name_index } => self.get_constant_utf8(*name_index),
			_ => panic!("Expected a constant value of \"Class\""),
		}
	}

	pub fn get_constant_utf8(&self, idx: u2) -> &[u1] {
		let constant = &self[idx];

		match constant {
			ConstantPoolValueInfo::Utf8 { bytes, .. } => bytes,
			_ => panic!("Expected a constant value of \"Utf8\""),
		}
	}

	pub fn get_field_ref(&self, idx: u2) -> (u2, u2) {
		let constant = &self[idx];

		match constant {
			ConstantPoolValueInfo::Fieldref { class_index, name_and_type_index } => (*class_index, *name_and_type_index),
			_ => panic!("Expected a constant value of \"Fieldref\""),
		}
	}

	pub fn get_name_and_type(&self, idx: u2) -> (u2, u2) {
		let constant = &self[idx];

		match constant {
			ConstantPoolValueInfo::NameAndType { name_index, descriptor_index } => (*name_index, *descriptor_index),
			_ => panic!("Expected a constant value of \"NameAndType\""),
		}
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

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.4-140
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq)]
#[rustfmt::skip]
pub enum ConstantPoolTag {
    Class              = 7,
    FieldRef           = 9,
    MethodRef          = 10,
    InterfaceMethodref = 11,
    String             = 8,
    Integer            = 3,
    Float              = 4,
    Long               = 5,
    Double             = 6,
    NameAndType        = 12,
    Utf8               = 1,
    MethodHandle       = 15,
    MethodType         = 16,
    InvokeDynamic      = 18,
}

impl From<u8> for ConstantPoolTag {
	fn from(value: u8) -> Self {
		match value {
			7 => ConstantPoolTag::Class,
			9 => ConstantPoolTag::FieldRef,
			10 => ConstantPoolTag::MethodRef,
			11 => ConstantPoolTag::InterfaceMethodref,
			8 => ConstantPoolTag::String,
			3 => ConstantPoolTag::Integer,
			4 => ConstantPoolTag::Float,
			5 => ConstantPoolTag::Long,
			6 => ConstantPoolTag::Double,
			12 => ConstantPoolTag::NameAndType,
			1 => ConstantPoolTag::Utf8,
			15 => ConstantPoolTag::MethodHandle,
			16 => ConstantPoolTag::MethodType,
			18 => ConstantPoolTag::InvokeDynamic,
			_ => unreachable!(),
		}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstantPoolValueInfo {
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.4.1
	Class {
		name_index: u2,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.4.2
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
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.4.3
	String {
		string_index: u2,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.4.4
	Integer {
		bytes: u4,
	},
	Float {
		bytes: u4,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.4.5
	Long {
		high_bytes: u4,
		low_bytes: u4,
	},
	Double {
		high_bytes: u4,
		low_bytes: u4,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.4.6
	NameAndType {
		name_index: u2,
		descriptor_index: u2,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.4.7
	Utf8 {
		length: u2,
		bytes: Vec<u1>,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.4.8
	MethodHandle {
		reference_kind: u1,
		reference_index: u2,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.4.9
	MethodType {
		descriptor_index: u2,
	},
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.4.10
	InvokeDynamic {
		bootstrap_method_attr_index: u2,
		name_and_type_index: u2,
	},
}
