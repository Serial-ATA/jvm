use super::class::ClassPtr;
use super::field::Field;
use super::method::Method;

use std::sync::Arc;

pub type MethodRef = Arc<Method>;
pub type FieldRef = Arc<Field>;
pub type ClassRef = Arc<ClassPtr>;

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.4
#[derive(Debug, Clone, PartialEq)]
pub enum Reference {
	Class(ClassRef),
	Array,
	Interface,
	Null,
}
