use super::ReferenceType;
use crate::method::Method;

use classfile::{ClassFile, ConstantPool};

pub struct Class {
	pub name: Vec<u8>,
	pub access_flags: u16,
	pub constant_pool: ConstantPool,
	pub super_class: Option<ClassRef>,
	pub methods: Vec<Method>,
	// TODO
	// pub fields: Vec<Field>,
	// pub loader: ClassLoader
}

impl Class {
	pub fn new(parsed_file: ClassFile) -> Self {
		let access_flags = parsed_file.access_flags;

		let class_name_index = parsed_file.this_class;
		let name = parsed_file
			.constant_pool
			.get_class_name(class_name_index)
			.to_vec();

		let super_class_index = parsed_file.super_class;

		let mut super_class = None;
		if super_class_index != 0 {
			// TODO
		}

		let constant_pool = parsed_file.constant_pool;

		let methods = parsed_file
			.methods
			.iter()
			.map(|mi| Method::new(mi, &constant_pool))
			.collect();

		Self {
			name,
			access_flags,
			constant_pool,
			super_class,
			methods,
		}
	}
}

pub struct ClassRef(usize);

impl ReferenceType<Class> for ClassRef {
	fn new(val: Class) -> Self {
		let boxed = Box::new(val);
		let ptr = Box::into_raw(boxed);
		Self(ptr as usize)
	}

	#[inline(always)]
	fn as_raw(&self) -> *const Class {
		self.0 as *const Class
	}

	#[inline(always)]
	fn as_mut_raw(&self) -> *mut Class {
		self.0 as *mut Class
	}
}
