use core::ffi::{c_char, VaList};
use jni::sys::{
	jboolean, jbyte, jchar, jclass, jdouble, jfloat, jint, jlong, jmethodID, jobject, jshort,
	jvalue, JNIEnv,
};

// --------------
//   NON-STATIC
// --------------

pub extern "system" fn GetMethodID(
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

pub extern "system" fn CallObjectMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jobject {
	unimplemented!("jni::CallObjectMethodV")
}

pub extern "system" fn CallObjectMethodA(
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

pub extern "system" fn CallBooleanMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jboolean {
	unimplemented!("jni::CallBooleanMethodV")
}

pub extern "system" fn CallBooleanMethodA(
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

pub extern "system" fn CallByteMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jbyte {
	unimplemented!("jni::CallByteMethodV")
}

pub extern "system" fn CallByteMethodA(
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

pub extern "system" fn CallCharMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jchar {
	unimplemented!("jni::CallCharMethodV")
}

pub extern "system" fn CallCharMethodA(
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

pub extern "system" fn CallShortMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jshort {
	unimplemented!("jni::CallShortMethodV")
}

pub extern "system" fn CallShortMethodA(
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

pub extern "system" fn CallIntMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jint {
	unimplemented!("jni::CallIntMethodV")
}

pub extern "system" fn CallIntMethodA(
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

pub extern "system" fn CallLongMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jlong {
	unimplemented!("jni::CallLongMethodV")
}

pub extern "system" fn CallLongMethodA(
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

pub extern "system" fn CallFloatMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jfloat {
	unimplemented!("jni::CallFloatMethodV")
}

pub extern "system" fn CallFloatMethodA(
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

pub extern "system" fn CallDoubleMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jdouble {
	unimplemented!("jni::CallDoubleMethodV")
}

pub extern "system" fn CallDoubleMethodA(
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

pub extern "system" fn CallVoidMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) {
	unimplemented!("jni::CallVoidMethodV")
}

pub extern "system" fn CallVoidMethodA(
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

pub extern "system" fn CallNonvirtualObjectMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jobject {
	unimplemented!("jni::CallNonvirtualObjectMethodV")
}

pub extern "system" fn CallNonvirtualObjectMethodA(
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

pub extern "system" fn CallNonvirtualBooleanMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jboolean {
	unimplemented!("jni::CallNonvirtualBooleanMethodV")
}

pub extern "system" fn CallNonvirtualBooleanMethodA(
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

pub extern "system" fn CallNonvirtualByteMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jbyte {
	unimplemented!("jni::CallNonvirtualByteMethodV")
}

pub extern "system" fn CallNonvirtualByteMethodA(
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

pub extern "system" fn CallNonvirtualCharMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jchar {
	unimplemented!("jni::CallNonvirtualCharMethodV")
}

pub extern "system" fn CallNonvirtualCharMethodA(
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

pub extern "system" fn CallNonvirtualShortMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jshort {
	unimplemented!("jni::CallNonvirtualShortMethodV")
}

pub extern "system" fn CallNonvirtualShortMethodA(
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

pub extern "system" fn CallNonvirtualIntMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jint {
	unimplemented!("jni::CallNonvirtualIntMethodV")
}

pub extern "system" fn CallNonvirtualIntMethodA(
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

pub extern "system" fn CallNonvirtualLongMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jlong {
	unimplemented!("jni::CallNonvirtualLongMethodV")
}

pub extern "system" fn CallNonvirtualLongMethodA(
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

pub extern "system" fn CallNonvirtualFloatMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jfloat {
	unimplemented!("jni::CallNonvirtualFloatMethodV")
}

pub extern "system" fn CallNonvirtualFloatMethodA(
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

pub extern "system" fn CallNonvirtualDoubleMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jdouble {
	unimplemented!("jni::CallNonvirtualDoubleMethodV")
}

pub extern "system" fn CallNonvirtualDoubleMethodA(
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

pub extern "system" fn CallNonvirtualVoidMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) {
	unimplemented!("jni::CallNonvirtualVoidMethodV")
}

pub extern "system" fn CallNonvirtualVoidMethodA(
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

pub extern "system" fn GetStaticMethodID(
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

pub extern "system" fn CallStaticObjectMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jobject {
	unimplemented!("jni::CallStaticObjectMethodV")
}

pub extern "system" fn CallStaticObjectMethodA(
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

pub extern "system" fn CallStaticBooleanMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jboolean {
	unimplemented!("jni::CallStaticBooleanMethodV")
}

pub extern "system" fn CallStaticBooleanMethodA(
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

pub extern "system" fn CallStaticByteMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jbyte {
	unimplemented!("jni::CallStaticByteMethodV")
}

pub extern "system" fn CallStaticByteMethodA(
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

pub extern "system" fn CallStaticCharMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jchar {
	unimplemented!("jni::CallStaticCharMethodV")
}

pub extern "system" fn CallStaticCharMethodA(
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

pub extern "system" fn CallStaticShortMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jshort {
	unimplemented!("jni::CallStaticShortMethodV")
}

pub extern "system" fn CallStaticShortMethodA(
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

pub extern "system" fn CallStaticIntMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jint {
	unimplemented!("jni::CallStaticIntMethodV")
}

pub extern "system" fn CallStaticIntMethodA(
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

pub extern "system" fn CallStaticLongMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jlong {
	unimplemented!("jni::CallStaticLongMethodV")
}

pub extern "system" fn CallStaticLongMethodA(
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

pub extern "system" fn CallStaticFloatMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jfloat {
	unimplemented!("jni::CallStaticFloatMethodV")
}

pub extern "system" fn CallStaticFloatMethodA(
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

pub extern "system" fn CallStaticDoubleMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jdouble {
	unimplemented!("jni::CallStaticDoubleMethodV")
}

pub extern "system" fn CallStaticDoubleMethodA(
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

pub extern "system" fn CallStaticVoidMethodV(
	env: *mut JNIEnv,
	cls: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) {
	unimplemented!("jni::CallStaticVoidMethodV")
}

pub extern "system" fn CallStaticVoidMethodA(
	env: *mut JNIEnv,
	cls: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) {
	unimplemented!("jni::CallStaticVoidMethodA")
}
