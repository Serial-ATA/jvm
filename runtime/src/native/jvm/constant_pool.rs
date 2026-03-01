#![native_macros::jni_fn_module]

use jni::env::JniEnv;
use jni::objects::{JClass, JObject, JObjectArray, JString};
use jni::sys::{jbyte, jdouble, jfloat, jint, jlong};
use native_macros::jni_call;
use std::ffi::{c_char, c_int, c_uchar};

#[jni_call]
pub extern "C" fn JVM_GetClassConstantPool(_env: JniEnv, _class: JClass) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_ConstantPoolGetSize(_env: JniEnv, _obj: JObject) -> jint {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_ConstantPoolGetClassAt(_env: JniEnv, _obj: JObject, _index: jint) -> JClass {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_ConstantPoolGetClassAtIfLoaded(
	_env: JniEnv,
	_obj: JObject,
	_index: jint,
) -> JClass {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_ConstantPoolGetMethodAt(
	_env: JniEnv,
	_obj: JObject,
	_index: jint,
) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_ConstantPoolGetMethodAtIfLoaded(
	_env: JniEnv,
	_obj: JObject,
	_index: jint,
) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_ConstantPoolGetFieldAt(_env: JniEnv, _obj: JObject, _index: jint) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_ConstantPoolGetFieldAtIfLoaded(
	_env: JniEnv,
	_obj: JObject,
	_index: jint,
) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_ConstantPoolGetMemberRefInfoAt(
	_env: JniEnv,
	_obj: JObject,
	_index: jint,
) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_ConstantPoolGetClassRefIndexAt(
	_env: JniEnv,
	_obj: JObject,
	_index: jint,
) -> jint {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_ConstantPoolGetNameAndTypeRefIndexAt(
	_env: JniEnv,
	_obj: JObject,
	_index: jint,
) -> jint {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_ConstantPoolGetNameAndTypeRefInfoAt(
	_env: JniEnv,
	_obj: JObject,
	_index: jint,
) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_ConstantPoolGetIntAt(_env: JniEnv, _obj: JObject, _index: jint) -> jint {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_ConstantPoolGetLongAt(_env: JniEnv, _obj: JObject, _index: jint) -> jlong {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_ConstantPoolGetFloatAt(_env: JniEnv, _obj: JObject, _index: jint) -> jfloat {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_ConstantPoolGetDoubleAt(
	_env: JniEnv,
	_obj: JObject,
	_index: jint,
) -> jdouble {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_ConstantPoolGetStringAt(
	_env: JniEnv,
	_obj: JObject,
	_index: jint,
) -> JString {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_ConstantPoolGetUTF8At(_env: JniEnv, _obj: JObject, _index: jint) -> JString {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_ConstantPoolGetTagAt(_env: JniEnv, _obj: JObject, _index: jint) -> jbyte {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetClassCPTypes(_env: JniEnv, _cb: JClass, _types: *mut c_uchar) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetClassCPEntriesCount(_env: JniEnv, _cb: JClass) -> jint {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetCPFieldNameUTF(_env: JniEnv, _cb: JClass, _index: jint) -> *const c_char {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetCPMethodNameUTF(_env: JniEnv, _cb: JClass, _index: jint) -> *const c_char {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetCPMethodSignatureUTF(
	_env: JniEnv,
	_cb: JClass,
	_index: jint,
) -> *const c_char {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetCPFieldSignatureUTF(
	_env: JniEnv,
	_cb: JClass,
	_index: jint,
) -> *const c_char {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetCPClassNameUTF(_env: JniEnv, _cb: JClass, _index: jint) -> *const c_char {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetCPFieldClassNameUTF(
	_env: JniEnv,
	_cb: JClass,
	_index: jint,
) -> *const c_char {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetCPMethodClassNameUTF(
	_env: JniEnv,
	_cb: JClass,
	_index: jint,
) -> *const c_char {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetCPFieldModifiers(
	_env: JniEnv,
	_cb: JClass,
	_index: c_int,
	_called_class: JClass,
) -> jint {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetCPMethodModifiers(
	_env: JniEnv,
	_cb: JClass,
	_index: c_int,
	_called_class: JClass,
) -> jint {
	todo!()
}
