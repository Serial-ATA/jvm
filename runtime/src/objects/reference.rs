//! https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.4

use super::class::ClassPtr;
use super::instance::Instance;
use crate::objects::field::Field;
use crate::objects::instance::array::{Array, ObjectArrayInstanceRef, PrimitiveArrayInstanceRef};
use crate::objects::instance::class::ClassInstanceRef;
use crate::objects::instance::mirror::MirrorInstanceRef;
use crate::objects::instance::object::Object;
use crate::thread::JavaThread;
use crate::thread::exceptions::{Throws, throw};

use std::fmt::Debug;

use ::jni::sys::jint;
use instructions::Operand;

/// A reference to an object
///
/// This implements [`Instance`], and calls to the trait's methods will panic at runtime if the
/// reference isn't an appropriate type.
///
/// It is important to note that calling [`clone()`](Clone::clone) on this will **not** clone the
/// object. It will simply clone the *reference*.
///
/// To clone objects, see the [`CloneableInstance::clone`] impl on each respective reference type.
///
/// [`CloneableInstance::clone`]: super::instance::CloneableInstance::clone
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Reference(*mut ());

const _: () = {
	assert!(
		size_of::<Reference>() == size_of::<usize>(),
		"Reference must be pointer-sized"
	);
};

// SAFETY: Synchronization handled manually
unsafe impl Send for Reference {}
unsafe impl Sync for Reference {}

impl Debug for Reference {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		unsafe {
			match self.ty() {
				Some(ReferenceType::Class) => self.as_class_unchecked().fmt(f),
				Some(ReferenceType::Mirror) => self.as_mirror_unchecked().fmt(f),
				Some(ReferenceType::PrimitiveArray) => self.as_primitive_array_unchecked().fmt(f),
				Some(ReferenceType::ObjectArray) => self.as_object_array_unchecked().fmt(f),
				None => f.write_str("Null"),
			}
		}
	}
}

impl PartialEq for Reference {
	fn eq(&self, other: &Self) -> bool {
		if self.is_null() && other.is_null() {
			return true;
		}

		self.0 == other.0
	}
}

/// The object type that a [`Reference`] targets
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ReferenceType {
	/// A class instance ([`ClassInstanceRef`])
	Class = 0b00,
	/// A mirror instance ([`MirrorInstanceRef`])
	Mirror = 0b01,
	/// A primitive array instance ([`PrimitiveArrayInstanceRef`])
	PrimitiveArray = 0b10,
	/// An object array instance ([`ObjectArrayInstanceRef`])
	ObjectArray = 0b11,
}

impl Reference {
	const TAG_MASK: usize = 0b11;
	const ADDRESS_MASK: usize = !Self::TAG_MASK;

	#[inline]
	pub fn null() -> Self {
		Self(std::ptr::null_mut())
	}

	#[inline]
	pub fn is_null(self) -> bool {
		self.addr().is_null()
	}

	#[inline]
	pub fn class(instance: ClassInstanceRef) -> Self {
		let raw = unsafe { instance.raw() };
		Self((raw as usize | ReferenceType::Class as usize) as *mut ())
	}

	#[inline]
	unsafe fn as_class_unchecked(self) -> ClassInstanceRef {
		unsafe { std::mem::transmute::<_, ClassInstanceRef>(self.addr()) }
	}

	#[inline]
	pub fn mirror(instance: MirrorInstanceRef) -> Self {
		let raw = unsafe { instance.raw() };
		Self((raw as usize | ReferenceType::Mirror as usize) as *mut ())
	}

	#[inline]
	unsafe fn as_mirror_unchecked(self) -> MirrorInstanceRef {
		unsafe { std::mem::transmute::<_, MirrorInstanceRef>(self.addr()) }
	}

	#[inline]
	pub fn array(instance: PrimitiveArrayInstanceRef) -> Self {
		let raw = unsafe { instance.raw() };
		Self((raw as usize | ReferenceType::PrimitiveArray as usize) as *mut ())
	}

