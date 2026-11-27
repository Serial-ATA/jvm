use super::convert_operand;
use super::references::{JObjectExt, method_ref_from_jmethodid};
use crate::objects::method::Method;
use crate::objects::reference::Reference;
use crate::symbols::Symbol;
use crate::thread::JavaThread;
use crate::thread::exceptions::{Throws, throw_with_ret};

use common::unicode;
use core::ffi::c_char;
use instructions::Operand;
use jni::sys::{
	JNIEnv, jboolean, jbyte, jchar, jclass, jdouble, jfloat, jint, jlong, jmethodID, jobject,
	jshort, jvalue,
};
use std::ffi::{CStr, VaList};
use std::ptr;

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
	let thread = JavaThread::current();
	assert_eq!(thread.env().raw(), env);

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

	let Some(class_obj) = (unsafe { clazz.to_reference() }) else {
		return core::ptr::null::<Method>() as jmethodID;
	};

	let class = class_obj.extract_target_class();
	match class.resolve_method(name_sym, sig_sym) {
		Throws::Ok(method) => thread.jni_refs().allocate(method),
		Throws::Exception(e) => {
			e.throw(thread);

			core::ptr::null::<Method>() as jmethodID
		},
	}
}

macro_rules! define_call_methods {
	($($ty:ident => ($jni_ty:ty, $ty_code:ident, $default:expr)),* $(,)?) => {
		paste::paste! {
        $(
			#[unsafe(no_mangle)]
			pub unsafe extern "C" fn [<Call $ty Method>](
				env: *mut JNIEnv,
				obj: jobject,
				methodID: jmethodID,
				args: ...
			) -> $jni_ty {
                unsafe { [<Call $ty MethodV>](env, obj, methodID, args) }
			}

			#[unsafe(no_mangle)]
			pub unsafe extern "system" fn [<Call $ty MethodV>](
				env: *mut JNIEnv,
				obj: jobject,
				methodID: jmethodID,
				args: VaList<'_>,
			) -> $jni_ty {
				let thread = JavaThread::current();
				assert_eq!(thread.env().raw(), env);

				let Some(receiver) = (unsafe { obj.to_reference() }) else {
					throw_with_ret!($default, thread, NullPointerException);
				};
				thread.stack().push_op(Operand::Reference(receiver));

				let Some(ret) =
					(unsafe { call_with_va_list_args(thread, methodID, args) })
				else {
					panic!("unexpected return type on stack");
				};

				unsafe { convert_operand(thread, ret).$ty_code }
			}

			#[unsafe(no_mangle)]
			pub unsafe extern "C" fn [<Call $ty MethodA>](
				env: *mut JNIEnv,
				obj: jobject,
				methodID: jmethodID,
				args: *const jvalue,
			) -> $jni_ty {
				unimplemented!("jni::{}", stringify!([<Call $ty MethodA>]));
			}

			// --------------
			//   NON-VIRTUAL
			// --------------
			#[unsafe(no_mangle)]
			pub unsafe extern "C" fn [<CallNonvirtual $ty Method>](
				env: *mut JNIEnv,
				obj: jobject,
				clazz: jclass,
				methodID: jmethodID,
				_: ...
			) -> $jni_ty {
				unimplemented!("jni::{}", stringify!([<CallNonvirtual $ty Method>]));
			}

			#[unsafe(no_mangle)]
			pub unsafe extern "system" fn [<CallNonvirtual $ty MethodV>](
				env: *mut JNIEnv,
				obj: jobject,
                clazz: jclass,
				methodID: jmethodID,
				args: VaList<'_>,
			) -> $jni_ty {
				unimplemented!("jni::{}", stringify!([<CallNonvirtual $ty MethodV>]));
			}

			#[unsafe(no_mangle)]
			pub unsafe extern "C" fn [<CallNonvirtual $ty MethodA>](
				env: *mut JNIEnv,
				obj: jobject,
                clazz: jclass,
				methodID: jmethodID,
				args: *const jvalue,
			) -> $jni_ty {
				unimplemented!("jni::{}", stringify!([<CallNonvirtual $ty MethodA>]));
			}

			// --------------
			//     STATIC
			// --------------

			#[unsafe(no_mangle)]
			pub unsafe extern "C" fn [<CallStatic $ty Method>](
				env: *mut JNIEnv,
				clazz: jclass,
				methodID: jmethodID,
				args: ...
			) -> $jni_ty {
				unsafe { [<CallStatic $ty MethodV>](env, clazz, methodID, args) }
			}

			#[unsafe(no_mangle)]
			pub unsafe extern "system" fn [<CallStatic $ty MethodV>](
				env: *mut JNIEnv,
				clazz: jclass,
				methodID: jmethodID,
				args: VaList<'_>,
			) -> $jni_ty {
				let thread = JavaThread::current();
				assert_eq!(thread.env().raw(), env);

				let Some(ret) = (unsafe { call_with_va_list_args(thread, methodID, args) }) else {
					return $default;
				};

				unsafe { convert_operand(thread, ret).$ty_code }
			}

			#[unsafe(no_mangle)]
			pub unsafe extern "system" fn [<CallStatic $ty MethodA>](
				env: *mut JNIEnv,
				clazz: jclass,
				methodID: jmethodID,
				args: *const jvalue,
			) -> $jni_ty {
				let thread = JavaThread::current();
				assert_eq!(thread.env().raw(), env);

				let Some(ret) = (unsafe { call_with_c_array_args(thread, clazz, methodID, args) }) else {
					return $default;
				};

				unsafe { convert_operand(thread, ret).$ty_code }
			}
        )*
		}
	};
}

