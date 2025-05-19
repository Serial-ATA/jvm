use super::{IntoJni, method_ref_from_jmethodid, reference_from_jobject};
use crate::objects::method::Method;
use crate::stack::local_stack::LocalStack;
use crate::symbols::Symbol;
use crate::thread::JavaThread;
use crate::thread::exceptions::Throws;
use crate::objects::reference::Reference;

use core::ffi::c_char;
use instructions::Operand;
use jni::sys::{
	JNIEnv, jboolean, jbyte, jchar, jclass, jdouble, jfloat, jint, jlong, jmethodID, jobject,
	jshort, jvalue, va_list,
};
use std::ffi::CStr;
// --------------
//   NON-STATIC
// --------------

#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetMethodID(
	env: *mut JNIEnv,
	clazz: jclass,
	name: *const c_char,
	sig: *const c_char,
) -> jmethodID {
	unimplemented!("jni::GetMethodID")
}

#[unsafe(no_mangle)]
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

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallObjectMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) -> jobject {
	unimplemented!("jni::CallObjectMethodA")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallBooleanMethod(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	...
) -> jboolean {
	unimplemented!("jni::CallBooleanMethod");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallBooleanMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: va_list,
) -> jboolean {
	unimplemented!("jni::CallBooleanMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallBooleanMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) -> jboolean {
	unimplemented!("jni::CallBooleanMethodA")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallByteMethod(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	...
) -> jbyte {
	unimplemented!("jni::CallByteMethod");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallByteMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: va_list,
) -> jbyte {
	unimplemented!("jni::CallByteMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallByteMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) -> jbyte {
	unimplemented!("jni::CallByteMethodA")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallCharMethod(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	...
) -> jchar {
	unimplemented!("jni::CallCharMethod");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallCharMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: va_list,
) -> jchar {
	unimplemented!("jni::CallCharMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallCharMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) -> jchar {
	unimplemented!("jni::CallCharMethodA")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallShortMethod(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	...
) -> jshort {
	unimplemented!("jni::CallShortMethod");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallShortMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: va_list,
) -> jshort {
	unimplemented!("jni::CallShortMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallShortMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) -> jshort {
	unimplemented!("jni::CallShortMethodA")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallIntMethod(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	...
) -> jint {
	unimplemented!("jni::CallIntMethod");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallIntMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: va_list,
) -> jint {
	unimplemented!("jni::CallIntMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallIntMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) -> jint {
	unimplemented!("jni::CallIntMethodA")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallLongMethod(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	...
) -> jlong {
	unimplemented!("jni::CallLongMethod");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallLongMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: va_list,
) -> jlong {
	unimplemented!("jni::CallLongMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallLongMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) -> jlong {
	unimplemented!("jni::CallLongMethodA")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallFloatMethod(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	...
) -> jfloat {
	unimplemented!("jni::CallFloatMethod");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallFloatMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: va_list,
) -> jfloat {
	unimplemented!("jni::CallFloatMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallFloatMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) -> jfloat {
	unimplemented!("jni::CallFloatMethodA")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallDoubleMethod(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	...
) -> jdouble {
	unimplemented!("jni::CallDoubleMethod");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallDoubleMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: va_list,
) -> jdouble {
	unimplemented!("jni::CallDoubleMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallDoubleMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) -> jdouble {
	unimplemented!("jni::CallDoubleMethodA")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallVoidMethod(env: *mut JNIEnv, obj: jobject, methodID: jmethodID, ...) {
	unimplemented!("jni::CallVoidMethod");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallVoidMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: va_list,
) {
	unimplemented!("jni::CallVoidMethodV")
}

#[unsafe(no_mangle)]
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallNonvirtualObjectMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jobject {
	unimplemented!("jni::CallNonvirtualObjectMethod")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallNonvirtualObjectMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jobject {
	unimplemented!("jni::CallNonvirtualObjectMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallNonvirtualObjectMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jobject {
	unimplemented!("jni::CallNonvirtualObjectMethodA")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallNonvirtualBooleanMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jboolean {
	unimplemented!("jni::CallNonvirtualBooleanMethod")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallNonvirtualBooleanMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jboolean {
	unimplemented!("jni::CallNonvirtualBooleanMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallNonvirtualBooleanMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jboolean {
	unimplemented!("jni::CallNonvirtualBooleanMethodA")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallNonvirtualByteMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jbyte {
	unimplemented!("jni::CallNonvirtualByteMethod")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallNonvirtualByteMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jbyte {
	unimplemented!("jni::CallNonvirtualByteMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallNonvirtualByteMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jbyte {
	unimplemented!("jni::CallNonvirtualByteMethodA")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallNonvirtualCharMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jchar {
	unimplemented!("jni::CallNonvirtualCharMethod")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallNonvirtualCharMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jchar {
	unimplemented!("jni::CallNonvirtualCharMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallNonvirtualCharMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jchar {
	unimplemented!("jni::CallNonvirtualCharMethodA")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallNonvirtualShortMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jshort {
	unimplemented!("jni::CallNonvirtualShortMethod")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallNonvirtualShortMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jshort {
	unimplemented!("jni::CallNonvirtualShortMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallNonvirtualShortMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jshort {
	unimplemented!("jni::CallNonvirtualShortMethodA")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallNonvirtualIntMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jint {
	unimplemented!("jni::CallNonvirtualIntMethod")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallNonvirtualIntMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jint {
	unimplemented!("jni::CallNonvirtualIntMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallNonvirtualIntMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jint {
	unimplemented!("jni::CallNonvirtualIntMethodA")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallNonvirtualLongMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jlong {
	unimplemented!("jni::CallNonvirtualLongMethod")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallNonvirtualLongMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jlong {
	unimplemented!("jni::CallNonvirtualLongMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallNonvirtualLongMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jlong {
	unimplemented!("jni::CallNonvirtualLongMethodA")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallNonvirtualFloatMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jfloat {
	unimplemented!("jni::CallNonvirtualFloatMethod")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallNonvirtualFloatMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jfloat {
	unimplemented!("jni::CallNonvirtualFloatMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallNonvirtualFloatMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jfloat {
	unimplemented!("jni::CallNonvirtualFloatMethodA")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallNonvirtualDoubleMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jdouble {
	unimplemented!("jni::CallNonvirtualDoubleMethod")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallNonvirtualDoubleMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jdouble {
	unimplemented!("jni::CallNonvirtualDoubleMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallNonvirtualDoubleMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jdouble {
	unimplemented!("jni::CallNonvirtualDoubleMethodA")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallNonvirtualVoidMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	...
) {
	unimplemented!("jni::CallNonvirtualVoidMethod")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallNonvirtualVoidMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) {
	unimplemented!("jni::CallNonvirtualVoidMethodV")
}

#[unsafe(no_mangle)]
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

#[unsafe(no_mangle)]
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

	let Some(class_obj) = (unsafe { reference_from_jobject(clazz) }) else {
		return core::ptr::null::<Method>() as jmethodID;
	};

	let class = class_obj.extract_target_class();
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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallStaticObjectMethod(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jobject {
	unimplemented!("jni::CallStaticObjectMethod");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallStaticObjectMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jobject {
	unimplemented!("jni::CallStaticObjectMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallStaticObjectMethodA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jobject {
	let Some(ret) = call_with_c_array_args(env, clazz, methodID, args) else {
		return Default::default();
	};

	unsafe { ret.into_jni().l }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallStaticBooleanMethod(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jboolean {
	unimplemented!("jni::CallStaticBooleanMethod");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallStaticBooleanMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jboolean {
	unimplemented!("jni::CallStaticBooleanMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallStaticBooleanMethodA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jboolean {
	let Some(ret) = call_with_c_array_args(env, clazz, methodID, args) else {
		return Default::default();
	};

	unsafe { ret.into_jni().z }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallStaticByteMethod(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jbyte {
	unimplemented!("jni::CallStaticByteMethod");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallStaticByteMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jbyte {
	unimplemented!("jni::CallStaticByteMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallStaticByteMethodA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jbyte {
	let Some(ret) = call_with_c_array_args(env, clazz, methodID, args) else {
		return Default::default();
	};

	unsafe { ret.into_jni().b }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallStaticCharMethod(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jchar {
	unimplemented!("jni::CallStaticCharMethod");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallStaticCharMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jchar {
	unimplemented!("jni::CallStaticCharMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallStaticCharMethodA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jchar {
	let Some(ret) = call_with_c_array_args(env, clazz, methodID, args) else {
		return Default::default();
	};

	unsafe { ret.into_jni().c }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallStaticShortMethod(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jshort {
	unimplemented!("jni::CallStaticShortMethod");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallStaticShortMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jshort {
	unimplemented!("jni::CallStaticShortMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallStaticShortMethodA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jshort {
	let Some(ret) = call_with_c_array_args(env, clazz, methodID, args) else {
		return Default::default();
	};

	unsafe { ret.into_jni().s }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallStaticIntMethod(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jint {
	unimplemented!("jni::CallStaticIntMethod");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallStaticIntMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jint {
	unimplemented!("jni::CallStaticIntMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallStaticIntMethodA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jint {
	let Some(ret) = call_with_c_array_args(env, clazz, methodID, args) else {
		return Default::default();
	};

	unsafe { ret.into_jni().i }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallStaticLongMethod(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jlong {
	unimplemented!("jni::CallStaticLongMethod");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallStaticLongMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jlong {
	unimplemented!("jni::CallStaticLongMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallStaticLongMethodA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jlong {
	let Some(ret) = call_with_c_array_args(env, clazz, methodID, args) else {
		return Default::default();
	};

	unsafe { ret.into_jni().j }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallStaticFloatMethod(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jfloat {
	unimplemented!("jni::CallStaticFloatMethod");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallStaticFloatMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jfloat {
	unimplemented!("jni::CallStaticFloatMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallStaticFloatMethodA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jfloat {
	let Some(ret) = call_with_c_array_args(env, clazz, methodID, args) else {
		return Default::default();
	};

	unsafe { ret.into_jni().f }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallStaticDoubleMethod(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jdouble {
	unimplemented!("jni::CallStaticDoubleMethod");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallStaticDoubleMethodV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jdouble {
	unimplemented!("jni::CallStaticDoubleMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallStaticDoubleMethodA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jdouble {
	let Some(ret) = call_with_c_array_args(env, clazz, methodID, args) else {
		return Default::default();
	};

	unsafe { ret.into_jni().d }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallStaticVoidMethod(
	env: *mut JNIEnv,
	cls: jclass,
	methodID: jmethodID,
	args: ...
) {
	unimplemented!("jni::CallStaticVoidMethod")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallStaticVoidMethodV(
	env: *mut JNIEnv,
	cls: jclass,
	methodID: jmethodID,
	args: va_list,
) {
	unimplemented!("jni::CallStaticVoidMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallStaticVoidMethodA(
	env: *mut JNIEnv,
	cls: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) {
	call_with_c_array_args(env, cls, methodID, args);
}

fn call_with_c_array_args(
	env: *mut JNIEnv,
	cls: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> Option<Operand<Reference>> {
	let thread = JavaThread::current();
	assert_eq!(thread.env().raw(), env);

	let class_obj = unsafe { reference_from_jobject(cls) };
	let Some(class_obj) = class_obj else {
		return None; // TODO: Exception?
	};

	let class = class_obj.extract_target_class();

	let method = unsafe { method_ref_from_jmethodid(methodID) };
	let Some(method) = method else {
		return None; // TODO: Exception?
	};

	let Some(arguments) = (unsafe { method.args_for_c_array(args) }) else {
		return None; // TODO: Exception?
	};

	// SAFETY: `Method::args_for_c_args` ensures that the arguments are constructed correctly
	let call_args =
		unsafe { LocalStack::new_with_args(arguments, method.code.max_locals as usize) };
	thread.invoke_method_scoped(method, call_args)
}
