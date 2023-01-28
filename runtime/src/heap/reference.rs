use super::class::ClassPtr;
use super::field::Field;
use super::method::Method;
use crate::class_instance::{ArrayInstancePtr, ClassInstancePtr, Instance, MirrorInstancePtr};

use common::traits::PtrType;
use instructions::Operand;
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
