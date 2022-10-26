use crate::heap::method::Method;
use crate::classpath::classloader::ClassLoader;

use classfile::{ClassFile, ConstantPool};
use common::traits::ReferenceType;

pub struct Class {
    pub name: Vec<u8>,
    pub access_flags: u16,
    pub constant_pool: ConstantPool,
    pub super_class: Option<ClassRef>,
    pub methods: Vec<Method>,
    pub loader: ClassLoader,
    // TODO
    // pub fields: Vec<Field>,
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
            loader
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

impl Drop for ClassRef {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(self.0 as *mut Class) };
    }
}