	#[inline]
	unsafe fn as_primitive_array_unchecked(self) -> PrimitiveArrayInstanceRef {
		unsafe { std::mem::transmute::<_, PrimitiveArrayInstanceRef>(self.addr()) }
	}

	#[inline]
	pub fn object_array(instance: ObjectArrayInstanceRef) -> Self {
		let raw = unsafe { instance.raw() };
		Self((raw as usize | ReferenceType::ObjectArray as usize) as *mut ())
	}

	#[inline]
	unsafe fn as_object_array_unchecked(self) -> ObjectArrayInstanceRef {
		unsafe { std::mem::transmute::<_, ObjectArrayInstanceRef>(self.addr()) }
	}

	/// The type of object this references
	///
	/// For `null` references, this returns `None`
	pub fn ty(self) -> Option<ReferenceType> {
		if self.is_null() {
			return None;
		}

		match self.0 as usize & Self::TAG_MASK {
			0b00 => Some(ReferenceType::Class),
			0b01 => Some(ReferenceType::Mirror),
			0b10 => Some(ReferenceType::PrimitiveArray),
			0b11 => Some(ReferenceType::ObjectArray),
			_ => unreachable!(),
		}
	}

	fn addr(self) -> *mut () {
		(self.0 as usize & Self::ADDRESS_MASK) as *mut ()
	}

	/// The pointer backing this reference *including* the type tag
	pub fn raw_tagged(self) -> *const () {
		self.0.cast_const()
	}

	/// Construct a `Reference` from a raw **tagged** pointer
	///
	/// # Safety
	///
	/// The caller **must** ensure that the pointer is both tagged and pointing to a valid object.
	pub unsafe fn from_raw(ptr: *mut ()) -> Self {
		Self(ptr)
	}
}

impl Object for Reference {
	type Descriptor = ();

	fn hash(&self, thread: &'static JavaThread) -> jint {
		match self.ty() {
			Some(ReferenceType::Class) => unsafe { self.as_class_unchecked() }.hash(thread),
			Some(ReferenceType::Mirror) => unsafe { self.as_mirror_unchecked() }.hash(thread),
			Some(ReferenceType::PrimitiveArray) => {
				unsafe { self.as_primitive_array_unchecked() }.hash(thread)
			},
			Some(ReferenceType::ObjectArray) => {
				unsafe { self.as_object_array_unchecked() }.hash(thread)
			},
			// Null references are always 0
			None => 0,
		}
	}

	fn class(&self) -> ClassPtr {
		match self.ty() {
			Some(ReferenceType::Class) => unsafe { self.as_class_unchecked() }.class(),
			Some(ReferenceType::Mirror) => unsafe { self.as_mirror_unchecked() }.class(),
			Some(ReferenceType::PrimitiveArray) => {
				unsafe { self.as_primitive_array_unchecked() }.class()
			},
			Some(ReferenceType::ObjectArray) => unsafe { self.as_object_array_unchecked() }.class(),
			None => panic!("NullPointerException"),
		}
	}

	#[inline]
	fn is_object_array(&self) -> bool {
		self.ty() == Some(ReferenceType::ObjectArray)
	}

	#[inline]
	fn is_primitive_array(&self) -> bool {
		self.ty() == Some(ReferenceType::PrimitiveArray)
	}

	#[inline]
	fn is_class(&self) -> bool {
		self.ty() == Some(ReferenceType::Class)
	}

	#[inline]
	fn is_mirror(&self) -> bool {
		self.ty() == Some(ReferenceType::Mirror)
	}

	fn field_allocation_size(&self) -> usize {
		match self.ty() {
			Some(ReferenceType::Class) => {
				unsafe { self.as_class_unchecked() }.field_allocation_size()
			},
			Some(ReferenceType::Mirror) => {
				unsafe { self.as_mirror_unchecked() }.field_allocation_size()
			},
			Some(ReferenceType::PrimitiveArray) => {
				unsafe { self.as_primitive_array_unchecked() }.field_allocation_size()
			},
			Some(ReferenceType::ObjectArray) => {
				unsafe { self.as_object_array_unchecked() }.field_allocation_size()
			},
			None => panic!("NullPointerException"),
		}
	}

