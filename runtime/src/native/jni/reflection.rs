//! # Reflection Support
//!
//! Programmers can use the JNI to call Java methods or access Java fields if they know the name and type of the methods or fields.
//!
//! The Java Core Reflection API allows programmers to introspect Java classes at runtime.
//! JNI provides a set of conversion functions between field and method IDs used in the JNI to field and method objects used in the Java Core Reflection API.

use jni::{jboolean, jclass, jfieldID, jmethodID, jobject, JNIEnv};

pub extern "system" fn FromReflectedMethod(env: *mut JNIEnv, method: jobject) -> jmethodID {
	unimplemented!("jni::FromReflectedMethod")
}

pub extern "system" fn FromReflectedField(env: *mut JNIEnv, field: jobject) -> jfieldID {
	unimplemented!("jni::FromReflectedField")
}

pub extern "system" fn ToReflectedMethod(
	env: *mut JNIEnv,
	cls: jclass,
	methodID: jmethodID,
	isStatic: jboolean,
) -> jobject {
	unimplemented!("jni::ToReflectedMethod")
}

pub extern "system" fn ToReflectedField(
	env: *mut JNIEnv,
	cls: jclass,
	fieldID: jfieldID,
	isStatic: jboolean,
) -> jobject {
	unimplemented!("jni::ToReflectedField")
}
