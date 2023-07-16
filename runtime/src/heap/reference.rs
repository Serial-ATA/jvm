use super::class::ClassPtr;
use super::field::Field;
use super::method::Method;
use crate::class_instance::{ArrayInstancePtr, ClassInstancePtr, Instance};
use crate::heap::mirror::MirrorInstancePtr;
use crate::method::MethodPtr;

use std::sync::Arc;

use common::traits::PtrType;
use instructions::Operand;
use symbols::{sym, Symbol};

pub type MethodRef = Arc<MethodPtr>;
pub type FieldRef = Arc<Field>;
pub type ClassRef = Arc<ClassPtr>;

pub type ClassInstanceRef = Arc<ClassInstancePtr>;
pub type ArrayInstanceRef = Arc<ArrayInstancePtr>;
pub type MirrorInstanceRef = Arc<MirrorInstancePtr>;

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.4
#[derive(Debug, Clone, PartialEq)]
pub enum Reference {
	Class(ClassInstanceRef),
	Array(ArrayInstanceRef),
	Mirror(MirrorInstanceRef),
	Null,
}

impl Reference {
	pub fn is_null(&self) -> bool {
		matches!(self, Self::Null)
	}

	pub fn is_instance_of(&self, class: ClassRef) -> bool {
		// The following rules are used to determine whether an objectref that is not null can be cast to the resolved type
		//
		// S is the type of the object referred to by objectref, and T is the resolved class, array, or interface type

		match self {
			Reference::Class(class_ref) => {
				// If S is a class type, then:
				//
				//     If T is a class type, then S must be the same class as T, or S must be a subclass of T;
				if !class.is_interface() && !class.is_array() {
					if class_ref.get().class == class {
						return true;
					}

					return class_ref.get().is_subclass_of(class);
				}
				//     If T is an interface type, then S must implement interface T.
				if class.is_interface() {
					return class_ref.get().implements(class);
				}
			},
			Reference::Array(array_ref) => {
				if array_ref.get().class == class {
					return true;
				}

				// If S is an array type SC[], that is, an array of components of type SC, then:
				//
				//     If T is a class type, then T must be Object.
				if !class.is_interface() && !class.is_array() {
					return class.get().name == sym!(java_lang_Object);
				}
				//     If T is an interface type, then T must be one of the interfaces implemented by arrays (JLS §4.10.3).
				if class.is_interface() {
					let class_name = class.get().name;
					return class_name == sym!(java_lang_Cloneable)
						|| class_name == sym!(java_io_Serializable);
				}
				//     If T is an array type TC[], that is, an array of components of type TC, then one of the following must be true:
				if class.is_array() {
					//         TC and SC are the same primitive type.
					unimplemented!("Reference::is_instance_of with arrays")
					//         TC and SC are reference types, and type SC can be cast to TC by these run-time rules.
				}
			},
			Reference::Mirror(mirror) => {
				let mirror_deref = mirror.get();
				if mirror_deref.class == class || mirror_deref.has_target(&class) {
					return true;
				}
			},
			Reference::Null => return false,
		}

		false
	}

	pub fn class_name(&self) -> Symbol {
		match self {
			Reference::Class(class_instance) => class_instance.get().class.get().name,
			Reference::Array(array_instance) => array_instance.get().class.get().name,
			Reference::Mirror(mirror_instance) => mirror_instance.get().class.get().name,
			Reference::Null => panic!("NullPointerException"),
		}
	}

	pub fn extract_array(&self) -> ArrayInstanceRef {
		match self {
			Self::Array(arr) => Arc::clone(arr),
			Self::Null => panic!("NullPointerException"),
			_ => panic!("Expected an array reference!"),
		}
	}

	pub fn extract_class(&self) -> ClassInstanceRef {
		match self {
			Self::Class(class) => Arc::clone(class),
			Self::Null => panic!("NullPointerException"),
			_ => panic!("Expected a class reference!"),
		}
	}

	pub fn extract_target_class(&self) -> ClassRef {
		match self {
			Self::Class(class) => Arc::clone(&class.get().class),
			Self::Mirror(mirror) => Arc::clone(&mirror.get().class),
			Self::Null => panic!("NullPointerException"),
			_ => panic!("Expected a class or mirror reference!"),
		}
	}

	pub fn extract_mirror(&self) -> MirrorInstanceRef {
		match self {
			Self::Mirror(mirror) => Arc::clone(mirror),
			Self::Null => panic!("NullPointerException"),
			_ => panic!("Expected a mirror reference!"),
		}
	}

	/// Extract a mirror instance from a `Class` or `Array` instance, this is NOT the same as `Reference::extract_mirror`
	pub fn extract_class_mirror(&self) -> MirrorInstanceRef {
		match self {
			Reference::Class(class) => class.get().class.get_mirror(),
			Reference::Array(arr) => arr.get().class.get_mirror(),
			Self::Null => panic!("NullPointerException"),
			_ => panic!("Expected a class/array reference!"),
		}
	}
}

impl Instance for Reference {
	fn get_field_value(&self, field: FieldRef) -> Operand<Reference> {
		match self {
			Reference::Class(class) => class.get().get_field_value(field),
			Reference::Mirror(mirror) => mirror.get().get_field_value(field),
			Reference::Null => panic!("NullPointerException"),
			_ => panic!("Expected a class reference!"),
		}
	}

	fn get_field_value0(&self, field_idx: usize) -> Operand<Reference> {
		match self {
			Reference::Class(class) => class.get().get_field_value0(field_idx),
			Reference::Mirror(mirror) => mirror.get().get_field_value0(field_idx),
			Reference::Null => panic!("NullPointerException"),
			_ => panic!("Expected a class reference!"),
		}
	}

	fn put_field_value(&mut self, field: FieldRef, value: Operand<Reference>) {
		match self {
			Reference::Class(class) => class.get_mut().put_field_value(field, value),
			Reference::Mirror(mirror) => mirror.get_mut().put_field_value(field, value),
			Reference::Null => panic!("NullPointerException"),
			_ => panic!("Expected a class reference!"),
		}
	}

	fn put_field_value0(&mut self, field_idx: usize, value: Operand<Reference>) {
		match self {
			Reference::Class(class) => class.get_mut().put_field_value0(field_idx, value),
			Reference::Mirror(mirror) => mirror.get_mut().put_field_value0(field_idx, value),
			Reference::Null => panic!("NullPointerException"),
			_ => panic!("Expected a class reference!"),
		}
	}
}
