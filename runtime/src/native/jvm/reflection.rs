#![native_macros::jni_fn_module]

use std::ffi::c_char;

use jni::env::JniEnv;
use jni::objects::{JByteArray, JClass, JObject, JObjectArray, JString};
use jni::sys::{jboolean, jbyte, jint, jsize};
use native_macros::jni_call;

#[jni_call]
pub extern "C" fn JVM_GetCallerClass(_env: JniEnv) -> JClass {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_FindPrimitiveClass(_env: JniEnv, _utf: *const c_char) -> JClass {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_FindClassFromBootLoader(_env: JniEnv, _name: *const c_char) -> JClass {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_FindClassFromLoader(
	_env: JniEnv,
	_name: *const c_char,
	_init: jboolean,
	_loader: JObject,
) -> JClass {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_FindClassFromClass(
	_env: JniEnv,
	_name: *const c_char,
	_init: jboolean,
	_from: JClass,
) -> JClass {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_DefineClass(
	_env: JniEnv,
	_name: *const c_char,
	_loader: JObject,
	_buf: *const jbyte,
	_len: jsize,
	_protection_domain: JObject,
) -> JClass {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_LookupDefineClass(
	_env: JniEnv,
	_lookup: JClass,
	_name: *const c_char,
	_loader: JObject,
	_buf: *const jbyte,
	_len: jsize,
	_protection_domain: JObject,
	_initialize: jboolean,
	_flags: jint,
	_class_data: JObject,
) -> JClass {
	todo!()
}

#[jni_call(no_strict_types)]
pub extern "C" fn JVM_DefineClassWithSource(
	_env: JniEnv,
	_name: *const c_char,
	_loader: JObject,
	_buf: *const jbyte,
	_len: jsize,
	_protection_domain: JObject,
	_source: *const c_char,
) -> JClass {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_FindLoadedClass(_env: JniEnv, _loader: JObject, _name: JString) -> JClass {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_InitClassName(_env: JniEnv, _class: JClass) -> JString {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetClassInterfaces(_env: JniEnv, _class: JClass) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_IsHiddenClass(_env: JniEnv, _class: JClass) -> jboolean {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_FindScopedValueBindings(_env: JniEnv, _class: JClass) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetDeclaredClasses(_env: JniEnv, _class: JClass) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetDeclaringClass(_env: JniEnv, _class: JClass) -> JClass {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetSimpleBinaryName(_env: JniEnv, _class: JClass) -> JString {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetClassSignature(_env: JniEnv, _class: JClass) -> JString {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetClassAnnotations(_env: JniEnv, _class: JClass) -> JByteArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetClassTypeAnnotations(_env: JniEnv, _class: JClass) -> JByteArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetMethodTypeAnnotations(_env: JniEnv, _method: JObject) -> JByteArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetFieldTypeAnnotations(_env: JniEnv, _field: JObject) -> JByteArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetMethodParameters(_env: JniEnv, _method: JObject) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetClassDeclaredFields(
	_env: JniEnv,
	_class: JClass,
	_public_only: jboolean,
) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_IsRecord(_env: JniEnv, _class: JClass) -> jboolean {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetRecordComponents(_env: JniEnv, _class: JClass) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetClassDeclaredMethods(
	_env: JniEnv,
	_class: JClass,
	_public_only: jboolean,
) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetClassDeclaredConstructors(
	_env: JniEnv,
	_class: JClass,
	_public_only: jboolean,
) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_AreNestMates(_env: JniEnv, _current: JClass, _member: JClass) -> jboolean {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetNestHost(_env: JniEnv, _current: JClass) -> JClass {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetNestMembers(_env: JniEnv, _current: JClass) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetPermittedSubclasses(_env: JniEnv, _current: JClass) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetClassFileVersion(_env: JniEnv, _current: JClass) -> jint {
	todo!()
}
