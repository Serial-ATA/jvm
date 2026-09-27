//! # JNI Functions
//!
//! This module contains the definitions for the JNI functions, divided into modules as is [the specification](https://docs.oracle.com/javase/8/docs/technotes/guides/jni/spec/functions.html).

#![allow(unused_variables, non_snake_case)]
#![allow(clippy::missing_safety_doc)]

use crate::classes;
use crate::objects::instance::array::ObjectArrayInstanceRef;
use crate::objects::reference::Reference;

use ::jni::objects::{JObjectArray, JString};
use ::jni::sys::jvalue;
use instructions::Operand;

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
use crate::thread::JavaThread;
use references::JObjectExt;

pub mod reflection;
pub mod register;
pub mod string;
pub mod version;
pub mod vm;
pub mod weak;

fn convert_operand(thread: &'static JavaThread, op: Operand<Reference>) -> jvalue {
	match op {
		// Integers cover all over types (boolean, short, etc)
		Operand::Int(v) => jvalue { i: v },
		Operand::Float(v) => jvalue { f: v },
		Operand::Double(v) => jvalue { d: v },
		Operand::Long(v) => jvalue { j: v },
		Operand::Reference(v) => {
			let obj_ref = thread.jni_refs().allocate(v);
			jvalue { l: obj_ref }
		},
		Operand::Empty => unreachable!(),
	}
}

pub trait JniStringExt {
	/// Call [`java::lang::String::extract()`] on this string
	///
	/// # Safety
	///
	/// The `JString` is assumed to point to a valid `java/lang/String` object.
	///
	/// [`java::lang::String::extract()`]: crate::java::lang::String::extract
	unsafe fn extract(&self) -> String;
}

impl JniStringExt for JString {
	unsafe fn extract(&self) -> String {
		let Some(string_ref) = (unsafe { self.to_reference() }) else {
			panic!("`JString` is null")
		};
		classes::java::lang::String::extract(string_ref.extract_class())
	}
}

pub trait JniObjectArrayExt {
	/// Shorthand for [`Reference::extract_object_array()`] for JNI object arrays
	///
	/// # Safety
	///
	/// The `JObjectArray` is assumed to point to a valid object array.
	unsafe fn extract_object_array(&self) -> ObjectArrayInstanceRef;
}

impl JniObjectArrayExt for JObjectArray {
	unsafe fn extract_object_array(&self) -> ObjectArrayInstanceRef {
		let Some(obj) = (unsafe { self.to_reference() }) else {
			panic!("`JObjectArray` is null")
		};
		obj.extract_object_array()
	}
}
