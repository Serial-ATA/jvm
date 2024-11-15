use super::class::ClassPtr;
use super::field::Field;
use crate::class_instance::{ArrayInstancePtr, ClassInstancePtr, Instance};
use crate::monitor::Monitor;
use crate::objects::mirror::MirrorInstancePtr;
use crate::JavaThread;

use std::ffi::c_void;
use std::ptr::NonNull;
use std::sync::Arc;

use common::traits::PtrType;
use instructions::Operand;
use symbols::{sym, Symbol};

pub type FieldRef = Arc<Field>;
pub type ClassRef = Arc<ClassPtr>;

pub type ClassInstanceRef = Arc<ClassInstancePtr>;
pub type ArrayInstanceRef = Arc<ArrayInstancePtr>;
pub type MirrorInstanceRef = Arc<MirrorInstancePtr>;

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.4
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
}

impl Reference {
	pub fn is_instance_of(&self, T_class: ClassRef) -> bool {
		// The following rules are used to determine whether an objectref that is not null can be cast to the resolved type
		//
		// S is the type of the object referred to by objectref, and T is the resolved class, array, or interface type

		match &self.instance {
			ReferenceInstance::Class(S_class) => {
				// If S is a class type, then:
				//
				//     If T is a class type, then S must be the same class as T, or S must be a subclass of T;
				if !T_class.is_interface() && !T_class.is_array() {
					if S_class.get().class == T_class {
						return true;
					}

					return S_class.get().is_subclass_of(T_class);
				}
				//     If T is an interface type, then S must implement interface T.
				if T_class.is_interface() {
					return S_class.get().implements(T_class);
				}
			},
			ReferenceInstance::Array(S_array) => {
				if S_array.get().class == T_class {
					return true;
				}

				// If S is an array type SC[], that is, an array of components of type SC, then:
				//
				//     If T is a class type, then T must be Object.
				if !T_class.is_interface() && !T_class.is_array() {
					return T_class.name == sym!(java_lang_Object);
				}
				//     If T is an interface type, then T must be one of the interfaces implemented by arrays (JLS §4.10.3).
				if T_class.is_interface() {
					let class_name = T_class.name;
					return class_name == sym!(java_lang_Cloneable)
						|| class_name == sym!(java_io_Serializable);
				}
				//     If T is an array type TC[], that is, an array of components of type TC, then one of the following must be true:
				if T_class.is_array() {
					//         TC and SC are the same primitive type.
					let source_component = &S_array.get().class.unwrap_array_instance().component;
					let dest_component = &T_class.unwrap_array_instance().component;
					if source_component.is_primitive() && dest_component.is_primitive() {
						return source_component == dest_component;
					}

					//         TC and SC are reference types, and type SC can be cast to TC by these run-time rules.
					unimplemented!("Reference::is_instance_of with reference arrays")
				}
			},
			ReferenceInstance::Mirror(S_mirror) => {
				let mirror_deref = S_mirror.get();
				if mirror_deref.class == T_class || mirror_deref.has_target(&T_class) {
					return true;
				}
			},
			ReferenceInstance::Null => return false,
		}

		false
	}

	pub fn class_name(&self) -> Symbol {
		match &self.instance {
			ReferenceInstance::Class(class_instance) => class_instance.get().class.get().name,
			ReferenceInstance::Array(array_instance) => array_instance.get().class.get().name,
			ReferenceInstance::Mirror(mirror_instance) => mirror_instance.get().class.get().name,
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

	pub fn extract_target_class(&self) -> ClassRef {
		match &self.instance {
			ReferenceInstance::Class(class) => Arc::clone(&class.get().class),
			ReferenceInstance::Mirror(mirror) => Arc::clone(&mirror.get().class),
			ReferenceInstance::Array(arr) => Arc::clone(&arr.get().class),
			ReferenceInstance::Null => panic!("NullPointerException"),
			_ => panic!("Expected a class or mirror reference!"),
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
			ReferenceInstance::Class(class) => class.get().class.mirror(),
			ReferenceInstance::Array(arr) => arr.get().class.mirror(),
			ReferenceInstance::Null => panic!("NullPointerException"),
			_ => panic!("Expected a class/array reference!"),
		}
	}
}

// TODO: Can this also handle Reference::Array in the future? Doing many manual checks in jdk.internal.misc.Unsafe
impl Instance for Reference {
	fn get_field_value(&self, field: FieldRef) -> Operand<Reference> {
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

	fn put_field_value(&mut self, field: FieldRef, value: Operand<Reference>) {
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

	unsafe fn get_field_value_raw(&mut self, field_idx: usize) -> NonNull<Operand<Reference>> {
		match &self.instance {
			ReferenceInstance::Class(class) => class.get_mut().get_field_value_raw(field_idx),
			ReferenceInstance::Mirror(mirror) => mirror.get_mut().get_field_value_raw(field_idx),
			ReferenceInstance::Null => panic!("NullPointerException"),
			_ => panic!("Expected a class reference!"),
		}
	}
}
