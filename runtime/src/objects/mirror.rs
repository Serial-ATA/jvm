use crate::class_instance::Instance;
use crate::field::Field;
use crate::objects::class::Class;
use crate::reference::{MirrorInstanceRef, Reference};

use std::fmt::{Debug, Formatter};
use std::ptr::NonNull;

use classfile::FieldType;
use common::traits::PtrType;
use instructions::Operand;

#[derive(Debug, Clone, PartialEq)]
enum MirrorTarget {
	Class(&'static Class),
	Primitive(FieldType),
}

// TODO: Make fields private
/// A mirror instance
///
/// A "mirror" is simply an instance of java.lang.Class with an associated [`ClassInstance`].
/// It contains information about the object it's describing, as well as wrapping up the object itself.
///
/// [`ClassInstance`]: super::class_instance::ClassInstance
#[derive(Debug, Clone, PartialEq)]
pub struct MirrorInstance {
	pub class: &'static Class,
	pub fields: Box<[Operand<Reference>]>,
	target: MirrorTarget,
}

impl MirrorInstance {
	pub fn new(mirror_class: &'static Class, target: &'static Class) -> MirrorInstanceRef {
		let fields = Self::initialize_fields(mirror_class);
		MirrorInstancePtr::new(Self {
			class: mirror_class,
			fields,
			target: MirrorTarget::Class(target),
		})
	}

	pub fn new_array(mirror_class: &'static Class, target: &'static Class) -> MirrorInstanceRef {
		let fields = Self::initialize_fields(mirror_class);
		MirrorInstancePtr::new(Self {
			class: mirror_class,
			fields,
			target: MirrorTarget::Class(target),
		})
	}

	pub fn new_primitive(mirror_class: &'static Class, target: FieldType) -> MirrorInstanceRef {
		assert!(
			!matches!(target, FieldType::Array(_) | FieldType::Object(_)),
			"`Array` and `Object` field types are incompatible with the primitive mirror"
		);

		let fields = Self::initialize_fields(mirror_class);
		MirrorInstancePtr::new(Self {
			class: mirror_class,
			fields,
			target: MirrorTarget::Primitive(target),
		})
	}

	pub fn has_target(&self, class: &'static Class) -> bool {
		match &self.target {
			MirrorTarget::Class(target) => *target == class,
			_ => false,
		}
	}

	pub fn is_primitive(&self) -> bool {
		matches!(&self.target, MirrorTarget::Primitive(_))
	}

	pub fn is_array(&self) -> bool {
		matches!(&self.target, MirrorTarget::Class(class) if class.is_array())
	}

	pub fn expect_class(&self) -> &'static Class {
		match &self.target {
			MirrorTarget::Class(class) => *class,
			_ => panic!("Expected mirror instance to point to class!"),
		}
	}

	pub fn expect_primitive(&self) -> FieldType {
		match &self.target {
			MirrorTarget::Primitive(primitive) => primitive.clone(),
			_ => panic!("Expected mirror instance to point to primitive!"),
		}
	}

	fn initialize_fields(mirror_class: &'static Class) -> Box<[Operand<Reference>]> {
		let instance_field_count = mirror_class.instance_field_count();

		// Set the default values for our non-static fields
		let mut fields = Vec::with_capacity(instance_field_count);
		for field in mirror_class.fields().filter(|field| !field.is_static()) {
			fields.push(Field::default_value_for_ty(&field.descriptor))
		}

		fields.into_boxed_slice()
	}
}

impl Instance for MirrorInstance {
	fn get_field_value(&self, field: &Field) -> Operand<Reference> {
		self.get_field_value0(field.idx)
	}

	fn get_field_value0(&self, field_idx: usize) -> Operand<Reference> {
		if field_idx >= self.fields.len() {
			panic!(
				"Failed to resolve field index: {:?}, in class: {:?}",
				field_idx, self.class
			);
		}

		self.fields[field_idx].clone()
	}

	fn put_field_value(&mut self, field: &Field, value: Operand<Reference>) {
		self.put_field_value0(field.idx, value)
	}

	fn put_field_value0(&mut self, field_idx: usize, value: Operand<Reference>) {
		if field_idx >= self.fields.len() {
			panic!(
				"Failed to resolve field index: {:?}, in class: {:?}",
				field_idx, self.class
			);
		}

		let current = &self.fields[field_idx];
		assert!(
			current.is_compatible_with(&value),
			"Expected type compatible with: {:?}, found: {:?}",
			current,
			value
		);

		self.fields[field_idx] = value;
	}

	unsafe fn get_field_value_raw(&self, field_idx: usize) -> NonNull<Operand<Reference>> {
		assert!(field_idx < self.fields.len());
		NonNull::new_unchecked(self.fields.as_ptr().offset(field_idx as isize) as _)
	}
}

// A pointer to a MirrorInstance
//
// This can *not* be constructed by hand, as dropping it will
// deallocate the instance.
#[derive(PartialEq)]
pub struct MirrorInstancePtr(usize);

impl PtrType<MirrorInstance, MirrorInstanceRef> for MirrorInstancePtr {
	fn new(val: MirrorInstance) -> MirrorInstanceRef {
		let boxed = Box::new(val);
		let ptr = Box::into_raw(boxed);
		MirrorInstanceRef::new(Self(ptr as usize))
	}

	#[inline(always)]
	fn as_raw(&self) -> *const MirrorInstance {
		self.0 as *const MirrorInstance
	}

	#[inline(always)]
	fn as_mut_raw(&self) -> *mut MirrorInstance {
		self.0 as *mut MirrorInstance
	}

	fn get(&self) -> &MirrorInstance {
		unsafe { &(*self.as_raw()) }
	}

	fn get_mut(&self) -> &mut MirrorInstance {
		unsafe { &mut (*self.as_mut_raw()) }
	}
}

impl Drop for MirrorInstancePtr {
	fn drop(&mut self) {
		let _ = unsafe { Box::from_raw(self.0 as *mut MirrorInstance) };
	}
}

impl Debug for MirrorInstancePtr {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let instance = self.get();
		write!(f, "{:?}", instance)
	}
}
