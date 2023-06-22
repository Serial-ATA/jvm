#![allow(unused_variables)]

use core::ffi::{c_char, c_void, VaList};
use jni::{
	jarray, jboolean, jbooleanArray, jbyte, jbyteArray, jchar, jcharArray, jclass, jdouble,
	jdoubleArray, jfieldID, jfloat, jfloatArray, jint, jintArray, jlong, jlongArray, jmethodID,
	jobject, jobjectArray, jobjectRefType, jshort, jshortArray, jsize, jstring, jthrowable, jvalue,
	jweak, JNIEnv, JNINativeMethod, JavaVM,
};

pub extern "system" fn GetVersion(env: *mut JNIEnv) -> jint {
	unimplemented!("jni::GetVersion")
}

pub extern "system" fn DefineClass(
	env: *mut JNIEnv,
	name: *const c_char,
	loader: jobject,
	buf: *const jbyte,
	len: jsize,
) -> jclass {
	unimplemented!("jni::DefineClass")
}

pub extern "system" fn FindClass(env: *mut JNIEnv, name: *const c_char) -> jclass {
	unimplemented!("jni::FindClass")
}

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

pub extern "system" fn GetSuperclass(env: *mut JNIEnv, sub: jclass) -> jclass {
	unimplemented!("jni::GetSuperclass")
}

pub extern "system" fn IsAssignableFrom(env: *mut JNIEnv, sub: jclass, sup: jclass) -> jboolean {
	unimplemented!("jni::IsAssignableFrom")
}

pub extern "system" fn ToReflectedField(
	env: *mut JNIEnv,
	cls: jclass,
	fieldID: jfieldID,
	isStatic: jboolean,
) -> jobject {
	unimplemented!("jni::ToReflectedField")
}

pub extern "system" fn Throw(env: *mut JNIEnv, obj: jthrowable) -> jint {
	unimplemented!("jni::Throw");
}

pub extern "system" fn ThrowNew(env: *mut JNIEnv, clazz: jclass, msg: *const c_char) -> jint {
	unimplemented!("jni::ThrowNew");
}

pub extern "system" fn ExceptionOccurred(env: *mut JNIEnv) -> jthrowable {
	unimplemented!("jni::ExceptionOccurred");
}

pub extern "system" fn ExceptionDescribe(env: *mut JNIEnv) {
	unimplemented!("jni::ExceptionDescribe");
}

pub extern "system" fn ExceptionClear(env: *mut JNIEnv) {
	unimplemented!("jni::ExceptionClear");
}

pub extern "system" fn FatalError(env: *mut JNIEnv, msg: *const c_char) -> ! {
	unimplemented!("jni::FatalError");
}

pub extern "system" fn PushLocalFrame(env: *mut JNIEnv, capacity: jint) -> jint {
	unimplemented!("jni::PushLocalFrame");
}

pub extern "system" fn PopLocalFrame(env: *mut JNIEnv, result: jobject) -> jobject {
	unimplemented!("jni::PopLocalFrame");
}

pub extern "system" fn NewGlobalRef(env: *mut JNIEnv, lobj: jobject) -> jobject {
	unimplemented!("jni::NewGlobalRef");
}

pub extern "system" fn DeleteGlobalRef(env: *mut JNIEnv, gref: jobject) {
	unimplemented!("jni::DeleteGlobalRef");
}

pub extern "system" fn DeleteLocalRef(env: *mut JNIEnv, obj: jobject) {
	unimplemented!("jni::DeleteLocalRef");
}

pub extern "system" fn IsSameObject(env: *mut JNIEnv, obj1: jobject, obj2: jobject) -> jboolean {
	unimplemented!("jni::IsSameObject");
}

pub extern "system" fn NewLocalRef(env: *mut JNIEnv, ref_: jobject) -> jobject {
	unimplemented!("jni::NewLocalRef");
}

pub extern "system" fn EnsureLocalCapacity(env: *mut JNIEnv, capacity: jint) -> jint {
	unimplemented!("jni::EnsureLocalCapacity");
}

pub extern "system" fn AllocObject(env: *mut JNIEnv, clazz: jclass) -> jobject {
	unimplemented!("jni::AllocObject");
}

