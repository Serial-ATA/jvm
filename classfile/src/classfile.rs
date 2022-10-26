use crate::attribute::Attribute;
use crate::constant_pool::ConstantPool;
use crate::fieldinfo::FieldInfo;
use crate::methodinfo::MethodInfo;

use common::types::{u1, u2};

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-4.html#jvms-4.1
#[derive(Debug, Clone, PartialEq)]
pub struct ClassFile {
	pub minor_version: u2,
	pub major_version: u2,
	pub constant_pool: ConstantPool,
	pub access_flags: u2,
	pub this_class: u2,
	pub super_class: u2,
	pub interfaces: Vec<u2>,
	pub fields: Vec<FieldInfo>,
	pub methods: Vec<MethodInfo>,
	pub attributes: Vec<Attribute>,
}

impl ClassFile {
	pub fn get_super_class(&self) -> Option<&[u1]> {
		// For a class, the value of the super_class item either must be zero or must be a valid
		// index into the constant_pool table.
		let super_class_index = self.super_class;

		let mut super_class_name = None;

		// If the value of the super_class item is zero, then this class file must represent
		// the class Object, the only class or interface without a direct superclass.
		if super_class_index != 0 {
			super_class_name = Some(self.constant_pool.get_class_name(super_class_index));
		}

		super_class_name
	}
}
