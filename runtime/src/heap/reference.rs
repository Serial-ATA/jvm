use super::class::ClassPtr;
use super::field::Field;
use super::method::Method;
use crate::class_instance::{ArrayInstancePtr, ClassInstancePtr, MirrorInstancePtr};

use std::sync::Arc;

pub type MethodRef = Arc<Method>;
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
	Interface,
	Null,
}

impl Reference {
	pub fn is_null(&self) -> bool {
		matches!(self, Self::Null)
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

	pub fn extract_mirror(&self) -> MirrorInstanceRef {
		match self {
			Self::Mirror(mirror) => Arc::clone(mirror),
			Self::Null => panic!("NullPointerException"),
			_ => panic!("Expected a class reference!"),
		}
	}
}
