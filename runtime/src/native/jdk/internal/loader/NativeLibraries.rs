use crate::classes;
use crate::native::java::lang::String::StringInterner;
use crate::objects::class::Class;
use crate::objects::reference::Reference;
use crate::thread::JavaThread;
use crate::thread::exceptions::throw_and_return_null;

use std::ffi::CStr;

use ::jni::env::JniEnv;
use ::jni::java_vm::JniOnLoadFn;
use ::jni::sys::{jboolean, jlong};
use common::traits::PtrType;
use platform::{JNI_LIB_PREFIX, JNI_LIB_SUFFIX};

include_generated!("native/jdk/internal/loader/def/NativeLibraries.definitions.rs");

pub fn load(
	_env: JniEnv,
	_class: &'static Class,
	_impl_: Reference, // jdk.internal.loader.NativeLibraries$NativeLibraryImpl
	_name: Reference,  // java.lang.String
	_is_builtin: jboolean,
	_throw_exception_if_fail: jboolean,
) -> jboolean {
	unimplemented!("jdk.internal.loader.NativeLibraries#load")
}

pub fn unload(
	_env: JniEnv,
	_class: &'static Class,
	_name: Reference, // java.lang.String
	_is_builtin: jboolean,
	_handle: jlong,
) {
	unimplemented!("jdk.internal.loader.NativeLibraries#unload")
}

pub fn findBuiltinLib(
	env: JniEnv,
	_class: &'static Class,
	name: Reference, // java.lang.String
) -> Reference /* java.lang.String */
{
	if name.is_null() {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		throw_and_return_null!(thread, NullPointerException);
	}

	let lib_name = classes::java::lang::String::extract(name.extract_class().get());
	if lib_name.len() <= JNI_LIB_PREFIX.len() + JNI_LIB_SUFFIX.len() {
		return Reference::null();
	}

	let Ok(lib) = platform::libs::Library::load(&*lib_name) else {
		return Reference::null();
	};

	let mut stripped_lib_name = &*lib_name;
	if let Some(lstrip) = lib_name.strip_prefix(JNI_LIB_PREFIX) {
		stripped_lib_name = lstrip;
	}

	if let Some(rstrip) = lib_name.strip_suffix(JNI_LIB_SUFFIX) {
		stripped_lib_name = rstrip;
	}

	let mut on_load_sym = format!("JNI_OnLoad_{stripped_lib_name}").into_bytes();
	on_load_sym.push(b'\0');

	let on_load_sym_cstr = unsafe { CStr::from_bytes_with_nul_unchecked(on_load_sym.as_slice()) };
	let Ok(_) = (unsafe { lib.symbol::<JniOnLoadFn>(on_load_sym_cstr) }) else {
		return Reference::null();
	};

	Reference::class(StringInterner::intern(stripped_lib_name))
}
