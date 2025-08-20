use crate::native::jni::{IntoJni, reference_from_jobject};
use crate::symbols::Symbol;
use crate::thread::JavaThread;
use crate::thread::exceptions::Throws;

use core::ffi::c_char;
use std::ffi::CStr;

use common::unicode;
use jni::sys::{
	JNIEnv, jboolean, jbyte, jchar, jclass, jdouble, jfieldID, jfloat, jint, jlong, jobject, jshort,
};

// --------------
//   NON-STATIC
// --------------

pub extern "system" fn GetFieldID(
	env: *mut JNIEnv,
	clazz: jclass,
	name: *const c_char,
	sig: *const c_char,
) -> jfieldID {
	let thread = JavaThread::current();
	assert_eq!(thread.env().raw(), env);

	let Some(class) = (unsafe { reference_from_jobject(clazz) }) else {
		panic!("Invalid arguments to `GetFieldID`");
	};

	let name_c = unsafe { CStr::from_ptr(name) };
	let sig_c = unsafe { CStr::from_ptr(sig) };

	let Ok(name) = unicode::decode(name_c.to_bytes()) else {
		return std::ptr::null_mut();
	};
	let Ok(sig) = unicode::decode(sig_c.to_bytes()) else {
		return std::ptr::null_mut();
	};

	let name_sym = Symbol::intern(name);
	let sig_sym = Symbol::intern(sig);

	match class
		.extract_target_class()
		.resolve_field(name_sym, sig_sym)
	{
		Throws::Ok(field) => field.into_jni(),
		Throws::Exception(e) => {
			e.throw(thread);
			std::ptr::null_mut()
		},
	}
}

pub extern "system" fn GetObjectField(
	env: *mut JNIEnv,
	obj: jobject,
	fieldID: jfieldID,
) -> jobject {
	unimplemented!("jni::GetObjectField");
}

pub extern "system" fn GetBooleanField(
	env: *mut JNIEnv,
	obj: jobject,
	fieldID: jfieldID,
) -> jboolean {
	unimplemented!("jni::GetBooleanField");
}

pub extern "system" fn GetByteField(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jbyte {
	unimplemented!("jni::GetByteField");
}

pub extern "system" fn GetCharField(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jchar {
	unimplemented!("jni::GetCharField");
}

pub extern "system" fn GetShortField(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jshort {
	unimplemented!("jni::GetShortField");
}

pub extern "system" fn GetIntField(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jint {
	unimplemented!("jni::GetIntField");
}

pub extern "system" fn GetLongField(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jlong {
	unimplemented!("jni::GetLongField");
}

pub extern "system" fn GetFloatField(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID) -> jfloat {
	unimplemented!("jni::GetFloatField");
}

pub extern "system" fn GetDoubleField(
	env: *mut JNIEnv,
	obj: jobject,
	fieldID: jfieldID,
) -> jdouble {
	unimplemented!("jni::GetDoubleField");
}

pub extern "system" fn SetObjectField(
	env: *mut JNIEnv,
	obj: jobject,
	fieldID: jfieldID,
	val: jobject,
) {
	unimplemented!("jni::SetObjectField");
}

pub extern "system" fn SetBooleanField(
	env: *mut JNIEnv,
	obj: jobject,
	fieldID: jfieldID,
	val: jboolean,
) {
	unimplemented!("jni::SetBooleanField");
}

pub extern "system" fn SetByteField(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jbyte) {
	unimplemented!("jni::SetByteField");
}

pub extern "system" fn SetCharField(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jchar) {
	unimplemented!("jni::SetCharField");
}

pub extern "system" fn SetShortField(
	env: *mut JNIEnv,
	obj: jobject,
	fieldID: jfieldID,
	val: jshort,
) {
	unimplemented!("jni::SetShortField");
}

pub extern "system" fn SetIntField(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jint) {
	unimplemented!("jni::SetIntField");
}

pub extern "system" fn SetLongField(env: *mut JNIEnv, obj: jobject, fieldID: jfieldID, val: jlong) {
	unimplemented!("jni::SetLongField");
}

pub extern "system" fn SetFloatField(
	env: *mut JNIEnv,
	obj: jobject,
	fieldID: jfieldID,
	val: jfloat,
) {
	unimplemented!("jni::SetFloatField");
}

pub extern "system" fn SetDoubleField(
	env: *mut JNIEnv,
	obj: jobject,
	fieldID: jfieldID,
	val: jdouble,
) {
	unimplemented!("jni::SetDoubleField");
}

// --------------
//     STATIC
// --------------

pub extern "system" fn GetStaticFieldID(
	env: *mut JNIEnv,
	clazz: jclass,
	name: *const c_char,
	sig: *const c_char,
) -> jfieldID {
	unimplemented!("jni::GetStaticFieldID")
}

pub extern "system" fn GetStaticObjectField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
) -> jobject {
	unimplemented!("jni::GetStaticObjectField");
}

pub extern "system" fn GetStaticBooleanField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
) -> jboolean {
	unimplemented!("jni::GetStaticBooleanField");
}

pub extern "system" fn GetStaticByteField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
) -> jbyte {
	unimplemented!("jni::GetStaticByteField");
}

pub extern "system" fn GetStaticCharField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
) -> jchar {
	unimplemented!("jni::GetStaticCharField");
}

pub extern "system" fn GetStaticShortField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
) -> jshort {
	unimplemented!("jni::GetStaticShortField");
}

pub extern "system" fn GetStaticIntField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
) -> jint {
	unimplemented!("jni::GetStaticIntField");
}

pub extern "system" fn GetStaticLongField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
) -> jlong {
	unimplemented!("jni::GetStaticLongField");
}

pub extern "system" fn GetStaticFloatField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
) -> jfloat {
	unimplemented!("jni::GetStaticFloatField");
}

pub extern "system" fn GetStaticDoubleField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
) -> jdouble {
	unimplemented!("jni::GetStaticDoubleField");
}

pub extern "system" fn SetStaticObjectField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
	value: jobject,
) {
	unimplemented!("jni::SetStaticObjectField")
}

pub extern "system" fn SetStaticBooleanField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
	value: jboolean,
) {
	unimplemented!("jni::SetStaticBooleanField")
}

pub extern "system" fn SetStaticByteField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
	value: jbyte,
) {
	unimplemented!("jni::SetStaticByteField");
}

pub extern "system" fn SetStaticCharField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
	value: jchar,
) {
	unimplemented!("jni::SetStaticCharField");
}

pub extern "system" fn SetStaticShortField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
	value: jshort,
) {
	unimplemented!("jni::SetStaticShortField")
}

pub extern "system" fn SetStaticIntField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
	value: jint,
) {
	unimplemented!("jni::SetStaticIntField");
}

pub extern "system" fn SetStaticLongField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
	value: jlong,
) {
	unimplemented!("jni::SetStaticLongField");
}

pub extern "system" fn SetStaticFloatField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
	value: jfloat,
) {
	unimplemented!("jni::SetStaticFloatField")
}

pub extern "system" fn SetStaticDoubleField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
	value: jdouble,
) {
	unimplemented!("jni::SetStaticDoubleField")
}
