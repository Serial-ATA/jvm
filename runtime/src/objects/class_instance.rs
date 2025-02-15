use super::instance::{CloneableInstance, Header, Instance};
use crate::objects::class::Class;
use crate::objects::field::Field;
use crate::objects::monitor::Monitor;
use crate::objects::reference::{ClassInstanceRef, Reference};

use common::traits::PtrType;
use instructions::Operand;
use std::fmt::{Debug, Formatter};
use std::ptr::NonNull;
use std::sync::Arc;

#[derive(Debug)]
pub struct ClassInstance {
	header: Header,
	monitor: Arc<Monitor>,
	super_class: Option<ClassInstanceRef>,
	class: &'static Class,
	pub fields: Box<[Operand<Reference>]>,
}

impl ClassInstance {
	pub fn new(class: &'static Class) -> ClassInstanceRef {
		let instance_field_count = class.instance_field_count();

		let mut super_class = None;
		if let Some(ref super_class_) = class.super_class {
			super_class = Some(Self::new(super_class_));
		}

		// Set the default values for our non-static fields
		let mut fields = Vec::with_capacity(instance_field_count);
		for field in class.instance_fields() {
			fields.push(Field::default_value_for_ty(&field.descriptor))
		}

		// Sanity check
		assert_eq!(
			instance_field_count,
			fields.len(),
			"Created the wrong number of fields!"
		);

		ClassInstancePtr::new(Self {
			header: Header::new(),
			monitor: Arc::new(Monitor::new()),
			super_class,
			class,
			fields: fields.into_boxed_slice(),
		})
	}

	pub fn class(&self) -> &'static Class {
		self.class
	}

	pub fn is_subclass_of(&self, class: &Class) -> bool {
		self.class.is_subclass_of(class)
	}

	pub fn implements(&self, class: &Class) -> bool {
		self.class.implements(class)
	}
}

impl CloneableInstance for ClassInstance {
	type ReferenceTy = ClassInstanceRef;

	unsafe fn clone(&self) -> Self::ReferenceTy {
		let cloned_super;
		match &self.super_class {
			Some(super_class) => cloned_super = Some(unsafe { super_class.get().clone() }),
			None => cloned_super = None,
		}

		ClassInstancePtr::new(ClassInstance {
			header: Header::new(),
			monitor: Arc::new(Monitor::new()),
			super_class: cloned_super,
			class: self.class,
			fields: self.fields.clone(),
		})
	}
}

impl Instance for ClassInstance {
	fn header(&self) -> &Header {
		&self.header
	}

	fn monitor(&self) -> Arc<Monitor> {
		self.monitor.clone()
	}

	fn get_field_value(&self, field: &Field) -> Operand<Reference> {
		assert!(!field.is_static());
		self.get_field_value0(field.index())
	}

	fn get_field_value0(&self, field_idx: usize) -> Operand<Reference> {
		let mut count = 0;

		let mut current_class = &self.class;
		loop {
			count += current_class.instance_field_count();
			if count > field_idx {
				return self.fields[field_idx].clone();
			}

			if let Some(ref super_class) = current_class.super_class {
				current_class = super_class;
				continue;
			}

			break;
		}

		panic!(
			"Failed to resolve field index: {:?}, in class: {:?}",
			field_idx, self.class
		);
	}

	fn put_field_value(&mut self, field: &Field, value: Operand<Reference>) {
		assert!(!field.is_static());
		self.put_field_value0(field.index(), value)
	}

	fn put_field_value0(&mut self, field_idx: usize, value: Operand<Reference>) {
		let mut count = 0;

		let mut current_class = &self.class;
		loop {
			count += current_class.instance_field_count();
			if count > field_idx {
				let current = &self.fields[field_idx];
				assert!(
					current.is_compatible_with(&value),
					"Expected type compatible with: {current:?}, found: {value:?} (class: {}, \
					 field index: {field_idx})",
					self.class.name.as_str(),
				);

				self.fields[field_idx] = value;
				return;
			}

			if let Some(ref super_class) = current_class.super_class {
				current_class = super_class;
				continue;
			}

			break;
		}

		panic!(
			"Failed to resolve field index: {:?}, in class: {:?}",
			field_idx, self.class
		);
	}

	unsafe fn get_field_value_raw(&self, field_idx: usize) -> NonNull<Operand<Reference>> {
		assert!(field_idx < self.fields.len());
		NonNull::new_unchecked(self.fields.as_ptr().offset(field_idx as isize) as _)
	}
}

// A pointer to a ClassInstance
//
// This can *not* be constructed by hand, as dropping it will
// deallocate the instance.
#[derive(PartialEq)]
pub struct ClassInstancePtr(usize);

impl PtrType<ClassInstance, ClassInstanceRef> for ClassInstancePtr {
	fn new(val: ClassInstance) -> ClassInstanceRef {
		let boxed = Box::new(val);
		let ptr = Box::into_raw(boxed);
		ClassInstanceRef::new(Self(ptr as usize))
	}

	#[inline(always)]
	fn as_raw(&self) -> *const ClassInstance {
		self.0 as *const ClassInstance
	}

	#[inline(always)]
	fn as_mut_raw(&self) -> *mut ClassInstance {
		self.0 as *mut ClassInstance
	}

	fn get(&self) -> &ClassInstance {
		unsafe { &(*self.as_raw()) }
	}

	fn get_mut(&self) -> &mut ClassInstance {
		unsafe { &mut (*self.as_mut_raw()) }
	}
}

impl Drop for ClassInstancePtr {
	fn drop(&mut self) {
		let _ = unsafe { Box::from_raw(self.0 as *mut ClassInstance) };
	}
}

impl Debug for ClassInstancePtr {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let class = self.get();
		f.write_str(class.class.name.as_str())
	}
}
