//! # JNI Functions
//!
//! This module contains the definitions for the JNI functions, divided into modules as is [the specification](https://docs.oracle.com/javase/8/docs/technotes/guides/jni/spec/functions.html).

#![allow(unused_variables, non_snake_case)]
use crate::objects::class::Class;

use jni::sys::jclass;

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

/// Create a `jclass` from a `ClassRef`
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
