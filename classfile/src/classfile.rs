use crate::accessflags::ClassAccessFlags;
use crate::attribute::{Attribute, SourceFile};
use crate::constant_pool::ConstantPool;
use crate::fieldinfo::FieldInfo;
use crate::methodinfo::MethodInfo;
use crate::parse::error::Result;
use crate::AttributeType;

use std::io::Read;

use common::int_types::{u1, u2};

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-4.html#jvms-4.1
#[derive(Debug, Clone, PartialEq)]
pub struct ClassFile {
	pub minor_version: u2,
	pub major_version: u2,
	pub constant_pool: ConstantPool,
	pub access_flags: ClassAccessFlags,
	pub this_class: u2,
	pub super_class: u2,
	pub interfaces: Vec<u2>,
	pub fields: Vec<FieldInfo>,
	pub methods: Vec<MethodInfo>,
	pub attributes: Vec<Attribute>,
}

impl ClassFile {
	pub fn read_from<R: Read>(reader: &mut R) -> Result<Self> {
		crate::parse::parse_class(reader)
	}

	pub fn get_super_class(&self) -> Option<&[u1]> {
		// For a class, the value of the super_class item either must be zero or must be a valid
		// index into the constant_pool table.
		let super_class_index = self.super_class;

		let mut super_class_name = None;

		// If the value of the super_class item is zero, then this class file must represent
		// the class Object, the only class or interface without a direct superclass.
		if super_class_index == 0 {
			assert_eq!(
				self.constant_pool.get_class_name(self.this_class),
				b"java/lang/Object",
				"Only java/lang/Object can have no superclass!"
			);
		} else {
			super_class_name = Some(self.constant_pool.get_class_name(super_class_index));
		}

		super_class_name
	}

	pub fn source_file_index(&self) -> Option<u2> {
		for attr in &self.attributes {
			if let AttributeType::SourceFile(SourceFile { sourcefile_index }) = attr.info {
				return Some(sourcefile_index);
			}
		}

		None
	}
}
