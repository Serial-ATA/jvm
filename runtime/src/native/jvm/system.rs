#![native_macros::jni_fn_module]

use crate::native::jni::reference_from_jobject;
use crate::objects::instance::array::Array;
use crate::objects::instance::object::Object;
use crate::thread::JavaThread;
use crate::thread::exceptions::{Throws, throw};

use std::time::{SystemTime, UNIX_EPOCH};

use ::jni::env::JniEnv;
use ::jni::objects::{JClass, JObject, JObjectArray, JString};
use ::jni::sys::{jint, jlong};
use native_macros::jni_call;

#[jni_call]
pub extern "C" fn JVM_CurrentTimeMillis(_env: JniEnv, _unused: JClass) -> jlong {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_NanoTime(_env: JniEnv, _unused: JClass) -> jlong {
	let time_nanos = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.expect("current system time should not be before the UNIX epoch")
		.as_nanos();

	time_nanos as jlong
}

#[jni_call]
pub extern "C" fn JVM_GetNanoTimeAdjustment(
	_env: JniEnv,
	_unused: JClass,
	_offset_secs: jlong,
) -> jlong {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_ArrayCopy(
	env: JniEnv,
	_unused: JClass,
	src: JObject,
	src_pos: jint,
	dst: JObject,
	dst_pos: jint,
	length: jint,
) {
	unsafe fn do_copy<T: Array>(src: T, src_pos: usize, dest: T, dest_pos: usize, length: usize) {
		unsafe {
			src.copy_into(src_pos, &dest, dest_pos, length);
		}
	}

	unsafe fn do_copy_within<T: Array>(src: T, src_pos: usize, dest_pos: usize, length: usize) {
		unsafe {
			src.copy_within(src_pos, dest_pos, length);
		}
	}

	let (Some(src), Some(dst)) = (unsafe { reference_from_jobject(src.raw()) }, unsafe {
		reference_from_jobject(dst.raw())
	}) else {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		throw!(thread, NullPointerException);
	};

	let src_len = match src.array_length() {
		Throws::Ok(len) => len,
		Throws::Exception(e) => {
			let thread = unsafe { &*JavaThread::for_env(env.raw()) };
			e.throw(thread);
			return;
		},
	};
	let dest_len = match dst.array_length() {
		Throws::Ok(len) => len,
		Throws::Exception(e) => {
			let thread = unsafe { &*JavaThread::for_env(env.raw()) };
			e.throw(thread);
			return;
		},
	};

	// TODO: Verify component types

	if src_pos < 0
		|| dst_pos < 0
		|| length < 0
		|| src_pos + length > src_len as jint
		|| dst_pos + length > dest_len as jint
	{
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		throw!(thread, IndexOutOfBoundsException);
	}

	if length == 0 {
		return;
	}

	if src == dst {
		if src.is_object_array() {
			unsafe {
				do_copy_within(
					src.extract_object_array(),
					src_pos as usize,
					dst_pos as usize,
					length as usize,
				)
			}
		} else {
			unsafe {
				do_copy_within(
					src.extract_primitive_array(),
					src_pos as usize,
					dst_pos as usize,
					length as usize,
				)
			}
		}

		return;
	}

	if src.is_object_array() {
		unsafe {
			do_copy(
				src.extract_object_array(),
				src_pos as usize,
				dst.extract_object_array(),
				dst_pos as usize,
				length as usize,
			)
		}
	} else {
		unsafe {
			do_copy(
				src.extract_primitive_array(),
				src_pos as usize,
				dst.extract_primitive_array(),
				dst_pos as usize,
				length as usize,
			)
		}
	}
}

#[jni_call]
pub extern "C" fn JVM_GetProperties(_env: JniEnv) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetTemporaryDirectory(_env: JniEnv) -> JString {
	todo!()
}
