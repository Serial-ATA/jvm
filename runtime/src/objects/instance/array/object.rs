#[cfg(test)]
mod tests;

use crate::objects::class::ClassPtr;
use crate::objects::instance::array::Array;
use crate::objects::instance::object::Object;
use crate::objects::instance::{CloneableInstance, Header};
use crate::objects::reference::Reference;
use crate::thread::JavaThread;
use crate::thread::exceptions::{Throws, throw};

use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::slice;

use common::int_types::s4;
use jni::sys::jint;

/// Reference to an allocated array of objects
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct ObjectArrayInstanceRef(*mut ObjectArrayInstance);

impl ObjectArrayInstanceRef {
	pub fn as_slice(&self) -> &[Reference] {
		// SAFETY: The pointer and length are always valid. Arrays cannot grow or shrink.
		unsafe {
			slice::from_raw_parts(self.field_base() as *const Reference, self.length as usize)
		}
	}
}

impl Object for ObjectArrayInstanceRef {
	type Descriptor = ObjectArrayInstance;

	fn hash(&self, thread: &'static JavaThread) -> jint {
		self.header.generate_hash(thread)
	}

	fn class(&self) -> ClassPtr {
		unsafe { (&*self.0).class }
	}

	fn is_object_array(&self) -> bool {
		true
	}

	unsafe fn raw(&self) -> *mut () {
		self.0.cast()
	}

	unsafe fn field_base(&self) -> *mut u8 {
		let base = self.0.cast::<u8>();
		unsafe { base.add(Self::BASE_OFFSET) }
	}
}

impl PartialEq for ObjectArrayInstanceRef {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}
}

impl Deref for ObjectArrayInstanceRef {
	type Target = ObjectArrayInstance;

	fn deref(&self) -> &Self::Target {
		unsafe { &*self.0 }
	}
}

impl Debug for ObjectArrayInstanceRef {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		self.deref().fmt(f)
	}
}

/// An instance of an array of objects
#[derive(Debug)]
pub struct ObjectArrayInstance {
	header: Header,
	class: ClassPtr,
	length: u32,
}

impl ObjectArrayInstance {
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.anewarray
	pub fn new(count: s4, component_class: ClassPtr) -> Throws<ObjectArrayInstanceRef> {
		if count.is_negative() {
			throw!(@DEFER NegativeArraySizeException);
		}

		let array_class_name = component_class.array_class_name();
		let array_class = component_class.loader().load(array_class_name)?;

		let header = ObjectArrayInstance {
			header: Header::new(),
			class: array_class,
			length: count as u32,
		};
		let element_size =
			count as usize * size_of::<<ObjectArrayInstanceRef as Array>::Component>();

		let new_array = unsafe { ObjectArrayInstanceRef::allocate(header, element_size) };

		Throws::Ok(ObjectArrayInstanceRef(new_array))
	}

	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.multianewarray
	pub fn new_multidimensional(
		counts: impl IntoIterator<Item = s4>,
		array_class: ClassPtr,
	) -> Throws<ObjectArrayInstanceRef> {
		fn inner(
			parent: ObjectArrayInstanceRef,
			parent_count: s4,
			counts: &mut impl Iterator<Item = s4>,
			array_class: ClassPtr,
		) -> Throws<()> {
			let Some(count) = counts.next() else {
				return Throws::Ok(());
			};

			let component_name = array_class.array_component_name();
			let component_class = array_class
				.loader()
				.load(component_name)
				.expect("component classes must exist");
			for i in 0..parent_count {
				let instance = ObjectArrayInstance::new(count, array_class)?;
				inner(instance, parent_count, counts, component_class)?;
				parent.store(i, Reference::object_array(instance))?;
			}

			Throws::Ok(())
		}

		assert!(
			array_class.is_array(),
			"multi-dimensional arrays must have array component types"
		);

		let mut counts = counts.into_iter();

		let initial_count = counts
			.next()
			.expect("multi-dimensional arrays must have at least one element");
		let initial_instance = Self::new(initial_count, array_class)?;

		let component_name = array_class.array_component_name();
		let component_class = array_class
			.loader()
			.load(component_name)
			.expect("component classes must exist");
		inner(
			initial_instance,
			initial_count,
			&mut counts,
			component_class,
		)?;

		Throws::Ok(initial_instance)
	}
}

impl CloneableInstance for ObjectArrayInstanceRef {
	type ReferenceTy = ObjectArrayInstanceRef;

	unsafe fn clone(&self) -> Self::ReferenceTy {
		let cloned_instance = ObjectArrayInstance::new(self.length as s4, self.class).expect("oom");

		// SAFETY: References are `Copy`
		unsafe {
			self.field_base()
				.cast::<Reference>()
				.copy_to_nonoverlapping(
					cloned_instance.field_base().cast::<Reference>(),
					self.length as usize,
				);
		}

		cloned_instance
	}
}

impl Array for ObjectArrayInstanceRef {
	type Component = Reference;

	const BASE_OFFSET: usize = size_of::<ObjectArrayInstance>();

	fn len(&self) -> usize {
		self.length as usize
	}

	fn align(&self) -> usize {
		align_of::<Self::Component>()
	}

	fn scale(&self) -> usize {
		size_of::<Self::Component>()
	}

	unsafe fn get_unchecked(&self, index: usize) -> Self::Component {
		// Object::get() operates with byte offsets
		let offset = index * self.scale();
		unsafe { Object::get::<Self::Component>(self, offset) }
	}

	unsafe fn store_unchecked(&self, index: usize, value: Self::Component) {
		// Object::put() operates with byte offsets
		let offset = index * self.scale();
		unsafe {
			Object::put::<Self::Component>(self, value, offset);
		}
	}

	unsafe fn copy_into(&self, src_pos: usize, dest: &Self, dest_pos: usize, length: usize) {
		unsafe {
			for i in 0..length {
				// `get_unchecked()` clones the value. `Reference`s cannot be copied.
				let value = self.get_unchecked(src_pos + i);

				// Using `store_unchecked()` ensures that whatever reference was at `dest_pos + i` gets dropped.
				dest.store_unchecked(dest_pos + i, value);
			}
		}
	}

	unsafe fn copy_within(&self, src_pos: usize, dest_pos: usize, length: usize) {
		unsafe {
			for i in 0..length {
				let current = self.get_unchecked(src_pos + i);
				self.store_unchecked(dest_pos + i, current);
			}
		}
	}
}
