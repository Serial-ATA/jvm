//! # JNI Functions
//!
//! This module contains the definitions for the JNI functions, divided into modules as is [the specification](https://docs.oracle.com/javase/8/docs/technotes/guides/jni/spec/functions.html).

#![allow(unused_variables, non_snake_case)]
#![allow(clippy::missing_safety_doc)]

use crate::objects::class::ClassPtr;
use crate::objects::field::Field;
use crate::objects::method::Method;
use crate::objects::reference::Reference;

use instructions::Operand;
use jni::objects::{JClass, JFieldId, JMethodId, JObject, JString, JValue};
use jni::sys::{jclass, jfieldID, jmethodID, jobject, jvalue};

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

/// Extra methods to convert [`Reference`]s to more concrete JNI types (e.g. [`JString`])
///
/// The [`IntoJni`] implementation for [`Reference`] will always produce a [`JObject`]. Since all other
/// JNI object types are just aliases for [`JObject`], that's correct. However, for documentation purposes
/// it's better to use the most concrete types whenever possible.
pub trait ReferenceJniExt {
	fn into_jstring(self) -> jni::sys::jstring;
	fn into_jstring_safe(self) -> JString;
}

impl ReferenceJniExt for Reference {
	fn into_jstring(self) -> jni::sys::jstring {
		debug_assert_eq!(
			self.extract_instance_class(),
			crate::globals::classes::java_lang_String(),
			"Expected java.lang.String"
		);
		self.into_jni()
	}

	fn into_jstring_safe(self) -> JString {
		// SAFETY: From the rust side, we can only create valid references, anyway
		unsafe { JString::from_raw(self.into_jstring()) }
	}
}

pub trait IntoJni {
	type RawJniTy;
	type SafeJniTy;

	/// Convert this type into its raw JNI counterpart
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use jvm_runtime::native::jni::IntoJni;
	///
	/// let class = jvm_runtime::globals::classes::java_lang_Object();
	/// let class_jni: jni::sys::jclass = class.into_jni();
	/// ```
	fn into_jni(self) -> Self::RawJniTy;
	/// Convert this type into its safe JNI counterpart
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use jvm_runtime::native::jni::IntoJni;
	///
	/// let class = jvm_runtime::globals::classes::java_lang_Object();
	/// let class_jni_safe: jni::objects::JClass = class.into_jni_safe();
	/// ```
	fn into_jni_safe(self) -> Self::SafeJniTy;
}

impl IntoJni for Operand<Reference> {
	type RawJniTy = jvalue;
	type SafeJniTy = JValue;

	fn into_jni(self) -> Self::RawJniTy {
		match self {
			// Integers cover all over types (boolean, short, etc)
			Operand::Int(v) => jvalue { i: v },
			Operand::Float(v) => jvalue { f: v },
			Operand::Double(v) => jvalue { d: v },
			Operand::Long(v) => jvalue { j: v },
			Operand::Reference(v) => jvalue { l: v.into_jni() },
			Operand::Empty => unreachable!(),
		}
	}

	fn into_jni_safe(self) -> Self::SafeJniTy {
		match self {
			Operand::Int(v) => JValue::Int(v),
			Operand::Float(v) => JValue::Float(v),
			Operand::Double(v) => JValue::Double(v),
			Operand::Long(v) => JValue::Long(v),
			Operand::Reference(v) => JValue::Object(v.into_jni_safe()),
			Operand::Empty => unreachable!(),
		}
	}
}

impl IntoJni for ClassPtr {
	type RawJniTy = jclass;
	type SafeJniTy = JClass;

	#[allow(trivial_casts)]
	fn into_jni(self) -> Self::RawJniTy {
		Reference::mirror(self.mirror()).into_jni() as Self::RawJniTy
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
		std::ptr::from_ref(self) as jmethodID
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
		self.raw_tagged() as jobject
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
		let field_ptr = field as *const Field;
		Some(&*field_ptr)
	}
}

/// Create a `Method` from a `jmethodID`
pub unsafe fn method_ref_from_jmethodid(method: jmethodID) -> Option<&'static Method> {
	if method.is_null() {
		return None;
	}

	unsafe {
		let method_ptr = method as *const Method;
		Some(&*method_ptr)
	}
}

/// Create a `Reference` from a `jobject`
pub unsafe fn reference_from_jobject(obj: jobject) -> Option<Reference> {
	if obj.is_null() {
		return None;
	}

	unsafe { Some(Reference::from_raw(obj.cast())) }
}
