use super::array::{ObjectArrayInstancePtr, PrimitiveArrayInstancePtr};
use super::class::Class;
use super::class_instance::ClassInstancePtr;
use super::field::Field;
use super::instance::{Header, Instance};
use super::mirror::MirrorInstancePtr;
use super::monitor::Monitor;
use crate::objects::array::Array;
use crate::symbols::Symbol;
use crate::thread::JavaThread;
use crate::thread::exceptions::{Throws, throw};

use std::ptr::NonNull;
use std::sync::Arc;

use ::jni::sys::jint;
use common::traits::PtrType;
use instructions::Operand;

pub type ClassInstanceRef = Arc<ClassInstancePtr>;
pub type PrimitiveArrayInstanceRef = Arc<PrimitiveArrayInstancePtr>;
pub type ObjectArrayInstanceRef = Arc<ObjectArrayInstancePtr>;
pub type MirrorInstanceRef = Arc<MirrorInstancePtr>;

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.4

/// A reference to an object
///
/// It is important to note that calling [`clone()`](Clone::clone) on this will **not** clone the
/// object. It will simply clone the *reference*, as well as a reference to the [`Monitor`].
///
/// To clone objects, see the [`CloneableInstance::clone`] impl on each respective instance type.
///
/// [`CloneableInstance::clone`]: super::instance::CloneableInstance::clone
#[derive(Debug, Clone)]
#[repr(C)]
pub struct Reference {
	instance: ReferenceInstance,
}

const _: () = {
	let reference_size = size_of::<Reference>();
	assert!(
		reference_size & (reference_size - 1) == 0,
		"size_of(`Reference`) is not a power of two"
	);
};

impl PartialEq for Reference {
	fn eq(&self, other: &Self) -> bool {
		self.instance == other.instance
	}
}

#[derive(Debug, Clone)]
#[repr(C)]
enum ReferenceInstance {
	Class(ClassInstanceRef),
	Array(PrimitiveArrayInstanceRef),
	ObjectArray(ObjectArrayInstanceRef),
	Mirror(MirrorInstanceRef),
	Null,
}

impl PartialEq for ReferenceInstance {
	fn eq(&self, other: &Self) -> bool {
		match (self, other) {
			(ReferenceInstance::Class(val), ReferenceInstance::Class(other)) => {
				val.as_raw() == other.as_raw()
			},
			(ReferenceInstance::Array(val), ReferenceInstance::Array(other)) => {
				val.as_raw() == other.as_raw()
			},
			(ReferenceInstance::ObjectArray(val), ReferenceInstance::ObjectArray(other)) => {
				val.as_raw() == other.as_raw()
			},
			(ReferenceInstance::Mirror(val), ReferenceInstance::Mirror(other)) => {
				val.as_raw() == other.as_raw()
			},
			// All null references are equal
			(ReferenceInstance::Null, ReferenceInstance::Null) => true,
			_ => false,
		}
	}
}

impl ReferenceInstance {
	fn raw(&self) -> *const () {
		match self {
			ReferenceInstance::Class(val) => val.as_raw() as _,
			ReferenceInstance::Array(val) => val.as_raw() as _,
			ReferenceInstance::ObjectArray(val) => val.as_raw() as _,
			ReferenceInstance::Mirror(val) => val.as_raw() as _,
			ReferenceInstance::Null => core::ptr::null(),
		}
	}
}

impl Reference {
	pub fn class(instance: ClassInstanceRef) -> Reference {
		Self {
			instance: ReferenceInstance::Class(instance),
		}
	}

	pub fn array(instance: PrimitiveArrayInstanceRef) -> Reference {
		Self {
			instance: ReferenceInstance::Array(instance),
		}
	}

	pub fn object_array(instance: ObjectArrayInstanceRef) -> Reference {
		Self {
			instance: ReferenceInstance::ObjectArray(instance),
		}
	}

	pub fn mirror(instance: MirrorInstanceRef) -> Reference {
		Self {
			instance: ReferenceInstance::Mirror(instance),
		}
	}

	pub fn null() -> Reference {
		Self {
			instance: ReferenceInstance::Null,
		}
	}
}

