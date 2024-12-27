use super::field::Field;
use crate::objects::class::Class;
use crate::objects::class_instance::{ArrayInstancePtr, ClassInstancePtr, Instance};
use crate::objects::mirror::MirrorInstancePtr;
use crate::objects::monitor::Monitor;
use crate::thread::JavaThread;

use std::ffi::c_void;
use std::ptr::NonNull;
use std::sync::Arc;

use common::traits::PtrType;
use instructions::Operand;
use symbols::Symbol;

pub type ClassInstanceRef = Arc<ClassInstancePtr>;
pub type ArrayInstanceRef = Arc<ArrayInstancePtr>;
pub type MirrorInstanceRef = Arc<MirrorInstancePtr>;

// https://docs.oracle.com/javase/specs/jvms/se23/html/jvms-2.html#jvms-2.4
#[derive(Debug, Clone)]
pub struct Reference {
	instance: ReferenceInstance,
	monitor: Arc<Monitor>,
}

impl PartialEq for Reference {
	fn eq(&self, other: &Self) -> bool {
		self.instance == other.instance
	}
}

#[derive(Debug, Clone, PartialEq)]
enum ReferenceInstance {
	Class(ClassInstanceRef),
	Array(ArrayInstanceRef),
	Mirror(MirrorInstanceRef),
	Null,
}

impl Reference {
	pub fn class(instance: ClassInstanceRef) -> Reference {
		Self {
			instance: ReferenceInstance::Class(instance),
			monitor: Arc::new(Monitor::new()),
		}
	}

	pub fn array(instance: ArrayInstanceRef) -> Reference {
		Self {
			instance: ReferenceInstance::Array(instance),
			monitor: Arc::new(Monitor::new()),
		}
	}

	pub fn mirror(instance: MirrorInstanceRef) -> Reference {
		Self {
			instance: ReferenceInstance::Mirror(instance),
			monitor: Arc::new(Monitor::new()),
		}
	}

	pub fn null() -> Reference {
		Self {
			instance: ReferenceInstance::Null,
			monitor: Arc::new(Monitor::new()),
		}
	}
}

impl Reference {
	pub fn is_class(&self) -> bool {
		matches!(self.instance, ReferenceInstance::Class(_))
	}

	pub fn is_array(&self) -> bool {
		matches!(self.instance, ReferenceInstance::Array(_))
	}

	pub fn is_null(&self) -> bool {
		matches!(self.instance, ReferenceInstance::Null)
	}
}

impl Reference {
	pub fn ptr(&self) -> *const c_void {
		match &self.instance {
			ReferenceInstance::Class(val) => val.as_raw() as _,
			ReferenceInstance::Array(val) => val.as_raw() as _,
			ReferenceInstance::Mirror(val) => val.as_raw() as _,
			ReferenceInstance::Null => core::ptr::null(),
		}
	}
}

impl Reference {
	pub fn monitor_enter(&self, thread: &'static JavaThread) {
		self.monitor.enter(thread);
	}

	pub fn monitor_exit(&self, thread: &'static JavaThread) {
		self.monitor.exit(thread);
	}

	pub fn notify_all(&self) {
		self.monitor.notify_all();
	}
}

impl Reference {
	pub fn is_instance_of(&self, other: &'static Class) -> bool {
		self.extract_instance_class().can_cast_to(other)
	}

	pub fn class_name(&self) -> Symbol {
		match &self.instance {
			ReferenceInstance::Class(class_instance) => class_instance.get().class().name,
			ReferenceInstance::Array(array_instance) => array_instance.get().class.name,
			ReferenceInstance::Mirror(mirror_instance) => mirror_instance.get().target_class().name,
			ReferenceInstance::Null => panic!("NullPointerException"),
		}
	}

	pub fn extract_array(&self) -> ArrayInstanceRef {
		match &self.instance {
			ReferenceInstance::Array(arr) => Arc::clone(arr),
			ReferenceInstance::Null => panic!("NullPointerException"),
			_ => panic!("Expected an array reference!"),
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
			ReferenceInstance::Mirror(mirror) => &mirror.get().target_class(),
			ReferenceInstance::Array(arr) => &arr.get().class,
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
			ReferenceInstance::Null => panic!("NullPointerException"),
			_ => panic!("Expected a class/array reference!"),
		}
	}
}

// TODO: Can this also handle Reference::Array in the future? Doing many manual checks in jdk.internal.misc.Unsafe
impl Instance for Reference {
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
			ReferenceInstance::Class(class) => class.get_mut().get_field_value_raw(field_idx),
			ReferenceInstance::Mirror(mirror) => mirror.get_mut().get_field_value_raw(field_idx),
			ReferenceInstance::Null => panic!("NullPointerException"),
			_ => panic!("Expected a class reference!"),
		}
	}
}
