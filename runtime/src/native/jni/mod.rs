//! # JNI Functions
//!
//! This module contains the definitions for the JNI functions, divided into modules as is [the specification](https://docs.oracle.com/javase/8/docs/technotes/guides/jni/spec/functions.html).

#![allow(unused_variables, non_snake_case)]
use crate::objects::class::Class;
use crate::objects::field::Field;

use jni::objects::{JClass, JFieldId};
use jni::sys::{jclass, jfieldID};

pub mod array;
pub mod class;
pub mod exceptions;
pub mod field;
pub mod invocation_api;
pub mod method;
pub mod monitor;
pub mod nio;
pub mod object;
pub mod references;
pub mod reflection;
pub mod register;
pub mod string;
pub mod version;
pub mod vm;
pub mod weak;

/// Create a `JFieldId` from a `Field`
#[allow(trivial_casts)]
pub fn safe_jfieldid_from_field_ref(field: &'static Field) -> JFieldId {
	let raw = jfieldid_from_field_ref(field);

	// SAFETY: We know that the `jclass` is valid because it was created from a `Class`
	unsafe { JFieldId::from_raw(raw) }
}

/// Create a `jfieldID` from a `Field`
#[allow(trivial_casts)]
pub fn jfieldid_from_field_ref(field: &'static Field) -> jfieldID {
	field as *const _ as jfieldID
}

/// Create a `Field` from a `jfieldID`
pub unsafe fn field_ref_from_jfieldid(field: jfieldID) -> Option<&'static Field> {
	if field.is_null() {
		return None;
	}

	unsafe {
		let field_ptr = core::mem::transmute::<jfieldID, *const Field>(field);
		Some(&*field_ptr)
	}
}

/// Create a `JClass` from a `Class`
#[allow(trivial_casts)]
pub fn safe_jclass_from_classref(class: &'static Class) -> JClass {
	let raw = jclass_from_classref(class);

	// SAFETY: We know that the `jclass` is valid because it was created from a `Class`
	unsafe { JClass::from_raw(raw) }
}

/// Create a `jclass` from a `Class`
#[allow(trivial_casts)]
pub fn jclass_from_classref(class: &'static Class) -> jclass {
	class as *const _ as jclass
}

/// Create a `ClassRef` from a `jclass`
pub unsafe fn classref_from_jclass(class: jclass) -> Option<&'static Class> {
	if class.is_null() {
		return None;
	}

	unsafe {
		let class_ptr = core::mem::transmute::<jclass, *const Class>(class);
		Some(&*class_ptr)
	}
}