	unsafe fn raw(&self) -> *mut () {
		panic!(
			"The raw address of `References` can't be fetched. See `Reference::raw_tagged()` \
			 instead."
		)
	}

	unsafe fn field_base(&self) -> *mut u8 {
		unsafe {
			match self.ty() {
				Some(ReferenceType::Class) => self.as_class_unchecked().field_base(),
				Some(ReferenceType::Mirror) => self.as_mirror_unchecked().field_base(),
				Some(ReferenceType::PrimitiveArray) => {
					self.as_primitive_array_unchecked().field_base()
				},
				Some(ReferenceType::ObjectArray) => self.as_object_array_unchecked().field_base(),
				_ => std::ptr::null_mut(),
			}
		}
	}

	unsafe fn put<T: Copy>(&self, value: T, offset: usize) {
		unsafe {
			match self.ty() {
				Some(ReferenceType::Class) => self.as_class_unchecked().put(value, offset),
				Some(ReferenceType::Mirror) => self.as_mirror_unchecked().put(value, offset),
				Some(ReferenceType::PrimitiveArray) => {
					self.as_primitive_array_unchecked().put(value, offset)
				},
				Some(ReferenceType::ObjectArray) => {
					self.as_object_array_unchecked().put(value, offset)
				},
				None => panic!("NullPointerException"),
			}
		}
	}

	unsafe fn get<T: Copy>(&self, offset: usize) -> T {
		unsafe {
			match self.ty() {
				Some(ReferenceType::Class) => self.as_class_unchecked().get(offset),
				Some(ReferenceType::Mirror) => self.as_mirror_unchecked().get(offset),
				Some(ReferenceType::PrimitiveArray) => {
					Object::get(&self.as_primitive_array_unchecked(), offset)
				},
				Some(ReferenceType::ObjectArray) => {
					Object::get(&self.as_object_array_unchecked(), offset)
				},
				None => panic!("NullPointerException"),
			}
		}
	}

	unsafe fn get_raw<T: Copy>(&self, offset: usize) -> *mut T {
		unsafe {
			match self.ty() {
				Some(ReferenceType::Class) => self.as_class_unchecked().get_raw(offset),
				Some(ReferenceType::Mirror) => self.as_mirror_unchecked().get_raw(offset),
				Some(ReferenceType::PrimitiveArray) => {
					self.as_primitive_array_unchecked().get_raw(offset)
				},
				Some(ReferenceType::ObjectArray) => {
					self.as_object_array_unchecked().get_raw(offset)
				},
				None => std::ptr::null_mut(),
			}
		}
	}
}

impl Reference {
	pub fn is_instance_of(&self, other: ClassPtr) -> bool {
		self.extract_instance_class().can_cast_to(other)
	}

	pub fn array_length(&self) -> Throws<usize> {
		if self.is_null() {
			throw!(@DEFER NullPointerException);
		}

		if self.is_primitive_array() {
			let array = unsafe { self.as_primitive_array_unchecked() };
			return Throws::Ok(array.len());
		}

		if self.is_object_array() {
			let array = unsafe { self.as_object_array_unchecked() };
			return Throws::Ok(array.len());
		}

		panic!("Expected an array reference!")
	}

	pub fn extract_primitive_array(&self) -> PrimitiveArrayInstanceRef {
		assert!(!self.is_null());

		if self.is_primitive_array() {
			return unsafe { self.as_primitive_array_unchecked() };
		}

		panic!("Expected a primitive array reference!")
	}

	pub fn extract_object_array(&self) -> ObjectArrayInstanceRef {
		assert!(!self.is_null());

		if self.is_object_array() {
			return unsafe { self.as_object_array_unchecked() };
		}

		panic!("Expected an object array reference!")
	}

