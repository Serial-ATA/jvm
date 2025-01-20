use super::{classref_from_jclass, reference_from_jobject, IntoJni};
use crate::objects::array::ArrayInstance;
use crate::objects::reference::Reference;

use core::ffi::c_void;
use std::ptr;

use common::int_types::s4;
use common::traits::PtrType;
use instructions::Operand;
use jni::sys::{
	jarray, jboolean, jbooleanArray, jbyte, jbyteArray, jchar, jcharArray, jclass, jdouble,
	jdoubleArray, jfloat, jfloatArray, jint, jintArray, jlong, jlongArray, jobject, jobjectArray,
	jshort, jshortArray, jsize, JNIEnv,
};

#[no_mangle]
pub extern "system" fn GetArrayLength(env: *mut JNIEnv, array: jarray) -> jsize {
	unimplemented!("jni::GetArrayLength");
}

#[no_mangle]
pub extern "system" fn NewObjectArray(
	env: *mut JNIEnv,
	len: jsize,
	clazz: jclass,
	init: jobject,
) -> jobjectArray {
	let class = unsafe { classref_from_jclass(clazz) };
	let Some(class) = class else {
		return ptr::null_mut() as jobjectArray;
	};

	if init.is_null() {
		return Reference::array(ArrayInstance::new_reference(len as s4, class)).into_jni();
	}

	unimplemented!("jni::NewObjectArray with non-null init")
}

#[no_mangle]
pub extern "system" fn GetObjectArrayElement(
	env: *mut JNIEnv,
	array: jobjectArray,
	index: jsize,
) -> jobject {
	unimplemented!("jni::GetObjectArrayElement");
}

#[no_mangle]
pub extern "system" fn SetObjectArrayElement(
	env: *mut JNIEnv,
	array: jobjectArray,
	index: jsize,
	val: jobject,
) {
	let array = unsafe { reference_from_jobject(array as jobject) };
	let Some(array) = array else {
		return; // TODO: NPE?
	};

	let array = array.extract_array();

	let val = unsafe { reference_from_jobject(val) };
	let Some(val) = val else {
		return; // TODO: ArrayStoreException?
	};

	let instance = array.get_mut();
	instance.store(index as s4, Operand::Reference(val));
}

#[no_mangle]
pub extern "system" fn NewBooleanArray(env: *mut JNIEnv, len: jsize) -> jbooleanArray {
	unimplemented!("jni::NewBooleanArray");
}

#[no_mangle]
pub extern "system" fn NewByteArray(env: *mut JNIEnv, len: jsize) -> jbyteArray {
	unimplemented!("jni::NewByteArray");
}

#[no_mangle]
pub extern "system" fn NewCharArray(env: *mut JNIEnv, len: jsize) -> jcharArray {
	unimplemented!("jni::NewCharArray");
}

#[no_mangle]
pub extern "system" fn NewShortArray(env: *mut JNIEnv, len: jsize) -> jshortArray {
	unimplemented!("jni::NewShortArray");
}

#[no_mangle]
pub extern "system" fn NewIntArray(env: *mut JNIEnv, len: jsize) -> jintArray {
	unimplemented!("jni::NewIntArray");
}

#[no_mangle]
pub extern "system" fn NewLongArray(env: *mut JNIEnv, len: jsize) -> jlongArray {
	unimplemented!("jni::NewLongArray");
}

#[no_mangle]
pub extern "system" fn NewFloatArray(env: *mut JNIEnv, len: jsize) -> jfloatArray {
	unimplemented!("jni::NewFloatArray");
}

#[no_mangle]
pub extern "system" fn NewDoubleArray(env: *mut JNIEnv, len: jsize) -> jdoubleArray {
	unimplemented!("jni::NewDoubleArray");
}

#[no_mangle]
pub extern "system" fn GetBooleanArrayElements(
	env: *mut JNIEnv,
	array: jbooleanArray,
	isCopy: *mut jboolean,
) -> *mut jboolean {
	unimplemented!("jni::GetBooleanArrayElements")
}

#[no_mangle]
pub extern "system" fn GetByteArrayElements(
	env: *mut JNIEnv,
	array: jbyteArray,
	isCopy: *mut jboolean,
) -> *mut jbyte {
	unimplemented!("jni::GetByteArrayElements")
}