pub unsafe extern "C" fn NewObject(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jobject {
	unimplemented!("jni::NewObject");
}

extern "system" fn NewObjectV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jobject {
	unimplemented!("jni::NewObjectV")
}

extern "system" fn NewObjectA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jobject {
	unimplemented!("jni::NewObjectA")
}

pub extern "system" fn GetObjectClass(env: *mut JNIEnv, obj: jobject) -> jclass {
	unimplemented!("jni::GetObjectClass");
}

pub extern "system" fn IsInstanceOf(env: *mut JNIEnv, obj: jobject, clazz: jclass) -> jboolean {
	unimplemented!("jni::IsInstanceOf");
}

extern "system" fn GetMethodID(
	env: *mut JNIEnv,
	clazz: jclass,
	name: *const c_char,
	sig: *const c_char,
) -> jmethodID {
	unimplemented!("jni::GetMethodID")
}

pub unsafe extern "C" fn CallObjectMethod(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	...
) -> jobject {
	unimplemented!("jni::CallObjectMethod");
}

extern "system" fn CallObjectMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jobject {
	unimplemented!("jni::CallObjectMethodV")
}

extern "system" fn CallObjectMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) -> jobject {
	unimplemented!("jni::CallObjectMethodA")
}

pub unsafe extern "C" fn CallBooleanMethod(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	...
) -> jboolean {
	unimplemented!("jni::CallBooleanMethod");
}

extern "system" fn CallBooleanMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jboolean {
	unimplemented!("jni::CallBooleanMethodV")
}

extern "system" fn CallBooleanMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) -> jboolean {
	unimplemented!("jni::CallBooleanMethodA")
}

pub unsafe extern "C" fn CallByteMethod(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	...
) -> jbyte {
	unimplemented!("jni::CallByteMethod");
}

extern "system" fn CallByteMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jbyte {
	unimplemented!("jni::CallByteMethodV")
}

extern "system" fn CallByteMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) -> jbyte {
	unimplemented!("jni::CallByteMethodA")
}

pub unsafe extern "C" fn CallCharMethod(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	...
) -> jchar {
	unimplemented!("jni::CallCharMethod");
}

extern "system" fn CallCharMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jchar {
	unimplemented!("jni::CallCharMethodV")
}

extern "system" fn CallCharMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) -> jchar {
	unimplemented!("jni::CallCharMethodA")
}

pub unsafe extern "C" fn CallShortMethod(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	...
) -> jshort {
	unimplemented!("jni::CallShortMethod");
}

extern "system" fn CallShortMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jshort {
	unimplemented!("jni::CallShortMethodV")
}

extern "system" fn CallShortMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) -> jshort {
	unimplemented!("jni::CallShortMethodA")
}

pub unsafe extern "C" fn CallIntMethod(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	...
) -> jint {
	unimplemented!("jni::CallIntMethod");
}

extern "system" fn CallIntMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jint {
	unimplemented!("jni::CallIntMethodV")
}

extern "system" fn CallIntMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) -> jint {
	unimplemented!("jni::CallIntMethodA")
}

pub unsafe extern "C" fn CallLongMethod(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	...
) -> jlong {
	unimplemented!("jni::CallLongMethod");
}

extern "system" fn CallLongMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jlong {
	unimplemented!("jni::CallLongMethodV")
}

extern "system" fn CallLongMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) -> jlong {
	unimplemented!("jni::CallLongMethodA")
}

pub unsafe extern "C" fn CallFloatMethod(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	...
) -> jfloat {
	unimplemented!("jni::CallFloatMethod");
}

extern "system" fn CallFloatMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jfloat {
	unimplemented!("jni::CallFloatMethodV")
}

extern "system" fn CallFloatMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) -> jfloat {
	unimplemented!("jni::CallFloatMethodA")
}

pub unsafe extern "C" fn CallDoubleMethod(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	...
) -> jdouble {
	unimplemented!("jni::CallDoubleMethod");
}

extern "system" fn CallDoubleMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jdouble {
	unimplemented!("jni::CallDoubleMethodV")
}

extern "system" fn CallDoubleMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) -> jdouble {
	unimplemented!("jni::CallDoubleMethodA")
}

