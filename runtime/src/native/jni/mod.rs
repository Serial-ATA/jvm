//! # JNI Functions
//!
//! This module contains the definitions for the JNI functions, divided into modules as is [the specification](https://docs.oracle.com/javase/8/docs/technotes/guides/jni/spec/functions.html).

#![allow(unused_variables, non_snake_case)]

use crate::objects::class::Class;
use crate::objects::field::Field;
use crate::objects::method::Method;
use crate::objects::reference::Reference;

use jni::objects::{JClass, JFieldId, JMethodId, JObject};
use jni::sys::{jclass, jfieldID, jmethodID, jobject};

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

pub trait IntoJni {
	type RawJniTy;
	type SafeJniTy;

	/// Convert this type into its raw JNI counterpart
	///
	/// # Examples
	///
	/// ```rust
	/// let class = crate::globals::classes::java_lang_Object();
	/// let class_jni: jni::sys::jclass = class.into_jni();
	/// ```
	fn into_jni(self) -> Self::RawJniTy;
	/// Convert this type into its safe JNI counterpart
	///
	/// # Examples
	///
	/// ```rust
	/// let class = crate::globals::classes::java_lang_Object();
	/// let class_jni_safe: jni::objects::JClass = class.into_jni_safe();
	/// ```
	fn into_jni_safe(self) -> Self::SafeJniTy;
}

impl IntoJni for &'static Class {
	type RawJniTy = jclass;
	type SafeJniTy = JClass;

	#[allow(trivial_casts)]
	fn into_jni(self) -> Self::RawJniTy {
		self as *const _ as jclass
	}

	fn into_jni_safe(self) -> Self::SafeJniTy {
		let raw = self.into_jni();

		// SAFETY: We know that the `jclass` is valid because it was created from a `Class`
		unsafe { JClass::from_raw(raw) }
	}
}

impl IntoJni for &'static Field {
	type RawJniTy = jfieldID;
	type SafeJniTy = JFieldId;

	#[allow(trivial_casts)]
	fn into_jni(self) -> Self::RawJniTy {
		self as *const _ as jfieldID
	}

	fn into_jni_safe(self) -> Self::SafeJniTy {
		let raw = self.into_jni();

		// SAFETY: We know that the `jfieldID` is valid because it was created from a `Field`
		unsafe { JFieldId::from_raw(raw) }
	}
}

impl IntoJni for &'static Method {
	type RawJniTy = jmethodID;
	type SafeJniTy = JMethodId;

	#[allow(trivial_casts)]
	fn into_jni(self) -> Self::RawJniTy {
		self as *const _ as jmethodID
	}

	fn into_jni_safe(self) -> Self::SafeJniTy {
		let raw = self.into_jni();

		// SAFETY: We know that the `jmethodID` is valid because it was created from a `Method`
		unsafe { JMethodId::from_raw(raw) }
	}
}

impl IntoJni for Reference {
	type RawJniTy = jobject;
	type SafeJniTy = JObject;

	#[allow(trivial_casts)]
	fn into_jni(self) -> Self::RawJniTy {
		// Leak the reference to keep it alive indefinitely
		Box::leak::<'static>(Box::new(self)) as *mut Reference as jobject
	}

	fn into_jni_safe(self) -> Self::SafeJniTy {
		let raw = self.into_jni();

		// SAFETY: We know that the `jobject` is valid because it was created from an `Reference`
		unsafe { JObject::from_raw(raw) }
	}
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

/// Create a `Method` from a `jmethodID`
pub unsafe fn method_ref_from_jmethodid(method: jmethodID) -> Option<&'static Method> {
	if method.is_null() {
		return None;
	}

	unsafe {
		let method_ptr = core::mem::transmute::<jmethodID, *const Method>(method);
		Some(&*method_ptr)
	}
}

/// Create a `Reference` from a `jobject`
pub unsafe fn reference_from_jobject(obj: jobject) -> Option<Reference> {
	if obj.is_null() {
		return None;
	}

	unsafe {
		let obj = core::mem::transmute::<jobject, *mut Reference>(obj);
		Some((&*obj).clone())
	}
}

/// Create a `Class` from a `JClass`
#[allow(trivial_casts)]
pub fn safe_classref_from_jclass(class: JClass) -> &'static Class {
	debug_assert!(!class.raw().is_null());

	// SAFETY: We assume that a `JClass`, being from the safe API, was created in a valid way
	unsafe {
		let class_ptr = core::mem::transmute::<jclass, *const Class>(class.raw());
		&*class_ptr
	}
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