impl Reference {
	pub fn is_class(&self) -> bool {
		matches!(self.instance, ReferenceInstance::Class(_))
	}

	pub fn is_primitive_array(&self) -> bool {
		matches!(self.instance, ReferenceInstance::Array(_))
	}

	pub fn is_object_array(&self) -> bool {
		matches!(self.instance, ReferenceInstance::ObjectArray(_))
	}

	pub fn is_mirror(&self) -> bool {
		matches!(self.instance, ReferenceInstance::Mirror(_))
	}

	pub fn is_null(&self) -> bool {
		matches!(self.instance, ReferenceInstance::Null)
	}
}

impl Reference {
	pub fn hash(&self) -> Option<jint> {
		// Null references are always 0
		if self.is_null() {
			return Some(0);
		}

		self.header().hash()
	}

	/// Generate a new hash for this object
	///
	/// In the event that another thread is already generating a hash, this thread will spin until it finishes.
	pub fn generate_hash(&self, thread: &'static JavaThread) -> jint {
		// Null references are always 0
		if self.is_null() {
			return 0;
		}

		self.header().generate_hash(thread)
	}
}

impl Reference {
	pub fn is_instance_of(&self, other: &'static Class) -> bool {
		self.extract_instance_class().can_cast_to(other)
	}

	pub fn class_name(&self) -> Symbol {
		match &self.instance {
			ReferenceInstance::Class(class_instance) => class_instance.get().class().name(),
			ReferenceInstance::Array(array_instance) => array_instance.get().class.name(),
			ReferenceInstance::ObjectArray(array_instance) => array_instance.get().class.name(),
			ReferenceInstance::Mirror(mirror_instance) => {
				mirror_instance.get().target_class().name()
			},
			ReferenceInstance::Null => panic!("NullPointerException"),
		}
	}

	pub fn array_length(&self) -> Throws<usize> {
		match &self.instance {
			ReferenceInstance::Array(arr) => Throws::Ok(arr.get().len()),
			ReferenceInstance::ObjectArray(arr) => Throws::Ok(arr.get().len()),
			ReferenceInstance::Null => throw!(@DEFER NullPointerException),
			_ => panic!("Expected an array reference!"),
		}
	}

	pub fn extract_primitive_array(&self) -> PrimitiveArrayInstanceRef {
		match &self.instance {
			ReferenceInstance::Array(arr) => Arc::clone(arr),
			ReferenceInstance::Null => panic!("NullPointerException"),
			_ => panic!("Expected an array reference!"),
		}
	}

	pub fn extract_object_array(&self) -> ObjectArrayInstanceRef {
		match &self.instance {
			ReferenceInstance::ObjectArray(arr) => Arc::clone(arr),
			ReferenceInstance::Null => panic!("NullPointerException"),
			_ => panic!("Expected an object array reference!"),
		}
	}

	pub fn extract_class(&self) -> ClassInstanceRef {
		match &self.instance {
			ReferenceInstance::Class(class) => Arc::clone(class),
			ReferenceInstance::Null => panic!("NullPointerException"),
			_ => panic!("Expected a class reference!"),
		}
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
	/// [`MirrorInstance::target_class`]: crate::objects::mirror::MirrorInstance::target_class
	pub fn extract_target_class(&self) -> &'static Class {
		match &self.instance {
			ReferenceInstance::Class(class) => class.get().class(),
			ReferenceInstance::Mirror(mirror) => mirror.get().target_class(),
			ReferenceInstance::Array(arr) => arr.get().class,
			ReferenceInstance::ObjectArray(arr) => arr.get().class,
			ReferenceInstance::Null => panic!("NullPointerException"),
		}
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
	pub fn extract_instance_class(&self) -> &'static Class {
		match &self.instance {
			ReferenceInstance::Class(class) => class.get().class(),
			ReferenceInstance::Mirror(mirror) => &mirror.get().class(),
			ReferenceInstance::Array(arr) => &arr.get().class,
			ReferenceInstance::ObjectArray(arr) => &arr.get().class,
			ReferenceInstance::Null => panic!("NullPointerException"),
		}
	}