#[no_mangle]
pub extern "system" fn GetCharArrayElements(
	env: *mut JNIEnv,
	array: jcharArray,
	isCopy: *mut jboolean,
) -> *mut jchar {
	unimplemented!("jni::GetCharArrayElements")
}

#[no_mangle]
pub extern "system" fn GetShortArrayElements(
	env: *mut JNIEnv,
	array: jshortArray,
	isCopy: *mut jboolean,
) -> *mut jshort {
	unimplemented!("jni::GetShortArrayElements")
}

#[no_mangle]
pub extern "system" fn GetIntArrayElements(
	env: *mut JNIEnv,
	array: jintArray,
	isCopy: *mut jboolean,
) -> *mut jint {
	unimplemented!("jni::GetIntArrayElements")
}

#[no_mangle]
pub extern "system" fn GetLongArrayElements(
	env: *mut JNIEnv,
	array: jlongArray,
	isCopy: *mut jboolean,
) -> *mut jlong {
	unimplemented!("jni::GetLongArrayElements")
}

#[no_mangle]
pub extern "system" fn GetFloatArrayElements(
	env: *mut JNIEnv,
	array: jfloatArray,
	isCopy: *mut jboolean,
) -> *mut jfloat {
	unimplemented!("jni::GetFloatArrayElements")
}

#[no_mangle]
pub extern "system" fn GetDoubleArrayElements(
	env: *mut JNIEnv,
	array: jdoubleArray,
	isCopy: *mut jboolean,
) -> *mut jdouble {
	unimplemented!("jni::GetDoubleArrayElements")
}

#[no_mangle]
pub extern "system" fn ReleaseBooleanArrayElements(
	env: *mut JNIEnv,
	array: jbooleanArray,
	elems: *mut jboolean,
	mode: jint,
) {
	unimplemented!("jni::ReleaseBooleanArrayElements")
}

#[no_mangle]
pub extern "system" fn ReleaseByteArrayElements(
	env: *mut JNIEnv,
	array: jbyteArray,
	elems: *mut jbyte,
	mode: jint,
) {
	unimplemented!("jni::ReleaseByteArrayElements")
}

#[no_mangle]
pub extern "system" fn ReleaseCharArrayElements(
	env: *mut JNIEnv,
	array: jcharArray,
	elems: *mut jchar,
	mode: jint,
) {
	unimplemented!("jni::ReleaseCharArrayElements")
}

#[no_mangle]
pub extern "system" fn ReleaseShortArrayElements(
	env: *mut JNIEnv,
	array: jshortArray,
	elems: *mut jshort,
	mode: jint,
) {
	unimplemented!("jni::ReleaseShortArrayElements")
}

#[no_mangle]
pub extern "system" fn ReleaseIntArrayElements(
	env: *mut JNIEnv,
	array: jintArray,
	elems: *mut jint,
	mode: jint,
) {
	unimplemented!("jni::ReleaseIntArrayElements");
}

#[no_mangle]
pub extern "system" fn ReleaseLongArrayElements(
	env: *mut JNIEnv,
	array: jlongArray,
	elems: *mut jlong,
	mode: jint,
) {
	unimplemented!("jni::ReleaseLongArrayElements")
}

#[no_mangle]
pub extern "system" fn ReleaseFloatArrayElements(
	env: *mut JNIEnv,
	array: jfloatArray,
	elems: *mut jfloat,
	mode: jint,
) {
	unimplemented!("jni::ReleaseFloatArrayElements")
}

#[no_mangle]
pub extern "system" fn ReleaseDoubleArrayElements(
	env: *mut JNIEnv,
	array: jdoubleArray,
	elems: *mut jdouble,
	mode: jint,
) {
	unimplemented!("jni::ReleaseDoubleArrayElements")
}

#[no_mangle]
pub extern "system" fn GetBooleanArrayRegion(
	env: *mut JNIEnv,
	array: jbooleanArray,
	start: jsize,
	l: jsize,
	buf: *mut jboolean,
) {
	unimplemented!("jni::GetBooleanArrayRegion")
}

#[no_mangle]
pub extern "system" fn GetByteArrayRegion(
	env: *mut JNIEnv,
	array: jbyteArray,
	start: jsize,
	len: jsize,
	buf: *mut jbyte,
) {
	unimplemented!("jni::GetByteArrayRegion")
}

