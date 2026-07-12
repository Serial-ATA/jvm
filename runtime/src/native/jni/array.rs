use super::{IntoJni, reference_from_jobject};
use crate::objects::instance::array::{
	Array, ObjectArrayInstance, PrimitiveArrayInstance, TypeCode,
};
use crate::objects::reference::Reference;
use crate::thread::JavaThread;
use crate::thread::exceptions::{Throws, throw};

use core::ffi::c_void;
use std::ptr;

use common::int_types::u1;
use jni::sys::{
	JNIEnv, jarray, jboolean, jbooleanArray, jbyte, jbyteArray, jchar, jcharArray, jclass, jdouble,
	jdoubleArray, jfloat, jfloatArray, jint, jintArray, jlong, jlongArray, jobject, jobjectArray,
	jshort, jshortArray, jsize,
};

#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetArrayLength(env: *mut JNIEnv, array: jarray) -> jsize {
	unimplemented!("jni::GetArrayLength");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn NewObjectArray(
	env: *mut JNIEnv,
	len: jsize,
	clazz: jclass,
	init: jobject,
) -> jobjectArray {
	let thread = JavaThread::current();
	assert_eq!(thread.env().raw(), env);

	let obj = unsafe { reference_from_jobject(clazz as _) };
	let Some(obj) = obj else {
		return ptr::null_mut() as jobjectArray;
	};

	let class = obj.extract_target_class();
	if init.is_null() {
		return match ObjectArrayInstance::new(len, class) {
			Throws::Ok(array) => Reference::object_array(array).into_jni(),
			Throws::Exception(e) => {
				e.throw(thread);
				ptr::null_mut() as jobjectArray
			},
		};
	}

	unimplemented!("jni::NewObjectArray with non-null init")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetObjectArrayElement(
	env: *mut JNIEnv,
	array: jobjectArray,
	index: jsize,
) -> jobject {
	unimplemented!("jni::GetObjectArrayElement");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn SetObjectArrayElement(
	env: *mut JNIEnv,
	array: jobjectArray,
	index: jsize,
	val: jobject,
) {
	let array = unsafe { reference_from_jobject(array as jobject) };
	let Some(array) = array else {
		return; // TODO: NPE?
	};

	let array = array.extract_object_array();

	let val = unsafe { reference_from_jobject(val) };
	let Some(val) = val else {
		return; // TODO: ArrayStoreException?
	};

	match array.store(index, val) {
		Throws::Ok(_) => {},
		Throws::Exception(e) => {
			let thread = JavaThread::current();
			assert_eq!(thread.env().raw(), env);
			e.throw(thread)
		},
	}
}

/// Generate all of the typed primitive array methods (`New<Type>Array`, `{Get,Set}<Type>ArrayRegion`).
macro_rules! define_primitive_array_methods {
    ($([$java_type:ident, $jni_type:ty, $rust_component_type:ty, $type_code:expr]),* $(,)?) => {
        $(
        paste::paste! {
            #[unsafe(no_mangle)]
            pub unsafe extern "system" fn [<New $java_type:camel Array>](env: *mut JNIEnv, len: jsize) -> $jni_type {
                let thread = JavaThread::current();
                assert_eq!(thread.env().raw(), env);

                match PrimitiveArrayInstance::new_from_type($type_code as u1, len) {
                    Throws::Ok(array) => Reference::array(array).into_jni(),
                    Throws::Exception(e) => {
                        e.throw(thread);
                        ptr::null_mut()
                    },
                }
            }

            #[unsafe(no_mangle)]
            pub unsafe extern "system" fn [<Get $java_type:camel ArrayRegion>](
                env: *mut JNIEnv,
                array: $jni_type,
                start: jsize,
                l: jsize,
                buf: *mut $rust_component_type,
            ) {
                let thread = JavaThread::current();
                assert_eq!(thread.env().raw(), env);

                let Some(array) = (unsafe { reference_from_jobject(array) }) else {
                    panic!("Invalid arguments to `{}`", stringify!([<Get $java_type:camel ArrayRegion>]));
                };

                if start < 0 || l < 0 {
                    throw!(thread, ArrayIndexOutOfBoundsException);
                }

                let start = start as usize;
                let len = l as usize;

                let buf = unsafe { std::slice::from_raw_parts_mut(buf, len) };
                let array_ref = array.extract_primitive_array();
                let slice = array_ref.as_slice::<$rust_component_type>();
                if start > slice.len() || slice[start..].len() < len {
                    throw!(thread, ArrayIndexOutOfBoundsException);
                }

                buf.copy_from_slice(&slice[start..len]);
            }

            #[unsafe(no_mangle)]
            pub unsafe extern "system" fn [<Set $java_type:camel ArrayRegion>](
                env: *mut JNIEnv,
                array: $jni_type,
                start: jsize,
                len: jsize,
                buf: *mut $rust_component_type,
            ) {
                let thread = JavaThread::current();
                assert_eq!(thread.env().raw(), env);

                let Some(array) = (unsafe { reference_from_jobject(array) }) else {
                    panic!("Invalid arguments to `{}`", stringify!([<Set $java_type:camel ArrayRegion>]));
                };

                if start < 0 || len < 0 {
                    throw!(thread, ArrayIndexOutOfBoundsException);
                }

                let buf = unsafe { std::slice::from_raw_parts_mut(buf, len as usize) };
                if let Throws::Exception(e) = array.extract_primitive_array().write_region(start, buf) {
                    e.throw(thread);
                }
            }
        }
        )*
    }
}

define_primitive_array_methods! {
	[boolean, jbooleanArray, jboolean, TypeCode::Boolean],
	[byte, jbyteArray, jbyte, TypeCode::Byte],
	[char, jcharArray, jchar, TypeCode::Char],
	[short, jshortArray, jshort, TypeCode::Short],
	[int, jintArray, jint, TypeCode::Int],
	[long, jlongArray, jlong, TypeCode::Long],
	[float, jfloatArray, jfloat, TypeCode::Float],
	[double, jdoubleArray, jdouble, TypeCode::Double],
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetBooleanArrayElements(
	env: *mut JNIEnv,
	array: jbooleanArray,
	isCopy: *mut jboolean,
) -> *mut jboolean {
	unimplemented!("jni::GetBooleanArrayElements")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetByteArrayElements(
	env: *mut JNIEnv,
	array: jbyteArray,
	isCopy: *mut jboolean,
) -> *mut jbyte {
	unimplemented!("jni::GetByteArrayElements")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetCharArrayElements(
	env: *mut JNIEnv,
	array: jcharArray,
	isCopy: *mut jboolean,
) -> *mut jchar {
	unimplemented!("jni::GetCharArrayElements")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetShortArrayElements(
	env: *mut JNIEnv,
	array: jshortArray,
	isCopy: *mut jboolean,
) -> *mut jshort {
	unimplemented!("jni::GetShortArrayElements")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetIntArrayElements(
	env: *mut JNIEnv,
	array: jintArray,
	isCopy: *mut jboolean,
) -> *mut jint {
	unimplemented!("jni::GetIntArrayElements")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetLongArrayElements(
	env: *mut JNIEnv,
	array: jlongArray,
	isCopy: *mut jboolean,
) -> *mut jlong {
	unimplemented!("jni::GetLongArrayElements")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetFloatArrayElements(
	env: *mut JNIEnv,
	array: jfloatArray,
	isCopy: *mut jboolean,
) -> *mut jfloat {
	unimplemented!("jni::GetFloatArrayElements")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetDoubleArrayElements(
	env: *mut JNIEnv,
	array: jdoubleArray,
	isCopy: *mut jboolean,
) -> *mut jdouble {
	unimplemented!("jni::GetDoubleArrayElements")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn ReleaseBooleanArrayElements(
	env: *mut JNIEnv,
	array: jbooleanArray,
	elems: *mut jboolean,
	mode: jint,
) {
	unimplemented!("jni::ReleaseBooleanArrayElements")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn ReleaseByteArrayElements(
	env: *mut JNIEnv,
	array: jbyteArray,
	elems: *mut jbyte,
	mode: jint,
) {
	unimplemented!("jni::ReleaseByteArrayElements")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn ReleaseCharArrayElements(
	env: *mut JNIEnv,
	array: jcharArray,
	elems: *mut jchar,
	mode: jint,
) {
	unimplemented!("jni::ReleaseCharArrayElements")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn ReleaseShortArrayElements(
	env: *mut JNIEnv,
	array: jshortArray,
	elems: *mut jshort,
	mode: jint,
) {
	unimplemented!("jni::ReleaseShortArrayElements")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn ReleaseIntArrayElements(
	env: *mut JNIEnv,
	array: jintArray,
	elems: *mut jint,
	mode: jint,
) {
	unimplemented!("jni::ReleaseIntArrayElements");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn ReleaseLongArrayElements(
	env: *mut JNIEnv,
	array: jlongArray,
	elems: *mut jlong,
	mode: jint,
) {
	unimplemented!("jni::ReleaseLongArrayElements")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn ReleaseFloatArrayElements(
	env: *mut JNIEnv,
	array: jfloatArray,
	elems: *mut jfloat,
	mode: jint,
) {
	unimplemented!("jni::ReleaseFloatArrayElements")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn ReleaseDoubleArrayElements(
	env: *mut JNIEnv,
	array: jdoubleArray,
	elems: *mut jdouble,
	mode: jint,
) {
	unimplemented!("jni::ReleaseDoubleArrayElements")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetPrimitiveArrayCritical(
	env: *mut JNIEnv,
	array: jarray,
	isCopy: *mut jboolean,
) -> *mut c_void {
	unimplemented!("jni::GetPrimitiveArrayCritical")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn ReleasePrimitiveArrayCritical(
	env: *mut JNIEnv,
	array: jarray,
	carray: *mut c_void,
	mode: jint,
) {
	unimplemented!("jni::ReleasePrimitiveArrayCritical");
}
