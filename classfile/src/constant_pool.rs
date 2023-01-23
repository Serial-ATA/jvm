use std::fmt::{Debug, Formatter};
use std::ops::{Deref, Index};
use std::sync::Arc;

use common::int_types::{s4, s8, u1, u2, u4};

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.4

pub type ConstantPoolRef = Arc<ConstantPool>;

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
			ConstantPoolValueInfo::Fieldref {
				class_index,
				name_and_type_index,
			} => (*class_index, *name_and_type_index),
			_ => panic!("Expected a constant value of \"Fieldref\""),
		}
	}

	pub fn get_method_ref(&self, idx: u2) -> (u2, u2) {
		let constant = &self[idx];

		match constant {
			ConstantPoolValueInfo::Methodref {
				class_index,
				name_and_type_index,
			} => (*class_index, *name_and_type_index),
			_ => panic!("Expected a constant value of \"Methodref\""),
		}
	}

	pub fn get_name_and_type(&self, idx: u2) -> (u2, u2) {
		let constant = &self[idx];

		match constant {
			ConstantPoolValueInfo::NameAndType {
				name_index,
				descriptor_index,
			} => (*name_index, *descriptor_index),
			_ => panic!("Expected a constant value of \"NameAndType\""),
		}
	}

	pub fn get_integer(&self, idx: u2) -> s4 {
		let constant = &self[idx];

		match constant {
			ConstantPoolValueInfo::Integer { bytes } => (*bytes) as s4,
			_ => panic!("Expected a constant value of \"Integer\""),
		}
	}

	pub fn get_float(&self, idx: u2) -> f32 {
		let constant = &self[idx];

		match constant {
			ConstantPoolValueInfo::Float { bytes } => (*bytes) as s4 as f32,
			_ => panic!("Expected a constant value of \"Float\""),
		}
	}

	pub fn get_long(&self, idx: u2) -> s8 {
		let constant = &self[idx];

		match constant {
			ConstantPoolValueInfo::Long {
				high_bytes,
				low_bytes,
			} => (s8::from(*high_bytes) << 32) + s8::from(*low_bytes),
			_ => panic!("Expected a constant value of \"Long\""),
		}
	}

	pub fn get_double(&self, idx: u2) -> f64 {
		let constant = &self[idx];

		match constant {
			ConstantPoolValueInfo::Double {
				high_bytes,
				low_bytes,
			} => {
				let high = high_bytes.to_be_bytes();
				let low = low_bytes.to_be_bytes();

				f64::from_be_bytes([
					high[0], high[1], high[2], high[3], low[0], low[1], low[2], low[3],
				])
			},
			_ => panic!("Expected a constant value of \"Double\""),
		}
	}

	pub fn get_string(&self, idx: u2) -> &[u1] {
		let constant = &self[idx];

		match constant {
			ConstantPoolValueInfo::String { string_index } => self.get_constant_utf8(*string_index),
			_ => panic!("Expected a constant value of \"String\""),
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
	Unusable, /* Used when storing longs/doubles (https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.4.5) */
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
	// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.4.11
	Module {
		name_index: u2,
	},
	// // https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.4.12
	Package {
		name_index: u2,
	},
}
