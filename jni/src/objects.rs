use jni_sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jobject, jshort, jvalue};
use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone)]
pub enum JValue {
	Boolean(jboolean),
	Byte(jbyte),
	Char(jchar),
	Short(jshort),
	Int(jint),
	Long(jlong),
	Float(jfloat),
	Double(jdouble),
	Object(jobject),
}

impl Debug for JValue {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		match self {
			JValue::Boolean(v) => v.fmt(f),
			JValue::Byte(v) => v.fmt(f),
			JValue::Char(v) => v.fmt(f),
			JValue::Short(v) => v.fmt(f),
			JValue::Int(v) => v.fmt(f),
			JValue::Long(v) => v.fmt(f),
			JValue::Float(v) => v.fmt(f),
			JValue::Double(v) => v.fmt(f),
			JValue::Object(v) => v.fmt(f),
		}
	}
}

impl JValue {
	pub fn raw(self) -> jvalue {
		match self {
			JValue::Boolean(b) => jvalue { z: b },
			JValue::Byte(b) => jvalue { b },
			JValue::Char(c) => jvalue { c },
			JValue::Short(s) => jvalue { s },
			JValue::Int(i) => jvalue { i },
			JValue::Long(l) => jvalue { j: l },
			JValue::Float(f) => jvalue { f },
			JValue::Double(d) => jvalue { d },
			JValue::Object(o) => jvalue { l: o },
		}
	}
}

impl From<jboolean> for JValue {
	fn from(other: jboolean) -> Self {
		JValue::Boolean(other)
	}
}

impl From<jbyte> for JValue {
	fn from(other: jbyte) -> Self {
		JValue::Byte(other)
	}
}

impl From<jchar> for JValue {
	fn from(other: jchar) -> Self {
		JValue::Char(other)
	}
}

impl From<jshort> for JValue {
	fn from(other: jshort) -> Self {
		JValue::Short(other)
	}
}

impl From<jint> for JValue {
	fn from(other: jint) -> Self {
		JValue::Int(other)
	}
}

impl From<jlong> for JValue {
	fn from(other: jlong) -> Self {
		JValue::Long(other)
	}
}

impl From<jfloat> for JValue {
	fn from(other: jfloat) -> Self {
		JValue::Float(other)
	}
}

impl From<jdouble> for JValue {
	fn from(other: jdouble) -> Self {
		JValue::Double(other)
	}
}

macro_rules! define_object_types {
	(
        $(
        struct $name:ident($ty:path)
        );+
        $(;)?
    ) => {
        $(
		#[derive(Copy, Clone, PartialEq, Eq)]
		pub struct $name($ty);

		impl $name {
			pub unsafe fn from_raw(raw: $ty) -> Self {
				Self(raw)
			}

			pub fn raw(&self) -> $ty {
				self.0
			}
		}

		impl From<$name> for JValue {
			fn from(value: $name) -> Self {
				JValue::Object(value.0)
			}
		}

		impl From<$name> for JObject {
			fn from(value: $name) -> Self {
				JObject(value.0)
			}
		}
        )+
	};
}

define_object_types! {
	struct JClass(jni_sys::jclass);
	struct JThrowable(jni_sys::jthrowable);
	struct JString(jni_sys::jstring);
	struct JWeak(jni_sys::jweak);

	struct JArray(jni_sys::jarray);
	struct JBooleanArray(jni_sys::jbooleanArray);
	struct JByteArray(jni_sys::jbyteArray);
	struct JCharArray(jni_sys::jcharArray);
	struct JShortArray(jni_sys::jshortArray);
	struct JIntArray(jni_sys::jintArray);
	struct JLongArray(jni_sys::jlongArray);
	struct JFloatArray(jni_sys::jfloatArray);
	struct JDoubleArray(jni_sys::jdoubleArray);
	struct JObjectArray(jni_sys::jobjectArray);
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct JObject(jni_sys::jobject);

impl JObject {
	pub unsafe fn from_raw(raw: jni_sys::jobject) -> Self {
		Self(raw)
	}

	pub fn raw(&self) -> jni_sys::jobject {
		self.0
	}

	pub fn is_null(&self) -> bool {
		self.raw().is_null()
	}
}

impl From<JObject> for JValue {
	fn from(value: JObject) -> Self {
		JValue::Object(value.0)
	}
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct JFieldId(jni_sys::jfieldID);

impl JFieldId {
	pub unsafe fn from_raw(raw: jni_sys::jfieldID) -> Self {
		Self(raw)
	}

	pub fn raw(&self) -> jni_sys::jfieldID {
		self.0
	}
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct JMethodId(jni_sys::jmethodID);

impl JMethodId {
	pub unsafe fn from_raw(raw: jni_sys::jmethodID) -> Self {
		Self(raw)
	}

	pub fn raw(&self) -> jni_sys::jmethodID {
		self.0
	}
}
