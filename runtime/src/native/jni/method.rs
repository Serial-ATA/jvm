use core::ffi::{c_char, VaList};
use jni::{
	jboolean, jbyte, jchar, jclass, jdouble, jfloat, jint, jlong, jmethodID, jobject, jshort,
	jvalue, JNIEnv,
};

// --------------
//   NON-STATIC
// --------------

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

// --------------
//   NON-VIRTUAL
// --------------

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

// --------------
//     STATIC
// --------------

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
