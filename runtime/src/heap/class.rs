use super::method::Method;
use super::reference::{ClassRef, FieldRef};
use crate::classpath::classloader::ClassLoader;
use super::field::Field;

use classfile::{ClassFile, ConstantPool, FieldType};
use common::traits::PtrType;
use common::types::u2;

pub struct Class {
	pub name: Vec<u8>,
	pub access_flags: u16,
	pub constant_pool: ConstantPool,
	pub super_class: Option<ClassRef>,
	pub methods: Vec<Method>,
    pub fields: Vec<Field>,
	pub loader: ClassLoader,
}

impl Class {
	pub fn new(parsed_file: ClassFile, super_class: Option<ClassRef>, loader: ClassLoader) -> Self {
		let access_flags = parsed_file.access_flags;

		let class_name_index = parsed_file.this_class;
		let name = parsed_file
			.constant_pool
			.get_class_name(class_name_index)
			.to_vec();

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
			loader,
		}
	}

    // https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-5.html#jvms-5.4.3.2
    pub fn resolve_field(&self, name_and_type_index: u2) -> Option<FieldRef> {
        let (name_index, descriptor_index) = self.constant_pool.get_name_and_type(name_and_type_index);

        let field_name = self.constant_pool.get_constant_utf8(name_index);
        let mut descriptor = self.constant_pool.get_constant_utf8(descriptor_index);

        let field_type = FieldType::parse(&mut descriptor);

        // When resolving a field reference, field resolution first attempts to look up
        // the referenced field in C and its superclasses:

        // 1. If C declares a field with the name and descriptor specified by the field reference,
        //    field lookup succeeds. The declared field is the result of the field lookup.
        for field in self.fields {
            if field.name == field_name && field.descriptor == field_type {
                return Some(FieldRef::new(field));
            }
        }

        // TODO:
        // 2. Otherwise, field lookup is applied recursively to the direct superinterfaces of the
        //    specified class or interface C.

        // 3. Otherwise, if C has a superclass S, field lookup is applied recursively to S.
        if let Some(super_class) = &self.super_class {
            let super_class = super_class.get();
            super_class.resolve_field(name_and_type_index);
        }

        // 4. Otherwise, field lookup fails.
        return None;
    }
}

// A pointer to a Class instance
//
// This can *not* be constructed by hand, as dropping it will
// deallocate the class.
pub struct ClassPtr(usize);

impl PtrType<Class, ClassRef> for ClassPtr {
	fn new(val: Class) -> ClassRef {
		let boxed = Box::new(val);
		let ptr = Box::into_raw(boxed);
		ClassRef::new(Self(ptr as usize))
	}

	#[inline(always)]
	fn as_raw(&self) -> *const Class {
		self.0 as *const Class
	}

	#[inline(always)]
	fn as_mut_raw(&self) -> *mut Class {
		self.0 as *mut Class
	}

    fn get(&self) -> &Class {
        unsafe { &(*self.as_raw()) }
    }
}

impl Drop for ClassPtr {
	fn drop(&mut self) {
		let _ = unsafe { Box::from_raw(self.0 as *mut Class) };
	}
}