define_call_methods! {
	Object  => (jobject,  l, ptr::null_mut()),
	Boolean => (jboolean, z, jboolean::default()),
	Byte    => (jbyte,    b, jbyte::default()),
	Char    => (jchar,    c, jchar::default()),
	Short   => (jshort,   s, jshort::default()),
	Int     => (jint,     i, jint::default()),
	Long    => (jlong,    j, jlong::default()),
	Float   => (jfloat,   f, jfloat::default()),
	Double  => (jdouble,  d, jdouble::default()),
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallVoidMethod(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	_: ...
) {
	unimplemented!("jni::CallVoidMethod");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallVoidMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: VaList<'_>,
) {
	unimplemented!("jni::CallVoidMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallVoidMethodA(
	env: *mut JNIEnv,
	obj: jobject,
	methodID: jmethodID,
	args: *const jvalue,
) {
	unimplemented!("jni::CallVoidMethodA")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallNonvirtualVoidMethod(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	_: ...
) {
	unimplemented!("jni::CallNonvirtualVoidMethod")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn CallNonvirtualVoidMethodV(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_>,
) {
	unimplemented!("jni::CallNonvirtualVoidMethodV")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn CallNonvirtualVoidMethodA(
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
	unsafe { GetMethodID(env, clazz, name, sig) }
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
	args: VaList<'_>,
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
	let thread = JavaThread::current();
	assert_eq!(thread.env().raw(), env);

	unsafe { call_with_c_array_args(thread, cls, methodID, args) };
}

pub(super) unsafe fn call_with_c_array_args(
	thread: &'static JavaThread,
	cls: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> Option<Operand<Reference>> {
	let class_obj = unsafe { cls.to_reference() };
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

	let stack = thread.stack();
	for arg in arguments {
		stack.push_op(arg);
	}

	thread.invoke_method_scoped(method)
}

pub(super) unsafe fn call_with_va_list_args(
	thread: &'static JavaThread,
	methodID: jmethodID,
	args: VaList<'_>,
) -> Option<Operand<Reference>> {
	let method = unsafe { method_ref_from_jmethodid(methodID) };
	let Some(method) = method else {
		return None; // TODO: Exception?
	};

	let Some(arguments) = (unsafe { method.args_for_va_list(args) }) else {
		return None; // TODO: Exception?
	};

	let stack = thread.stack();
	for arg in arguments {
		stack.push_op(arg);
	}

	thread.invoke_method_scoped(method)
}
