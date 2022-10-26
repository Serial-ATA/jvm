mod class;
mod field;
mod method;

use crate::class::ClassRef;

pub trait ReferenceType<T> {
	fn new(val: T) -> Self;
	fn as_raw(&self) -> *const T;
	fn as_mut_raw(&self) -> *mut T;
}

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.4
pub enum Reference {
	Class(ClassRef),
	Array,
	Interface,
	Null,
}