	pub fn extract_class(&self) -> ClassInstanceRef {
		assert!(!self.is_null());

		if self.is_class() {
			return unsafe { self.as_class_unchecked() };
		}

		panic!("Expected a class reference!")
	}

	/// Get the class that this reference targets
	///
	/// This has a subtle difference from [`Reference::extract_instance_class`] in the case of `mirror` instances.
	/// This will return the class that `mirror` instance is targeting, while `extract_instance_class` will return
	/// `java.lang.Class`.
	///
	/// This is a very important distinction to make when dealing with things such as method resolution.
	///
	/// See also:
	/// * [`Reference::extract_instance_class`]
	/// * [`MirrorInstance::target_class`]
	///
	/// For references other than `mirror`, this will return the same as `extract_instance_class`.
	///
	/// [`MirrorInstance::target_class`]: crate::objects::instance::mirror::MirrorInstance::target_class
	pub fn extract_target_class(&self) -> ClassPtr {
		if self.is_mirror() {
			let mirror = unsafe { self.as_mirror_unchecked() };
			return mirror.target_class();
		}

		self.class()
	}

	/// Get the class of the instance
	///
	/// This has a subtle difference from [`Reference::extract_target_class`] in the case of `mirror` instances.
	/// This will return `java.lang.Class` for `mirror` instances, while `extract_target_class` will return the class
	/// the mirror is targeting.
	///
	/// This is a very important distinction to make when dealing with things such as method resolution.
	///
	/// For references other than `mirror`, this will return the same as `extract_target_class`.
	pub fn extract_instance_class(&self) -> ClassPtr {
		self.class()
	}

	pub fn extract_mirror(&self) -> MirrorInstanceRef {
		assert!(!self.is_null());

		if self.is_mirror() {
			return unsafe { self.as_mirror_unchecked() };
		}

		panic!("Expected a mirror reference!")
	}

	/// Extract a mirror instance from a `Class` or `Array` instance, this is NOT the same as `Reference::extract_mirror`
	pub fn extract_class_mirror(&self) -> MirrorInstanceRef {
		assert!(!self.is_null());
		assert!(!self.is_mirror(), "Expected a class/array reference!");

		self.class().mirror()
	}
}

impl Instance for Reference {
	fn get_field_value(&self, field: &Field) -> Operand<Reference> {
		match self.ty() {
			Some(ReferenceType::Class) => {
				unsafe { self.as_class_unchecked() }.get_field_value(field)
			},
			Some(ReferenceType::Mirror) => {
				unsafe { self.as_mirror_unchecked() }.get_field_value(field)
			},
			_ => panic!("Expected a class/mirror reference!"),
		}
	}

	fn get_field_value0(&self, field_idx: usize) -> Operand<Reference> {
		match self.ty() {
			Some(ReferenceType::Class) => {
				unsafe { self.as_class_unchecked() }.get_field_value0(field_idx)
			},
			Some(ReferenceType::Mirror) => {
				unsafe { self.as_mirror_unchecked() }.get_field_value0(field_idx)
			},
			_ => panic!("Expected a class/mirror reference!"),
		}
	}

	fn put_field_value(&self, field: &Field, value: Operand<Reference>) {
		match self.ty() {
			Some(ReferenceType::Class) => {
				unsafe { self.as_class_unchecked() }.put_field_value(field, value)
			},
			Some(ReferenceType::Mirror) => {
				unsafe { self.as_mirror_unchecked() }.put_field_value(field, value)
			},
			_ => panic!("Expected a class/mirror reference!"),
		}
	}

	fn put_field_value0(&self, field_idx: usize, value: Operand<Reference>) {
		match self.ty() {
			Some(ReferenceType::Class) => {
				unsafe { self.as_class_unchecked() }.put_field_value0(field_idx, value)
			},
			Some(ReferenceType::Mirror) => {
				unsafe { self.as_mirror_unchecked() }.put_field_value0(field_idx, value)
			},
			_ => panic!("Expected a class/mirror reference!"),
		}
	}
}
