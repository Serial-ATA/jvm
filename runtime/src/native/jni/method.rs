use super::{classref_from_jclass, method_ref_from_jmethodid, IntoJni};
use crate::objects::method::Method;
use crate::stack::local_stack::LocalStack;
use crate::symbols::Symbol;
use crate::thread::exceptions::Throws;
use crate::thread::JavaThread;

use core::ffi::c_char;
use std::ffi::CStr;

use jni::sys::{
	jboolean, jbyte, jchar, jclass, jdouble, jfloat, jint, jlong, jmethodID, jobject, jshort,
	jvalue, va_list, JNIEnv,
};

// --------------
//   NON-STATIC
// --------------

#[no_mangle]
pub unsafe extern "system" fn GetMethodID(
	env: *mut JNIEnv,
	clazz: jclass,
	name: *const c_char,
	sig: *const c_char,
) -> jmethodID {
	unimplemented!("jni::GetMethodID")
}

#[no_mangle]
pub unsafe extern "C" fn CallObjectMethod(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	...
) -> jobject {
	unimplemented!("jni::CallObjectMethod");
}

pub unsafe extern "system" fn CallObjectMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: va_list,
) -> jobject {
	unimplemented!("jni::CallObjectMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallObjectMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) -> jobject {
	unimplemented!("jni::CallObjectMethodA")
}

#[no_mangle]
pub unsafe extern "C" fn CallBooleanMethod(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	...
) -> jboolean {
	unimplemented!("jni::CallBooleanMethod");
}

#[no_mangle]
pub unsafe extern "system" fn CallBooleanMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: va_list,
) -> jboolean {
	unimplemented!("jni::CallBooleanMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallBooleanMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) -> jboolean {
	unimplemented!("jni::CallBooleanMethodA")
}

#[no_mangle]
pub unsafe extern "C" fn CallByteMethod(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	...
) -> jbyte {
	unimplemented!("jni::CallByteMethod");
}

#[no_mangle]
pub unsafe extern "system" fn CallByteMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: va_list,
) -> jbyte {
	unimplemented!("jni::CallByteMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallByteMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) -> jbyte {
	unimplemented!("jni::CallByteMethodA")
}

#[no_mangle]
pub unsafe extern "C" fn CallCharMethod(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	...
) -> jchar {
	unimplemented!("jni::CallCharMethod");
}

#[no_mangle]
pub unsafe extern "system" fn CallCharMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: va_list,
) -> jchar {
	unimplemented!("jni::CallCharMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallCharMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) -> jchar {
	unimplemented!("jni::CallCharMethodA")
}

#[no_mangle]
pub unsafe extern "C" fn CallShortMethod(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	...
) -> jshort {
	unimplemented!("jni::CallShortMethod");
}

#[no_mangle]
pub unsafe extern "system" fn CallShortMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: va_list,
) -> jshort {
	unimplemented!("jni::CallShortMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallShortMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) -> jshort {
	unimplemented!("jni::CallShortMethodA")
}

#[no_mangle]
pub unsafe extern "C" fn CallIntMethod(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	...
) -> jint {
	unimplemented!("jni::CallIntMethod");
}

#[no_mangle]
pub unsafe extern "system" fn CallIntMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: va_list,
) -> jint {
	unimplemented!("jni::CallIntMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallIntMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) -> jint {
	unimplemented!("jni::CallIntMethodA")
}

#[no_mangle]
pub unsafe extern "C" fn CallLongMethod(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	...
) -> jlong {
	unimplemented!("jni::CallLongMethod");
}

#[no_mangle]
pub unsafe extern "system" fn CallLongMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: va_list,
) -> jlong {
	unimplemented!("jni::CallLongMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallLongMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) -> jlong {
	unimplemented!("jni::CallLongMethodA")
}

#[no_mangle]
pub unsafe extern "C" fn CallFloatMethod(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	...
) -> jfloat {
	unimplemented!("jni::CallFloatMethod");
}

#[no_mangle]
pub unsafe extern "system" fn CallFloatMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: va_list,
) -> jfloat {
	unimplemented!("jni::CallFloatMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallFloatMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) -> jfloat {
	unimplemented!("jni::CallFloatMethodA")
}

#[no_mangle]
pub unsafe extern "C" fn CallDoubleMethod(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	...
) -> jdouble {
	unimplemented!("jni::CallDoubleMethod");
}

#[no_mangle]
pub unsafe extern "system" fn CallDoubleMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: va_list,
) -> jdouble {
	unimplemented!("jni::CallDoubleMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallDoubleMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) -> jdouble {
	unimplemented!("jni::CallDoubleMethodA")
}

#[no_mangle]
pub unsafe extern "C" fn CallVoidMethod(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) {
	unimplemented!("jni::CallVoidMethod");
}

#[no_mangle]
pub unsafe extern "system" fn CallVoidMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: va_list,
) {
	unimplemented!("jni::CallVoidMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallVoidMethodA(
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

#[no_mangle]
pub unsafe extern "C" fn CallNonvirtualObjectMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jobject {
	unimplemented!("jni::CallNonvirtualObjectMethod")
}

#[no_mangle]
pub unsafe extern "system" fn CallNonvirtualObjectMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jobject {
	unimplemented!("jni::CallNonvirtualObjectMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallNonvirtualObjectMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jobject {
	unimplemented!("jni::CallNonvirtualObjectMethodA")
}

#[no_mangle]
pub unsafe extern "C" fn CallNonvirtualBooleanMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jboolean {
	unimplemented!("jni::CallNonvirtualBooleanMethod")
}

#[no_mangle]
pub unsafe extern "system" fn CallNonvirtualBooleanMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jboolean {
	unimplemented!("jni::CallNonvirtualBooleanMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallNonvirtualBooleanMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jboolean {
	unimplemented!("jni::CallNonvirtualBooleanMethodA")
}

#[no_mangle]
pub unsafe extern "C" fn CallNonvirtualByteMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jbyte {
	unimplemented!("jni::CallNonvirtualByteMethod")
}

#[no_mangle]
pub unsafe extern "system" fn CallNonvirtualByteMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jbyte {
	unimplemented!("jni::CallNonvirtualByteMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallNonvirtualByteMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jbyte {
	unimplemented!("jni::CallNonvirtualByteMethodA")
}

#[no_mangle]
pub unsafe extern "C" fn CallNonvirtualCharMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jchar {
	unimplemented!("jni::CallNonvirtualCharMethod")
}

#[no_mangle]
pub unsafe extern "system" fn CallNonvirtualCharMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jchar {
	unimplemented!("jni::CallNonvirtualCharMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallNonvirtualCharMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jchar {
	unimplemented!("jni::CallNonvirtualCharMethodA")
}

#[no_mangle]
pub unsafe extern "C" fn CallNonvirtualShortMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jshort {
	unimplemented!("jni::CallNonvirtualShortMethod")
}

#[no_mangle]
pub unsafe extern "system" fn CallNonvirtualShortMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jshort {
	unimplemented!("jni::CallNonvirtualShortMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallNonvirtualShortMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jshort {
	unimplemented!("jni::CallNonvirtualShortMethodA")
}

#[no_mangle]
pub unsafe extern "C" fn CallNonvirtualIntMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jint {
	unimplemented!("jni::CallNonvirtualIntMethod")
}

#[no_mangle]
pub unsafe extern "system" fn CallNonvirtualIntMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jint {
	unimplemented!("jni::CallNonvirtualIntMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallNonvirtualIntMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jint {
	unimplemented!("jni::CallNonvirtualIntMethodA")
}

#[no_mangle]
pub unsafe extern "C" fn CallNonvirtualLongMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jlong {
	unimplemented!("jni::CallNonvirtualLongMethod")
}

#[no_mangle]
pub unsafe extern "system" fn CallNonvirtualLongMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jlong {
	unimplemented!("jni::CallNonvirtualLongMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallNonvirtualLongMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jlong {
	unimplemented!("jni::CallNonvirtualLongMethodA")
}

#[no_mangle]
pub unsafe extern "C" fn CallNonvirtualFloatMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jfloat {
	unimplemented!("jni::CallNonvirtualFloatMethod")
}

#[no_mangle]
pub unsafe extern "system" fn CallNonvirtualFloatMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jfloat {
	unimplemented!("jni::CallNonvirtualFloatMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallNonvirtualFloatMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jfloat {
	unimplemented!("jni::CallNonvirtualFloatMethodA")
}

#[no_mangle]
pub unsafe extern "C" fn CallNonvirtualDoubleMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jdouble {
	unimplemented!("jni::CallNonvirtualDoubleMethod")
}

#[no_mangle]
pub unsafe extern "system" fn CallNonvirtualDoubleMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jdouble {
	unimplemented!("jni::CallNonvirtualDoubleMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallNonvirtualDoubleMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jdouble {
	unimplemented!("jni::CallNonvirtualDoubleMethodA")
}

#[no_mangle]
pub unsafe extern "C" fn CallNonvirtualVoidMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) {
	unimplemented!("jni::CallNonvirtualVoidMethod")
}

#[no_mangle]
pub unsafe extern "system" fn CallNonvirtualVoidMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) {
	unimplemented!("jni::CallNonvirtualVoidMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallNonvirtualVoidMethodA(
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

#[no_mangle]
pub unsafe extern "system" fn GetStaticMethodID(
	env: *mut JNIEnv,
	clazz: jclass,
	name: *const c_char,
	sig: *const c_char,
) -> jmethodID {
	let name = unsafe { CStr::from_ptr(name) };
	let sig = unsafe { CStr::from_ptr(sig) };

	let name = Symbol::intern(name.to_bytes());
	let sig = Symbol::intern(sig.to_bytes());

	let Some(class) = classref_from_jclass(clazz) else {
		return core::ptr::null::<Method>() as jmethodID;
	};

	match class.resolve_method(name, sig) {
		Throws::Ok(method) => method.into_jni(),
		Throws::Exception(e) => {
			let thread = JavaThread::current();
			assert_eq!(thread.env().raw(), env);
			e.throw(thread);

			core::ptr::null::<Method>() as jmethodID
		},
	}
}

#[no_mangle]
pub unsafe extern "C" fn CallStaticObjectMethod(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jobject {
	unimplemented!("jni::CallStaticObjectMethod");
}

#[no_mangle]
pub unsafe extern "system" fn CallStaticObjectMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jobject {
	unimplemented!("jni::CallStaticObjectMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallStaticObjectMethodA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jobject {
	unimplemented!("jni::CallStaticObjectMethodA")
}

#[no_mangle]
pub unsafe extern "C" fn CallStaticBooleanMethod(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jboolean {
	unimplemented!("jni::CallStaticBooleanMethod");
}

#[no_mangle]
pub unsafe extern "system" fn CallStaticBooleanMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jboolean {
	unimplemented!("jni::CallStaticBooleanMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallStaticBooleanMethodA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jboolean {
	unimplemented!("jni::CallStaticBooleanMethodA")
}

#[no_mangle]
pub unsafe extern "C" fn CallStaticByteMethod(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jbyte {
	unimplemented!("jni::CallStaticByteMethod");
}

#[no_mangle]
pub unsafe extern "system" fn CallStaticByteMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jbyte {
	unimplemented!("jni::CallStaticByteMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallStaticByteMethodA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jbyte {
	unimplemented!("jni::CallStaticByteMethodA")
}

#[no_mangle]
pub unsafe extern "C" fn CallStaticCharMethod(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jchar {
	unimplemented!("jni::CallStaticCharMethod");
}

#[no_mangle]
pub unsafe extern "system" fn CallStaticCharMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jchar {
	unimplemented!("jni::CallStaticCharMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallStaticCharMethodA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jchar {
	unimplemented!("jni::CallStaticCharMethodA")
}

#[no_mangle]
pub unsafe extern "C" fn CallStaticShortMethod(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jshort {
	unimplemented!("jni::CallStaticShortMethod");
}

#[no_mangle]
pub unsafe extern "system" fn CallStaticShortMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jshort {
	unimplemented!("jni::CallStaticShortMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallStaticShortMethodA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jshort {
	unimplemented!("jni::CallStaticShortMethodA")
}

#[no_mangle]
pub unsafe extern "C" fn CallStaticIntMethod(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jint {
	unimplemented!("jni::CallStaticIntMethod");
}

#[no_mangle]
pub unsafe extern "system" fn CallStaticIntMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jint {
	unimplemented!("jni::CallStaticIntMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallStaticIntMethodA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jint {
	unimplemented!("jni::CallStaticIntMethodA")
}

#[no_mangle]
pub unsafe extern "C" fn CallStaticLongMethod(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jlong {
	unimplemented!("jni::CallStaticLongMethod");
}

#[no_mangle]
pub unsafe extern "system" fn CallStaticLongMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jlong {
	unimplemented!("jni::CallStaticLongMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallStaticLongMethodA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jlong {
	unimplemented!("jni::CallStaticLongMethodA")
}

#[no_mangle]
pub unsafe extern "C" fn CallStaticFloatMethod(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jfloat {
	unimplemented!("jni::CallStaticFloatMethod");
}

#[no_mangle]
pub unsafe extern "system" fn CallStaticFloatMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jfloat {
	unimplemented!("jni::CallStaticFloatMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallStaticFloatMethodA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jfloat {
	unimplemented!("jni::CallStaticFloatMethodA")
}

#[no_mangle]
pub unsafe extern "C" fn CallStaticDoubleMethod(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jdouble {
	unimplemented!("jni::CallStaticDoubleMethod");
}

#[no_mangle]
pub unsafe extern "system" fn CallStaticDoubleMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jdouble {
	unimplemented!("jni::CallStaticDoubleMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallStaticDoubleMethodA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jdouble {
	unimplemented!("jni::CallStaticDoubleMethodA")
}

#[no_mangle]
pub unsafe extern "C" fn CallStaticVoidMethod(
	env: *mut JNIEnv,
	cls: jclass,
	methodID: jmethodID,
	args: ...
) {
	unimplemented!("jni::CallStaticVoidMethod")
}

#[no_mangle]
pub unsafe extern "system" fn CallStaticVoidMethodV(
	env: *mut JNIEnv,
	cls: jclass,
	methodID: jmethodID,
	args: va_list,
) {
	unimplemented!("jni::CallStaticVoidMethodV")
}

#[no_mangle]
pub unsafe extern "system" fn CallStaticVoidMethodA(
	env: *mut JNIEnv,
	cls: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) {
	let thread = JavaThread::current();
	assert_eq!(thread.env().raw(), env);

	let class = unsafe { classref_from_jclass(cls) };
	let Some(class) = class else {
		return; // TODO: Exception?
	};

	let method = unsafe { method_ref_from_jmethodid(methodID) };
	let Some(method) = method else {
		return; // TODO: Exception?
	};

	let Some(arguments) = (unsafe { method.args_for_c_array(args) }) else {
		return; // TODO: Exception?
	};

	// SAFETY: `Method::args_for_c_args` ensures that the arguments are constructed correctly
	let call_args =
		unsafe { LocalStack::new_with_args(arguments, method.code.max_locals as usize) };
	thread.invoke_method_scoped(method, call_args);
}
