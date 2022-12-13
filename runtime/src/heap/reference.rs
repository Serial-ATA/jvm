use super::class::ClassPtr;
use super::field::Field;
use super::method::Method;
use crate::class_instance::{ArrayInstance, ClassInstance};

use std::sync::Arc;

pub type MethodRef = Arc<Method>;
pub type FieldRef = Arc<Field>;
pub type ClassRef = Arc<ClassPtr>;

pub type ClassInstanceRef = Arc<ClassInstance>;
pub type ArrayInstanceRef = Arc<ArrayInstance>;

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.4
#[derive(Debug, Clone, PartialEq)]
pub enum Reference {
	Class(ClassInstanceRef),
	Array(ArrayInstanceRef),
	Interface,
	Null,
}