#[no_mangle]
pub extern "system" fn GetCharArrayRegion(
	env: *mut JNIEnv,
	array: jcharArray,
	start: jsize,
	len: jsize,
	buf: *mut jchar,
) {
	unimplemented!("jni::GetCharArrayRegion")
}

#[no_mangle]
pub extern "system" fn GetShortArrayRegion(
	env: *mut JNIEnv,
	array: jshortArray,
	start: jsize,
	len: jsize,
	buf: *mut jshort,
) {
	unimplemented!("jni::GetShortArrayRegion")
}

#[no_mangle]
pub extern "system" fn GetIntArrayRegion(
	env: *mut JNIEnv,
	array: jintArray,
	start: jsize,
	len: jsize,
	buf: *mut jint,
) {
	unimplemented!("jni::GetIntArrayRegion")
}

#[no_mangle]
pub extern "system" fn GetLongArrayRegion(
	env: *mut JNIEnv,
	array: jlongArray,
	start: jsize,
	len: jsize,
	buf: *mut jlong,
) {
	unimplemented!("jni::GetLongArrayRegion")
}

#[no_mangle]
pub extern "system" fn GetFloatArrayRegion(
	env: *mut JNIEnv,
	array: jfloatArray,
	start: jsize,
	len: jsize,
	buf: *mut jfloat,
) {
	unimplemented!("jni::GetFloatArrayRegion")
}

#[no_mangle]
pub extern "system" fn GetDoubleArrayRegion(
	env: *mut JNIEnv,
	array: jdoubleArray,
	start: jsize,
	len: jsize,
	buf: *mut jdouble,
) {
	unimplemented!("jni::GetDoubleArrayRegion")
}

#[no_mangle]
pub extern "system" fn SetBooleanArrayRegion(
	env: *mut JNIEnv,
	array: jbooleanArray,
	start: jsize,
	l: jsize,
	buf: *const jboolean,
) {
	unimplemented!("jni::SetBooleanArrayRegion")
}

#[no_mangle]
pub extern "system" fn SetByteArrayRegion(
	env: *mut JNIEnv,
	array: jbyteArray,
	start: jsize,
	len: jsize,
	buf: *const jbyte,
) {
	unimplemented!("jni::SetByteArrayRegion")
}

#[no_mangle]
pub extern "system" fn SetCharArrayRegion(
	env: *mut JNIEnv,
	array: jcharArray,
	start: jsize,
	len: jsize,
	buf: *const jchar,
) {
	unimplemented!("jni::SetCharArrayRegion")
}

#[no_mangle]
pub extern "system" fn SetShortArrayRegion(
	env: *mut JNIEnv,
	array: jshortArray,
	start: jsize,
	len: jsize,
	buf: *const jshort,
) {
	unimplemented!("jni::SetShortArrayRegion")
}

#[no_mangle]
pub extern "system" fn SetIntArrayRegion(
	env: *mut JNIEnv,
	array: jintArray,
	start: jsize,
	len: jsize,
	buf: *const jint,
) {
	unimplemented!("jni::SetIntArrayRegion")
}

#[no_mangle]
pub extern "system" fn SetLongArrayRegion(
	env: *mut JNIEnv,
	array: jlongArray,
	start: jsize,
	len: jsize,
	buf: *const jlong,
) {
	unimplemented!("jni::SetLongArrayRegion")
}

#[no_mangle]
pub extern "system" fn SetFloatArrayRegion(
	env: *mut JNIEnv,
	array: jfloatArray,
	start: jsize,
	len: jsize,
	buf: *const jfloat,
) {
	unimplemented!("jni::SetFloatArrayRegion")
}

#[no_mangle]
pub extern "system" fn SetDoubleArrayRegion(
	env: *mut JNIEnv,
	array: jdoubleArray,
	start: jsize,
	len: jsize,
	buf: *const jdouble,
) {
	unimplemented!("jni::SetDoubleArrayRegion")
}

#[no_mangle]
pub extern "system" fn GetPrimitiveArrayCritical(
	env: *mut JNIEnv,
	array: jarray,
	isCopy: *mut jboolean,
) -> *mut c_void {
	unimplemented!("jni::GetPrimitiveArrayCritical")
}

#[no_mangle]
pub extern "system" fn ReleasePrimitiveArrayCritical(
	env: *mut JNIEnv,
	array: jarray,
	carray: *mut c_void,
	mode: jint,
) {
	unimplemented!("jni::ReleasePrimitiveArrayCritical");
}
