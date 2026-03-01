#![native_macros::jni_fn_module]

use jni::env::JniEnv;
use jni::objects::{JClass, JObject, JObjectArray};
use jni::sys::{jboolean, jint};
use native_macros::jni_call;
use std::ffi::{c_char, c_int, c_uchar, c_ushort};

#[jni_call]
pub extern "C" fn JVM_InvokeMethod(
	_env: JniEnv,
	_method: JObject,
	_obj: JObject,
	_args0: JObjectArray,
) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_NewInstanceFromConstructor(
	_env: JniEnv,
	_c: JObject,
	_args0: JObjectArray,
) -> JObject {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetMethodIxExceptionIndexes(
	_env: JniEnv,
	_cb: JClass,
	_method_index: jint,
	_exceptions: *mut c_ushort,
) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetMethodIxExceptionsCount(
	_env: JniEnv,
	_cb: JClass,
	_method_index: jint,
) -> jint {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetMethodIxByteCode(
	_env: JniEnv,
	_cb: JClass,
	_method_index: jint,
	_code: *mut c_uchar,
) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetMethodIxByteCodeLength(
	_env: JniEnv,
	_cb: JClass,
	_method_index: jint,
) -> jint {
	todo!()
}

#[repr(C)]
pub struct JVM_ExceptionTableEntryType {
	pub start_pc: jint,
	pub end_pc: jint,
	pub handler_pc: jint,
	pub catch_type: jint,
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetMethodIxExceptionTableEntry(
	_env: JniEnv,
	_cb: JClass,
	_method_index: jint,
	_entry_index: jint,
	_entry: *mut JVM_ExceptionTableEntryType,
) {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetMethodIxExceptionTableLength(
	_env: JniEnv,
	_cb: JClass,
	_index: c_int,
) -> jint {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetMethodIxModifiers(_env: JniEnv, _cb: JClass, _index: c_int) -> jint {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetMethodIxLocalsCount(_env: JniEnv, _cb: JClass, _index: c_int) -> jint {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetMethodIxArgsSize(_env: JniEnv, _cb: JClass, _index: c_int) -> jint {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetMethodIxMaxStack(_env: JniEnv, _cb: JClass, _index: c_int) -> jint {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_IsConstructorIx(_env: JniEnv, _cb: JClass, _index: c_int) -> jboolean {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_IsVMGeneratedMethodIx(_env: JniEnv, _cb: JClass, _index: c_int) -> jboolean {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetMethodIxNameUTF(_env: JniEnv, _cb: JClass, _index: jint) -> *const c_char {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetMethodIxSignatureUTF(
	_env: JniEnv,
	_cb: JClass,
	_index: jint,
) -> *const c_char {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetEnclosingMethodInfo(_env: JniEnv, _class: JClass) -> JObjectArray {
	todo!()
}