pub unsafe extern "C" fn CallVoidMethod(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) {
	unimplemented!("jni::CallVoidMethod");
}

extern "system" fn CallVoidMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) {
	unimplemented!("jni::CallVoidMethodV")
}

extern "system" fn CallVoidMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) {
	unimplemented!("jni::CallVoidMethodA")
}

pub unsafe extern "C" fn CallNonvirtualObjectMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jobject {
	unimplemented!("jni::CallNonvirtualObjectMethod")
}

extern "system" fn CallNonvirtualObjectMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jobject {
	unimplemented!("jni::CallNonvirtualObjectMethodV")
}

extern "system" fn CallNonvirtualObjectMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jobject {
	unimplemented!("jni::CallNonvirtualObjectMethodA")
}

pub unsafe extern "C" fn CallNonvirtualBooleanMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jboolean {
	unimplemented!("jni::CallNonvirtualBooleanMethod")
}

extern "system" fn CallNonvirtualBooleanMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jboolean {
	unimplemented!("jni::CallNonvirtualBooleanMethodV")
}

extern "system" fn CallNonvirtualBooleanMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jboolean {
	unimplemented!("jni::CallNonvirtualBooleanMethodA")
}

pub unsafe extern "C" fn CallNonvirtualByteMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jbyte {
	unimplemented!("jni::CallNonvirtualByteMethod")
}

extern "system" fn CallNonvirtualByteMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jbyte {
	unimplemented!("jni::CallNonvirtualByteMethodV")
}

extern "system" fn CallNonvirtualByteMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jbyte {
	unimplemented!("jni::CallNonvirtualByteMethodA")
}

pub unsafe extern "C" fn CallNonvirtualCharMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jchar {
	unimplemented!("jni::CallNonvirtualCharMethod")
}

extern "system" fn CallNonvirtualCharMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jchar {
	unimplemented!("jni::CallNonvirtualCharMethodV")
}

extern "system" fn CallNonvirtualCharMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jchar {
	unimplemented!("jni::CallNonvirtualCharMethodA")
}

pub unsafe extern "C" fn CallNonvirtualShortMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jshort {
	unimplemented!("jni::CallNonvirtualShortMethod")
}

extern "system" fn CallNonvirtualShortMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jshort {
	unimplemented!("jni::CallNonvirtualShortMethodV")
}

extern "system" fn CallNonvirtualShortMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jshort {
	unimplemented!("jni::CallNonvirtualShortMethodA")
}

pub unsafe extern "C" fn CallNonvirtualIntMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jint {
	unimplemented!("jni::CallNonvirtualIntMethod")
}

extern "system" fn CallNonvirtualIntMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jint {
	unimplemented!("jni::CallNonvirtualIntMethodV")
}

extern "system" fn CallNonvirtualIntMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jint {
	unimplemented!("jni::CallNonvirtualIntMethodA")
}

pub unsafe extern "C" fn CallNonvirtualLongMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jlong {
	unimplemented!("jni::CallNonvirtualLongMethod")
}

extern "system" fn CallNonvirtualLongMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jlong {
	unimplemented!("jni::CallNonvirtualLongMethodV")
}

extern "system" fn CallNonvirtualLongMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jlong {
	unimplemented!("jni::CallNonvirtualLongMethodA")
}

pub unsafe extern "C" fn CallNonvirtualFloatMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jfloat {
	unimplemented!("jni::CallNonvirtualFloatMethod")
}

extern "system" fn CallNonvirtualFloatMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jfloat {
	unimplemented!("jni::CallNonvirtualFloatMethodV")
}

extern "system" fn CallNonvirtualFloatMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jfloat {
	unimplemented!("jni::CallNonvirtualFloatMethodA")
}

pub unsafe extern "C" fn CallNonvirtualDoubleMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jdouble {
	unimplemented!("jni::CallNonvirtualDoubleMethod")
}

extern "system" fn CallNonvirtualDoubleMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jdouble {
	unimplemented!("jni::CallNonvirtualDoubleMethodV")
}

extern "system" fn CallNonvirtualDoubleMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jdouble {
	unimplemented!("jni::CallNonvirtualDoubleMethodA")
}

