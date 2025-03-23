use crate::objects::array::Array;
use crate::objects::class::Class;
use crate::objects::instance::{CloneableInstance, Header};
use crate::objects::monitor::Monitor;
use crate::objects::reference::{ObjectArrayInstanceRef, Reference};
use crate::thread::exceptions::{throw, Throws};

use std::alloc::{alloc, Layout};
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use std::{iter, ptr, slice};

use common::int_types::s4;
use common::traits::PtrType;

/// An instance of an array of objects
#[derive(Debug)]
pub struct ObjectArrayInstance {
	header: Header,
	monitor: Arc<Monitor>,
	pub class: &'static Class,
	length: u32,
	base: *mut Reference,
}

impl ObjectArrayInstance {
	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.anewarray
	pub fn new(count: s4, component_class: &'static Class) -> Throws<ObjectArrayInstanceRef> {
		if count.is_negative() {
			throw!(@DEFER NegativeArraySizeException);
		}

		let array_class_name = component_class.array_class_name();
		let array_class = component_class.loader().load(array_class_name)?;

		let new_array = unsafe {
			Self::alloc_array(
				count as usize,
				iter::repeat_n(Reference::null(), count as usize),
			)
		};

		Throws::Ok(ObjectArrayInstancePtr::new(Self {
			header: Header::new(),
			monitor: Arc::new(Monitor::new()),
			class: array_class,
			length: count as u32,
			base: new_array,
		}))
	}

	// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-6.html#jvms-6.5.multianewarray
	pub fn new_multidimensional(
		counts: impl IntoIterator<Item = s4>,
		array_class: &'static Class,
	) -> Throws<ObjectArrayInstanceRef> {
		fn inner(
			parent: &mut ObjectArrayInstance,
			parent_count: s4,
			counts: &mut impl Iterator<Item = s4>,
			array_class: &'static Class,
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
				inner(instance.get_mut(), parent_count, counts, component_class)?;
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
			initial_instance.get_mut(),
			initial_count,
			&mut counts,
			component_class,
		)?;

		Throws::Ok(initial_instance)
	}

	pub fn as_slice(&self) -> &[Reference] {
		// SAFETY: The pointer and length are always valid. Arrays cannot grow or shrink.
		unsafe { slice::from_raw_parts(self.base, self.length as usize) }
	}

	pub fn as_mut_slice(&mut self) -> &mut [Reference] {
		// SAFETY: The pointer and length are always valid. Arrays cannot grow or shrink.
		unsafe { slice::from_raw_parts_mut(self.base, self.length as usize) }
	}

	/// Get a pointer to the value at `offset`
	///
	/// **NOTE**: The `offset` is a **BYTE OFFSET**! Do not attempt to use this as a normal getter.
	///           This should never be used outside of `jdk.internal.misc.Unsafe` natives.
	///
	/// # Safety
	///
	/// The caller must ensure that `offset` is within bounds.
	pub unsafe fn get_unchecked_raw(&mut self, offset: isize) -> *mut Reference {
		let start = unsafe { (self.base as *const u8).offset(offset) };
		start as *mut Reference
	}

	/// Replace the reference at `offset`
	///
	/// This will drop the value previously at `offset`.
	///
	/// **NOTE**: The `offset` is a **BYTE OFFSET**! Do not attempt to use this as a normal getter.
	///           This should never be used outside of `jdk.internal.misc.Unsafe` natives.
	///
	/// # Safety
	///
	/// The caller must ensure that `offset` is within bounds.
	pub unsafe fn store_unchecked_raw(&mut self, offset: isize, value: Reference) {
		let old = unsafe {
			let start = (self.base as *const u8).offset(offset);
			ptr::replace(start as *mut Reference, value)
		};
		drop(old);
	}

