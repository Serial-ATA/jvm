use crate::native::jni::reference_from_jobject;
use crate::native::method::NativeMethodPtr;
use crate::objects::method::MethodEntryPoint;
use crate::symbols::Symbol;
use crate::thread::JavaThread;
use crate::thread::exceptions::{Throws, throw_with_ret};

use std::ffi::CStr;

use ::jni::sys::{JNI_ERR, JNI_OK, JNIEnv, JNINativeMethod, jclass, jint};
use common::unicode;

#[unsafe(no_mangle)]
pub extern "system" fn RegisterNatives(
	env: *mut JNIEnv,
	clazz: jclass,
	methods: *const JNINativeMethod,
	nMethods: jint,
) -> jint {
	let thread = JavaThread::current();
	assert_eq!(thread.env().raw(), env);

	let Some(mirror) = (unsafe { reference_from_jobject(clazz) }) else {
		panic!("Invalid arguments to `GetSuperclass`");
	};

	let class = mirror.extract_target_class();
	if class.is_array() || class.is_interface() {
		return JNI_ERR;
	}

	// SAFETY: It's up to the caller to provide a valid array
	let methods = unsafe { std::slice::from_raw_parts(methods, nMethods as usize) };

	for raw_method in methods {
		// SAFETY: It's up to the caller to provide valid strings
		let name_c = unsafe { CStr::from_ptr(raw_method.name) };
		let signature_c = unsafe { CStr::from_ptr(raw_method.signature) };

		let (Ok(name), Ok(signature)) = (
			unicode::decode(name_c.to_bytes()),
			unicode::decode(signature_c.to_bytes()),
		) else {
			return JNI_ERR;
		};

		match class.resolve_method(Symbol::intern(name), Symbol::intern(signature)) {
			Throws::Ok(method) => {
				if !method.is_native() {
					throw_with_ret!(
						0,
						thread,
						NoSuchMethodError,
						"Method '{}' is not declared as native",
						method.external_name()
					);
				}

				method.set_entry_point(MethodEntryPoint::NativeMethod(NativeMethodPtr::External(
					raw_method.fnPtr,
				)));
			},
			Throws::Exception(e) => {
				e.throw(thread);
				return 0;
			},
		}
	}

	JNI_OK
}

#[unsafe(no_mangle)]
pub extern "system" fn UnregisterNatives(env: *mut JNIEnv, clazz: jclass) -> jint {
	unimplemented!("jni::UnregisterNatives");
}