pub unsafe extern "C" fn CallNonvirtualVoidMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) {
	unimplemented!("jni::CallNonvirtualVoidMethod")
}

extern "system" fn CallNonvirtualVoidMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) {
	unimplemented!("jni::CallNonvirtualVoidMethodV")
}

extern "system" fn CallNonvirtualVoidMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) {
	unimplemented!("jni::CallNonvirtualVoidMethodA")
}

extern "system" fn GetFieldID(
	env: *mut JNIEnv,
	clazz: jclass,
	name: *const c_char,
	sig: *const c_char,
) -> jfieldID {
	unimplemented!("jni::GetFieldID")
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

extern "system" fn GetStaticMethodID(
	env: *mut JNIEnv,
	clazz: jclass,
	name: *const c_char,
	sig: *const c_char,
) -> jmethodID {
	unimplemented!("jni::GetStaticMethodID")
}

pub unsafe extern "C" fn CallStaticObjectMethod(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jobject {
	unimplemented!("jni::CallStaticObjectMethod");
}

extern "system" fn CallStaticObjectMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jobject {
	unimplemented!("jni::CallStaticObjectMethodV")
}

extern "system" fn CallStaticObjectMethodA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jobject {
	unimplemented!("jni::CallStaticObjectMethodA")
}

pub unsafe extern "C" fn CallStaticBooleanMethod(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jboolean {
	unimplemented!("jni::CallStaticBooleanMethod");
}

extern "system" fn CallStaticBooleanMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jboolean {
	unimplemented!("jni::CallStaticBooleanMethodV")
}

extern "system" fn CallStaticBooleanMethodA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jboolean {
	unimplemented!("jni::CallStaticBooleanMethodA")
}

pub unsafe extern "C" fn CallStaticByteMethod(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jbyte {
	unimplemented!("jni::CallStaticByteMethod");
}

extern "system" fn CallStaticByteMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jbyte {
	unimplemented!("jni::CallStaticByteMethodV")
}

extern "system" fn CallStaticByteMethodA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jbyte {
	unimplemented!("jni::CallStaticByteMethodA")
}

pub unsafe extern "C" fn CallStaticCharMethod(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jchar {
	unimplemented!("jni::CallStaticCharMethod");
}

extern "system" fn CallStaticCharMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jchar {
	unimplemented!("jni::CallStaticCharMethodV")
}

extern "system" fn CallStaticCharMethodA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jchar {
	unimplemented!("jni::CallStaticCharMethodA")
}

pub unsafe extern "C" fn CallStaticShortMethod(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jshort {
	unimplemented!("jni::CallStaticShortMethod");
}

extern "system" fn CallStaticShortMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jshort {
	unimplemented!("jni::CallStaticShortMethodV")
}

extern "system" fn CallStaticShortMethodA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jshort {
	unimplemented!("jni::CallStaticShortMethodA")
}

pub unsafe extern "C" fn CallStaticIntMethod(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jint {
	unimplemented!("jni::CallStaticIntMethod");
}

extern "system" fn CallStaticIntMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jint {
	unimplemented!("jni::CallStaticIntMethodV")
}

extern "system" fn CallStaticIntMethodA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jint {
	unimplemented!("jni::CallStaticIntMethodA")
}

pub unsafe extern "C" fn CallStaticLongMethod(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jlong {
	unimplemented!("jni::CallStaticLongMethod");
}

extern "system" fn CallStaticLongMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jlong {
	unimplemented!("jni::CallStaticLongMethodV")
}

extern "system" fn CallStaticLongMethodA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jlong {
	unimplemented!("jni::CallStaticLongMethodA")
}

pub unsafe extern "C" fn CallStaticFloatMethod(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jfloat {
	unimplemented!("jni::CallStaticFloatMethod");
}

extern "system" fn CallStaticFloatMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jfloat {
	unimplemented!("jni::CallStaticFloatMethodV")
}

extern "system" fn CallStaticFloatMethodA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jfloat {
	unimplemented!("jni::CallStaticFloatMethodA")
}

pub unsafe extern "C" fn CallStaticDoubleMethod(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jdouble {
	unimplemented!("jni::CallStaticDoubleMethod");
}

extern "system" fn CallStaticDoubleMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jdouble {
	unimplemented!("jni::CallStaticDoubleMethodV")
}

extern "system" fn CallStaticDoubleMethodA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jdouble {
	unimplemented!("jni::CallStaticDoubleMethodA")
}

pub unsafe extern "C" fn CallStaticVoidMethod(
	env: *mut JNIEnv,
	cls: jclass,
	methodID: jmethodID,
	...
) {
	unimplemented!("jni::CallStaticVoidMethod");
}

extern "system" fn CallStaticVoidMethodV(
	env: *mut JNIEnv,
	cls: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) {
	unimplemented!("jni::CallStaticVoidMethodV")
}

extern "system" fn CallStaticVoidMethodA(
	env: *mut JNIEnv,
	cls: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) {
	unimplemented!("jni::CallStaticVoidMethodA")
}

extern "system" fn GetStaticFieldID(
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

extern "system" fn SetStaticObjectField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
	value: jobject,
) {
	unimplemented!("jni::SetStaticObjectField")
}

extern "system" fn SetStaticBooleanField(
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

extern "system" fn SetStaticShortField(
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

extern "system" fn SetStaticFloatField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
	value: jfloat,
) {
	unimplemented!("jni::SetStaticFloatField")
}

extern "system" fn SetStaticDoubleField(
	env: *mut JNIEnv,
	clazz: jclass,
	fieldID: jfieldID,
	value: jdouble,
) {
	unimplemented!("jni::SetStaticDoubleField")
}

pub extern "system" fn NewString(env: *mut JNIEnv, unicode: *const jchar, len: jsize) -> jstring {
	unimplemented!("jni::NewString");
}

pub extern "system" fn GetStringLength(env: *mut JNIEnv, str: jstring) -> jsize {
	unimplemented!("jni::GetStringLength");
}

extern "system" fn GetStringChars(
	env: *mut JNIEnv,
	str: jstring,
	isCopy: *mut jboolean,
) -> *const jchar {
	unimplemented!("jni::GetStringChars")
}

pub extern "system" fn ReleaseStringChars(env: *mut JNIEnv, str: jstring, chars: *const jchar) {
	unimplemented!("jni::ReleaseStringChars");
}

pub extern "system" fn NewStringUTF(env: *mut JNIEnv, utf: *const c_char) -> jstring {
	unimplemented!("jni::NewStringUTF");
}

pub extern "system" fn GetStringUTFLength(env: *mut JNIEnv, str: jstring) -> jsize {
	unimplemented!("jni::GetStringUTFLength");
}

extern "system" fn GetStringUTFChars(
	env: *mut JNIEnv,
	str: jstring,
	isCopy: *mut jboolean,
) -> *const c_char {
	unimplemented!("jni::GetStringUTFChars")
}

pub extern "system" fn ReleaseStringUTFChars(env: *mut JNIEnv, str: jstring, chars: *const c_char) {
	unimplemented!("jni::ReleaseStringUTFChars");
}

pub extern "system" fn GetArrayLength(env: *mut JNIEnv, array: jarray) -> jsize {
	unimplemented!("jni::GetArrayLength");
}

extern "system" fn NewObjectArray(
	env: *mut JNIEnv,
	len: jsize,
	clazz: jclass,
	init: jobject,
) -> jobjectArray {
	unimplemented!("jni::NewObjectArray")
}

pub extern "system" fn GetObjectArrayElement(
	env: *mut JNIEnv,
	array: jobjectArray,
	index: jsize,
) -> jobject {
	unimplemented!("jni::GetObjectArrayElement");
}

extern "system" fn SetObjectArrayElement(
	env: *mut JNIEnv,
	array: jobjectArray,
	index: jsize,
	val: jobject,
) {
	unimplemented!("jni::SetObjectArrayElement")
}

pub extern "system" fn NewBooleanArray(env: *mut JNIEnv, len: jsize) -> jbooleanArray {
	unimplemented!("jni::NewBooleanArray");
}

pub extern "system" fn NewByteArray(env: *mut JNIEnv, len: jsize) -> jbyteArray {
	unimplemented!("jni::NewByteArray");
}

pub extern "system" fn NewCharArray(env: *mut JNIEnv, len: jsize) -> jcharArray {
	unimplemented!("jni::NewCharArray");
}

pub extern "system" fn NewShortArray(env: *mut JNIEnv, len: jsize) -> jshortArray {
	unimplemented!("jni::NewShortArray");
}

pub extern "system" fn NewIntArray(env: *mut JNIEnv, len: jsize) -> jintArray {
	unimplemented!("jni::NewIntArray");
}

pub extern "system" fn NewLongArray(env: *mut JNIEnv, len: jsize) -> jlongArray {
	unimplemented!("jni::NewLongArray");
}

pub extern "system" fn NewFloatArray(env: *mut JNIEnv, len: jsize) -> jfloatArray {
	unimplemented!("jni::NewFloatArray");
}

pub extern "system" fn NewDoubleArray(env: *mut JNIEnv, len: jsize) -> jdoubleArray {
	unimplemented!("jni::NewDoubleArray");
}

extern "system" fn GetBooleanArrayElements(
	env: *mut JNIEnv,
	array: jbooleanArray,
	isCopy: *mut jboolean,
) -> *mut jboolean {
	unimplemented!("jni::GetBooleanArrayElements")
}

extern "system" fn GetByteArrayElements(
	env: *mut JNIEnv,
	array: jbyteArray,
	isCopy: *mut jboolean,
) -> *mut jbyte {
	unimplemented!("jni::GetByteArrayElements")
}

extern "system" fn GetCharArrayElements(
	env: *mut JNIEnv,
	array: jcharArray,
	isCopy: *mut jboolean,
) -> *mut jchar {
	unimplemented!("jni::GetCharArrayElements")
}

extern "system" fn GetShortArrayElements(
	env: *mut JNIEnv,
	array: jshortArray,
	isCopy: *mut jboolean,
) -> *mut jshort {
	unimplemented!("jni::GetShortArrayElements")
}

extern "system" fn GetIntArrayElements(
	env: *mut JNIEnv,
	array: jintArray,
	isCopy: *mut jboolean,
) -> *mut jint {
	unimplemented!("jni::GetIntArrayElements")
}

extern "system" fn GetLongArrayElements(
	env: *mut JNIEnv,
	array: jlongArray,
	isCopy: *mut jboolean,
) -> *mut jlong {
	unimplemented!("jni::GetLongArrayElements")
}

extern "system" fn GetFloatArrayElements(
	env: *mut JNIEnv,
	array: jfloatArray,
	isCopy: *mut jboolean,
) -> *mut jfloat {
	unimplemented!("jni::GetFloatArrayElements")
}

extern "system" fn GetDoubleArrayElements(
	env: *mut JNIEnv,
	array: jdoubleArray,
	isCopy: *mut jboolean,
) -> *mut jdouble {
	unimplemented!("jni::GetDoubleArrayElements")
}

extern "system" fn ReleaseBooleanArrayElements(
	env: *mut JNIEnv,
	array: jbooleanArray,
	elems: *mut jboolean,
	mode: jint,
) {
	unimplemented!("jni::ReleaseBooleanArrayElements")
}

extern "system" fn ReleaseByteArrayElements(
	env: *mut JNIEnv,
	array: jbyteArray,
	elems: *mut jbyte,
	mode: jint,
) {
	unimplemented!("jni::ReleaseByteArrayElements")
}

extern "system" fn ReleaseCharArrayElements(
	env: *mut JNIEnv,
	array: jcharArray,
	elems: *mut jchar,
	mode: jint,
) {
	unimplemented!("jni::ReleaseCharArrayElements")
}

extern "system" fn ReleaseShortArrayElements(
	env: *mut JNIEnv,
	array: jshortArray,
	elems: *mut jshort,
	mode: jint,
) {
	unimplemented!("jni::ReleaseShortArrayElements")
}

pub extern "system" fn ReleaseIntArrayElements(
	env: *mut JNIEnv,
	array: jintArray,
	elems: *mut jint,
	mode: jint,
) {
	unimplemented!("jni::ReleaseIntArrayElements");
}

extern "system" fn ReleaseLongArrayElements(
	env: *mut JNIEnv,
	array: jlongArray,
	elems: *mut jlong,
	mode: jint,
) {
	unimplemented!("jni::ReleaseLongArrayElements")
}

extern "system" fn ReleaseFloatArrayElements(
	env: *mut JNIEnv,
	array: jfloatArray,
	elems: *mut jfloat,
	mode: jint,
) {
	unimplemented!("jni::ReleaseFloatArrayElements")
}

extern "system" fn ReleaseDoubleArrayElements(
	env: *mut JNIEnv,
	array: jdoubleArray,
	elems: *mut jdouble,
	mode: jint,
) {
	unimplemented!("jni::ReleaseDoubleArrayElements")
}

extern "system" fn GetBooleanArrayRegion(
	env: *mut JNIEnv,
	array: jbooleanArray,
	start: jsize,
	l: jsize,
	buf: *mut jboolean,
) {
	unimplemented!("jni::GetBooleanArrayRegion")
}

extern "system" fn GetByteArrayRegion(
	env: *mut JNIEnv,
	array: jbyteArray,
	start: jsize,
	len: jsize,
	buf: *mut jbyte,
) {
	unimplemented!("jni::GetByteArrayRegion")
}

extern "system" fn GetCharArrayRegion(
	env: *mut JNIEnv,
	array: jcharArray,
	start: jsize,
	len: jsize,
	buf: *mut jchar,
) {
	unimplemented!("jni::GetCharArrayRegion")
}

extern "system" fn GetShortArrayRegion(
	env: *mut JNIEnv,
	array: jshortArray,
	start: jsize,
	len: jsize,
	buf: *mut jshort,
) {
	unimplemented!("jni::GetShortArrayRegion")
}

extern "system" fn GetIntArrayRegion(
	env: *mut JNIEnv,
	array: jintArray,
	start: jsize,
	len: jsize,
	buf: *mut jint,
) {
	unimplemented!("jni::GetIntArrayRegion")
}

extern "system" fn GetLongArrayRegion(
	env: *mut JNIEnv,
	array: jlongArray,
	start: jsize,
	len: jsize,
	buf: *mut jlong,
) {
	unimplemented!("jni::GetLongArrayRegion")
}

extern "system" fn GetFloatArrayRegion(
	env: *mut JNIEnv,
	array: jfloatArray,
	start: jsize,
	len: jsize,
	buf: *mut jfloat,
) {
	unimplemented!("jni::GetFloatArrayRegion")
}

extern "system" fn GetDoubleArrayRegion(
	env: *mut JNIEnv,
	array: jdoubleArray,
	start: jsize,
	len: jsize,
	buf: *mut jdouble,
) {
	unimplemented!("jni::GetDoubleArrayRegion")
}

extern "system" fn SetBooleanArrayRegion(
	env: *mut JNIEnv,
	array: jbooleanArray,
	start: jsize,
	l: jsize,
	buf: *const jboolean,
) {
	unimplemented!("jni::SetBooleanArrayRegion")
}

extern "system" fn SetByteArrayRegion(
	env: *mut JNIEnv,
	array: jbyteArray,
	start: jsize,
	len: jsize,
	buf: *const jbyte,
) {
	unimplemented!("jni::SetByteArrayRegion")
}

extern "system" fn SetCharArrayRegion(
	env: *mut JNIEnv,
	array: jcharArray,
	start: jsize,
	len: jsize,
	buf: *const jchar,
) {
	unimplemented!("jni::SetCharArrayRegion")
}

extern "system" fn SetShortArrayRegion(
	env: *mut JNIEnv,
	array: jshortArray,
	start: jsize,
	len: jsize,
	buf: *const jshort,
) {
	unimplemented!("jni::SetShortArrayRegion")
}

extern "system" fn SetIntArrayRegion(
	env: *mut JNIEnv,
	array: jintArray,
	start: jsize,
	len: jsize,
	buf: *const jint,
) {
	unimplemented!("jni::SetIntArrayRegion")
}

extern "system" fn SetLongArrayRegion(
	env: *mut JNIEnv,
	array: jlongArray,
	start: jsize,
	len: jsize,
	buf: *const jlong,
) {
	unimplemented!("jni::SetLongArrayRegion")
}

extern "system" fn SetFloatArrayRegion(
	env: *mut JNIEnv,
	array: jfloatArray,
	start: jsize,
	len: jsize,
	buf: *const jfloat,
) {
	unimplemented!("jni::SetFloatArrayRegion")
}

extern "system" fn SetDoubleArrayRegion(
	env: *mut JNIEnv,
	array: jdoubleArray,
	start: jsize,
	len: jsize,
	buf: *const jdouble,
) {
	unimplemented!("jni::SetDoubleArrayRegion")
}

extern "system" fn RegisterNatives(
	env: *mut JNIEnv,
	clazz: jclass,
	methods: *const JNINativeMethod,
	nMethods: jint,
) -> jint {
	unimplemented!("jni::RegisterNatives")
}

pub extern "system" fn UnregisterNatives(env: *mut JNIEnv, clazz: jclass) -> jint {
	unimplemented!("jni::UnregisterNatives");
}

pub extern "system" fn MonitorEnter(env: *mut JNIEnv, obj: jobject) -> jint {
	unimplemented!("jni::MonitorEnter");
}

pub extern "system" fn MonitorExit(env: *mut JNIEnv, obj: jobject) -> jint {
	unimplemented!("jni::MonitorExit");
}

pub extern "system" fn GetJavaVM(env: *mut JNIEnv, vm: *mut *mut JavaVM) -> jint {
	unimplemented!("jni::GetJavaVM");
}

extern "system" fn GetStringRegion(
	env: *mut JNIEnv,
	str: jstring,
	start: jsize,
	len: jsize,
	buf: *mut jchar,
) {
	unimplemented!("jni::GetStringRegion")
}

extern "system" fn GetStringUTFRegion(
	env: *mut JNIEnv,
	str: jstring,
	start: jsize,
	len: jsize,
	buf: *mut c_char,
) {
	unimplemented!("jni::GetStringUTFRegion")
}

extern "system" fn GetPrimitiveArrayCritical(
	env: *mut JNIEnv,
	array: jarray,
	isCopy: *mut jboolean,
) -> *mut c_void {
	unimplemented!("jni::GetPrimitiveArrayCritical")
}

pub extern "system" fn ReleasePrimitiveArrayCritical(
	env: *mut JNIEnv,
	array: jarray,
	carray: *mut c_void,
	mode: jint,
) {
	unimplemented!("jni::ReleasePrimitiveArrayCritical");
}

extern "system" fn GetStringCritical(
	env: *mut JNIEnv,
	string: jstring,
	isCopy: *mut jboolean,
) -> *const jchar {
	unimplemented!("jni::GetStringCritical")
}

pub extern "system" fn ReleaseStringCritical(
	env: *mut JNIEnv,
	string: jstring,
	cstring: *const jchar,
) {
	unimplemented!("jni::ReleaseStringCritical");
}

pub extern "system" fn NewWeakGlobalRef(env: *mut JNIEnv, obj: jobject) -> jweak {
	unimplemented!("jni::NewWeakGlobalRef");
}

pub extern "system" fn DeleteWeakGlobalRef(env: *mut JNIEnv, ref_: jweak) {
	unimplemented!("jni::DeleteWeakGlobalRef");
}

pub extern "system" fn ExceptionCheck(env: *mut JNIEnv) -> jboolean {
	unimplemented!("jni::ExceptionCheck");
}

extern "system" fn NewDirectByteBuffer(
	env: *mut JNIEnv,
	address: *mut c_void,
	capacity: jlong,
) -> jobject {
	unimplemented!("jni::NewDirectByteBuffer")
}

pub extern "system" fn GetDirectBufferAddress(env: *mut JNIEnv, buf: jobject) -> *mut c_void {
	unimplemented!("jni::GetDirectBufferAddress");
}

pub extern "system" fn GetDirectBufferCapacity(env: *mut JNIEnv, buf: jobject) -> jlong {
	unimplemented!("jni::GetDirectBufferCapacity");
}

pub extern "system" fn GetObjectRefType(env: *mut JNIEnv, obj: jobject) -> jobjectRefType {
	unimplemented!("jni::GetObjectRefType");
}
