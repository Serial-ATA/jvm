use crate::native::jni::JniEnvExt;
use crate::native::jni::references::JObjectExt;
use crate::objects::instance::array::{
	ObjectArrayInstance, PrimitiveArrayInstance, PrimitiveArrayInstanceRef,
};
use crate::objects::reference::Reference;
use crate::thread::exceptions::{handle_exception, throw_with_ret};

use std::ffi::c_uchar;

use ::jni::env::JniEnv;
use ::jni::objects::{JArray, JClass, JIntArray, JObject};
use ::jni::sys::{jboolean, jint, jvalue};
use native_macros::jni_call;

/// Maximum number of array dimensions
pub const MAX_DIM: u8 = 255;

#[jni_call]
pub extern "C" fn JVM_CopyOfSpecialArray(
	_env: JniEnv,
	_array: JArray,
	_from: jint,
	_to: jint,
) -> JArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_IsAtomicArray(_env: JniEnv, _array: JArray) -> jboolean {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_IsFlatArray(_env: JniEnv, _array: JArray) -> jboolean {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_IsNullRestrictedArray(_env: JniEnv, _array: JArray) -> jboolean {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetArrayLength(_env: JniEnv, _array: JObject) -> jint {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetArrayElement(_env: JniEnv, _array: JObject, _index: jint) -> JObject {
	todo!()
}

// TODO: Support jvalue, remove no_strict_types
#[jni_call(no_strict_types)]
pub extern "C" fn JVM_GetPrimitiveArrayElement(
	_env: JniEnv,
	_array: JObject,
	_index: jint,
	_w_code: jint,
) -> jvalue {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_SetArrayElement(_env: JniEnv, _array: JObject, _index: jint, _val: JObject) {
	todo!()
}

// TODO: Support jvalue, remove no_strict_types
#[jni_call(no_strict_types)]
pub extern "C" fn JVM_SetPrimitiveArrayElement(
	_env: JniEnv,
	_array: JObject,
	_index: jint,
	_val: jvalue,
	_v_code: c_uchar,
) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_NewArray(env: JniEnv, element_class: JClass, length: jint) -> JObject {
	let thread = unsafe { env.thread() };

	let Some(element_class) = (unsafe { element_class.to_reference() }) else {
		throw_with_ret!(JObject::null(), thread, NullPointerException);
	};

	if length.is_negative() {
		throw_with_ret!(
			JObject::null(),
			thread,
			NegativeArraySizeException,
			"{length}"
		);
	}

	let mirror = element_class.extract_mirror();
	if mirror.is_primitive() {
		let type_code = mirror
			.primitive_target()
			.as_array_type_code()
			.expect("should be a valid array type code");

		let array = PrimitiveArrayInstance::new_from_type(type_code, length);
		let array_instance: PrimitiveArrayInstanceRef =
			handle_exception!(JObject::null(), thread, array);
		return thread
			.jni_refs()
			.allocate_wrapped(Reference::array(array_instance));
	}

	let class = mirror.target_class();
	if class.is_array() && class.unwrap_array_instance().dimensions >= MAX_DIM {
		throw_with_ret!(JObject::null(), thread, IllegalArgumentException);
	}

	let array = ObjectArrayInstance::new(length, class);
	let array_instance = handle_exception!(JObject::null(), thread, array);
	thread
		.jni_refs()
		.allocate_wrapped(Reference::object_array(array_instance))
}

#[jni_call]
pub extern "C" fn JVM_NewMultiArray(
	_env: JniEnv,
	_element_class: JClass,
	_dimensions: JIntArray,
) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_NewNullRestrictedAtomicArray(
	_env: JniEnv,
	_element_class: JClass,
	_len: jint,
	_init_value: JObject,
) -> JArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_NewNullRestrictedNonAtomicArray(
	_env: JniEnv,
	_element_class: JClass,
	_len: jint,
	_init_value: JObject,
) -> JArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_NewNullableAtomicArray(
	_env: JniEnv,
	_element_class: JClass,
	_len: jint,
) -> JArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_NewReferenceArray(
	_env: JniEnv,
	_element_class: JClass,
	_len: jint,
) -> JArray {
	todo!()
}
