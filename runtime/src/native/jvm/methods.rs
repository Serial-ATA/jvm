#![native_macros::jni_fn_module]

use crate::native::java::lang::String::StringInterner;
use crate::native::jni::references::JObjectExt;
use crate::native::jvm::reflection::ensure_class_mirror;
use crate::objects::instance::array::{Array, ObjectArrayInstance, ObjectArrayInstanceRef};
use crate::objects::reference::Reference;
use crate::thread::JavaThread;
use crate::thread::exceptions::handle_exception;

use std::ffi::{c_char, c_int, c_uchar, c_ushort};

use jni::env::JniEnv;
use jni::objects::{JClass, JObject, JObjectArray};
use jni::sys::{jboolean, jint};
use native_macros::jni_call;

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

// Returns Object[3] where:
//
// [0]: The class holding the enclosing method (not null, java.lang.Class<?>)
// [1]: The enclosing method's name (nullable, java.lang.String)
// [2]: The enclosing method's descriptor (nullable. java.lang.String)
#[jni_call]
pub extern "C" fn JVM_GetEnclosingMethodInfo(env: JniEnv, class: JClass) -> JObjectArray {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	let this = unsafe { class.to_reference() }.expect("invalid class reference");
	let Some(mirror) = handle_exception!(JObjectArray::null(), thread, ensure_class_mirror(this))
	else {
		return JObjectArray::null();
	};

	let array = ObjectArrayInstance::new(3, crate::globals::classes::java_lang_Object());
	let array_instance: ObjectArrayInstanceRef =
		handle_exception!(JObjectArray::null(), thread, array);

	let target_class = mirror.target_class();

	let Some(enclosing_method) = target_class.unwrap_class_instance().enclosing_method else {
		// Class has no immediate enclosing method/class information
		return JObjectArray::null();
	};

	let enclosing_class = enclosing_method.class;

	// SAFETY: We know that the array has a length of 3
	unsafe {
		array_instance.store_unchecked(0, Reference::mirror(enclosing_class.mirror()));
	}

	let Some(enclosing_method) = enclosing_method.method else {
		// There is no immediate enclosing method
		return thread.jni_refs().allocate_wrapped(array_instance);
	};

	unsafe {
		array_instance.store_unchecked(
			1,
			Reference::class(StringInterner::intern(enclosing_method.name)),
		);

		array_instance.store_unchecked(
			2,
			Reference::class(StringInterner::intern(enclosing_method.descriptor_sym())),
		);
	}

	thread.jni_refs().allocate_wrapped(array_instance)
}