	pub fn extract_mirror(&self) -> MirrorInstanceRef {
		match &self.instance {
			ReferenceInstance::Mirror(mirror) => Arc::clone(mirror),
			ReferenceInstance::Null => panic!("NullPointerException"),
			_ => panic!("Expected a mirror reference!"),
		}
	}

	/// Extract a mirror instance from a `Class` or `Array` instance, this is NOT the same as `Reference::extract_mirror`
	pub fn extract_class_mirror(&self) -> MirrorInstanceRef {
		match &self.instance {
			ReferenceInstance::Class(class) => class.get().class().mirror(),
			ReferenceInstance::Array(arr) => arr.get().class.mirror(),
			ReferenceInstance::ObjectArray(arr) => arr.get().class.mirror(),
			ReferenceInstance::Null => panic!("NullPointerException"),
			_ => panic!("Expected a class/array reference!"),
		}
	}

	pub fn raw(&self) -> *const () {
		self.instance.raw()
	}
}

// TODO: Can this also handle Reference::Array in the future? Doing many manual checks in jdk.internal.misc.Unsafe
impl Instance for Reference {
	fn header(&self) -> &Header {
		match &self.instance {
			ReferenceInstance::Class(instance) => instance.get().header(),
			ReferenceInstance::Array(instance) => instance.get().header(),
			ReferenceInstance::ObjectArray(instance) => instance.get().header(),
			ReferenceInstance::Mirror(instance) => instance.get().header(),
			ReferenceInstance::Null => {
				unreachable!("Should never attempt to retrieve the header of a null object")
			},
		}
	}

	fn monitor(&self) -> Arc<Monitor> {
		match &self.instance {
			ReferenceInstance::Class(instance) => instance.get().monitor(),
			ReferenceInstance::Array(instance) => instance.get().monitor(),
			ReferenceInstance::ObjectArray(instance) => instance.get().monitor(),
			ReferenceInstance::Mirror(instance) => instance.get().monitor(),
			ReferenceInstance::Null => {
				unreachable!("Should never attempt to retrieve the monitor of a null object")
			},
		}
	}

	fn get_field_value(&self, field: &Field) -> Operand<Reference> {
		match &self.instance {
			ReferenceInstance::Class(class) => class.get().get_field_value(field),
			ReferenceInstance::Mirror(mirror) => mirror.get().get_field_value(field),
			ReferenceInstance::Null => panic!("NullPointerException"),
			_ => panic!("Expected a class reference!"),
		}
	}

	fn get_field_value0(&self, field_idx: usize) -> Operand<Reference> {
		match &self.instance {
			ReferenceInstance::Class(class) => class.get().get_field_value0(field_idx),
			ReferenceInstance::Mirror(mirror) => mirror.get().get_field_value0(field_idx),
			ReferenceInstance::Null => panic!("NullPointerException"),
			_ => panic!("Expected a class reference!"),
		}
	}

	fn put_field_value(&mut self, field: &Field, value: Operand<Reference>) {
		match &self.instance {
			ReferenceInstance::Class(class) => class.get_mut().put_field_value(field, value),
			ReferenceInstance::Mirror(mirror) => mirror.get_mut().put_field_value(field, value),
			ReferenceInstance::Null => panic!("NullPointerException"),
			_ => panic!("Expected a class reference!"),
		}
	}

	fn put_field_value0(&mut self, field_idx: usize, value: Operand<Reference>) {
		match &self.instance {
			ReferenceInstance::Class(class) => class.get_mut().put_field_value0(field_idx, value),
			ReferenceInstance::Mirror(mirror) => {
				mirror.get_mut().put_field_value0(field_idx, value)
			},
			ReferenceInstance::Null => panic!("NullPointerException"),
			_ => panic!("Expected a class reference!"),
		}
	}

	unsafe fn get_field_value_raw(&self, field_idx: usize) -> NonNull<Operand<Reference>> {
		match &self.instance {
			ReferenceInstance::Class(class) => unsafe {
				class.get_mut().get_field_value_raw(field_idx)
			},
			ReferenceInstance::Mirror(mirror) => unsafe {
				mirror.get_mut().get_field_value_raw(field_idx)
			},
			ReferenceInstance::Null => panic!("NullPointerException"),
			_ => panic!("Expected a class reference!"),
		}
	}
}