	unsafe fn alloc_array(
		count: usize,
		components: impl ExactSizeIterator<Item = Reference>,
	) -> *mut Reference {
		assert_eq!(count, components.len());

		let layout = Layout::array::<Reference>(count).expect("should always be valid");
		let array = unsafe { alloc(layout) } as *mut Reference;

		let mut ptr = array;
		for (_, r) in (0..count).into_iter().zip(components) {
			unsafe {
				ptr.write(r);
				ptr = ptr.add(1);
			}
		}

		array
	}

	// TODO: Make arrays implement `Instance`
	pub fn monitor(&self) -> Arc<Monitor> {
		self.monitor.clone()
	}
}

impl CloneableInstance for ObjectArrayInstance {
	type ReferenceTy = ObjectArrayInstanceRef;

	unsafe fn clone(&self) -> Self::ReferenceTy {
		let new_array =
			unsafe { Self::alloc_array(self.length as usize, self.as_slice().iter().cloned()) };

		ObjectArrayInstancePtr::new(Self {
			header: self.header.clone(),
			monitor: Arc::new(Monitor::new()),
			class: self.class,
			length: self.length,
			base: new_array,
		})
	}
}

impl super::Array for ObjectArrayInstance {
	type Component = Reference;

	const BASE_OFFSET: usize = 0;

	fn header(&self) -> &Header {
		&self.header
	}

	fn len(&self) -> usize {
		self.length as usize
	}

	unsafe fn get_unchecked(&self, index: usize) -> Self::Component {
		let value = unsafe { self.base.add(index).as_ref_unchecked() };
		value.clone()
	}

	unsafe fn store_unchecked(&mut self, index: usize, value: Self::Component) {
		let old = unsafe { ptr::replace(self.base.add(index), value) };
		drop(old);
	}

	unsafe fn copy_into(&self, src_pos: usize, dest: &mut Self, dest_pos: usize, length: usize) {
		unsafe {
			for i in 0..length {
				// `get_unchecked()` clones the value. `Reference`s cannot be copied.
				let value = self.get_unchecked(src_pos + i);

				// Using `store_unchecked()` ensures that whatever reference was at `dest_pos + i` gets dropped.
				dest.store_unchecked(dest_pos + i, value);
			}
		}
	}

	unsafe fn copy_within(&mut self, src_pos: usize, dest_pos: usize, length: usize) {
		unsafe {
			for i in 0..length {
				let current = self.get_unchecked(src_pos + i);
				self.store_unchecked(dest_pos + i, current);
			}
		}
	}
}

// A pointer to a ObjectArrayInstance
//
// This can *not* be constructed by hand, as dropping it will
// deallocate the instance.
#[derive(PartialEq)]
pub struct ObjectArrayInstancePtr(usize);

impl PtrType<ObjectArrayInstance, ObjectArrayInstanceRef> for ObjectArrayInstancePtr {
	fn new(val: ObjectArrayInstance) -> ObjectArrayInstanceRef {
		let boxed = Box::new(val);
		let ptr = Box::into_raw(boxed);
		ObjectArrayInstanceRef::new(Self(ptr as usize))
	}

	#[inline(always)]
	fn as_raw(&self) -> *const ObjectArrayInstance {
		self.0 as *const ObjectArrayInstance
	}

	#[inline(always)]
	fn as_mut_raw(&self) -> *mut ObjectArrayInstance {
		self.0 as *mut ObjectArrayInstance
	}

	fn get(&self) -> &ObjectArrayInstance {
		unsafe { &(*self.as_raw()) }
	}

	fn get_mut(&self) -> &mut ObjectArrayInstance {
		unsafe { &mut (*self.as_mut_raw()) }
	}
}

impl Drop for ObjectArrayInstancePtr {
	fn drop(&mut self) {
		let _ = unsafe { Box::from_raw(self.0 as *mut ObjectArrayInstance) };
	}
}

impl Debug for ObjectArrayInstancePtr {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let class = self.get();
		f.write_str(&class.class.name().as_str())
	}
}
