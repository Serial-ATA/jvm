use crate::native::java::lang::String::StringInterner;
use crate::objects::class::ClassPtr;
use crate::objects::reference::Reference;
use crate::thread::JavaThread;
use crate::thread::exceptions::{
	Throws, handle_exception, throw, throw_and_return_null, throw_with_ret,
};
use crate::{classes, globals};

use std::ffi::{CStr, c_void};
use std::ptr::NonNull;
use std::sync::{LazyLock, Once};

use ::jni::env::JniEnv;
use ::jni::java_vm::JniOnLoadFn;
use ::jni::sys::{jboolean, jlong};
use common::sync::ForceSendSync;
use jni::sys::JNI_VERSION_1_1;
use jni::version::JniVersion;
use platform::libs::{Library, Sym};
use platform::{JNI_LIB_PREFIX, JNI_LIB_SUFFIX};

include_generated!("native/jdk/internal/loader/def/NativeLibraries.definitions.rs");

static PROCESS_HANDLE: LazyLock<Option<ForceSendSync<NonNull<c_void>>>> = LazyLock::new(|| {
	platform::libs::Library::current().ok().map(|current| {
		ForceSendSync::new(NonNull::new(current.raw()).expect("current library should not be null"))
	})
});

fn process_handle() -> Throws<platform::libs::Library> {
	let Some(ptr) = &*PROCESS_HANDLE else {
		// Should never happen, but just in case...
		throw!(@DEFER InternalError, "Unable to determine current library");
	};

	// SAFETY: Pointer came from Library::current()
	Throws::Ok(unsafe { platform::libs::Library::from_raw(ptr.as_ptr()) })
}

struct LibraryName {
	name: String,
	stripped_name: String,
}

fn extract_and_verify_lib_name(name: Reference) -> Option<LibraryName> {
	let lib_name = classes::java::lang::String::extract(name.extract_class());
	if lib_name.len() <= JNI_LIB_PREFIX.len() + JNI_LIB_SUFFIX.len() {
		return None;
	}

	let mut stripped_lib_name = &*lib_name;
	if let Some(lstrip) = lib_name.strip_prefix(JNI_LIB_PREFIX) {
		stripped_lib_name = lstrip;
	}

	if let Some(rstrip) = lib_name.strip_suffix(JNI_LIB_SUFFIX) {
		stripped_lib_name = rstrip;
	}

	let stripped_name = stripped_lib_name.to_string();
	Some(LibraryName {
		name: lib_name,
		stripped_name,
	})
}

fn find_jni_on_load<'a>(lib: &'a Library, name: Option<&str>) -> Option<Sym<'a, JniOnLoadFn>> {
	let mut on_load_sym = match name {
		Some(name) => format!("JNI_OnLoad_{}\0", name).into_bytes(),
		None => String::from("JNI_OnLoad\0").into_bytes(),
	};

	let on_load_sym_cstr = unsafe { CStr::from_bytes_with_nul_unchecked(on_load_sym.as_slice()) };
	unsafe { lib.symbol::<JniOnLoadFn>(on_load_sym_cstr) }.ok()
}

fn init_ids(native_libraries_class: ClassPtr, native_library_impl_class: ClassPtr) {
	static ONCE: Once = Once::new();

	ONCE.call_once(|| unsafe {
		globals::classes::set_jdk_internal_loader_NativeLibraries(native_libraries_class);
		globals::classes::set_jdk_internal_loader_NativeLibraries_NativeLibraryImpl(
			native_library_impl_class,
		);
		classes::jdk::internal::loader::NativeLibraries::init_offsets();
	});
}

pub fn load(
	env: JniEnv,
	class: ClassPtr,
	impl_: Reference, // jdk.internal.loader.NativeLibraries$NativeLibraryImpl
	name: Reference,  // java.lang.String
	is_builtin: jboolean,
	throw_exception_if_fail: jboolean,
) -> jboolean {
	init_ids(class, impl_.extract_target_class());

	let thread = unsafe { &*JavaThread::for_env(env.raw()) };

	let Some(LibraryName { name: lib_name, .. }) = extract_and_verify_lib_name(name) else {
		return false;
	};

	let target_lib;
	if is_builtin {
		target_lib = handle_exception!(false, thread, process_handle());
	} else {
		match platform::libs::Library::load(&lib_name) {
			Ok(lib) => target_lib = lib,
			Err(e) => {
				if throw_exception_if_fail {
					throw_with_ret!(false, thread, UnsatisfiedLinkError, "{lib_name}: {e}");
				}

				return false;
			},
		}
	}

	let jni_version;
	let name = if is_builtin { Some(&*lib_name) } else { None };
	match find_jni_on_load(&target_lib, name) {
		Some(on_load) => {
			let Ok(vm) = env.get_java_vm() else {
				throw_with_ret!(
					false,
					thread,
					InternalError,
					"Unable to get the Java VM of the current thread"
				);
			};

			jni_version = unsafe { on_load(vm.raw().cast_mut(), std::ptr::null_mut()) };

			if thread.has_pending_exception() {
				if !is_builtin {
					let _ = target_lib.close();
				}

				return false;
			}
		},
		None => jni_version = JNI_VERSION_1_1,
	}

	if JniVersion::from_raw(jni_version).is_none() {
		if !is_builtin {
			let _ = target_lib.close();
		}

		throw_with_ret!(
			false,
			thread,
			UnsatisfiedLinkError,
			"unsupported JNI version {jni_version:#08X} required by {lib_name}"
		);
	}

	classes::jdk::internal::loader::NativeLibraries::NativeLibraryImpl::set_jniVersion(
		impl_,
		jni_version,
	);
	classes::jdk::internal::loader::NativeLibraries::NativeLibraryImpl::set_handle(
		impl_,
		target_lib.raw() as jlong,
	);

	true
}

pub fn unload(
	_env: JniEnv,
	_class: ClassPtr,
	_name: Reference, // java.lang.String
	_is_builtin: jboolean,
	_handle: jlong,
) {
	unimplemented!("jdk.internal.loader.NativeLibraries#unload")
}

pub fn findBuiltinLib(
	env: JniEnv,
	_class: ClassPtr,
	name: Reference, // java.lang.String
) -> Reference /* java.lang.String */
{
	if name.is_null() {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		throw_and_return_null!(thread, NullPointerException);
	}

	let Some(lib_name) = extract_and_verify_lib_name(name) else {
		return Reference::null();
	};

	let Ok(lib) = platform::libs::Library::load(&lib_name.name) else {
		return Reference::null();
	};

	if find_jni_on_load(&lib, Some(&lib_name.stripped_name)).is_none() {
		return Reference::null();
	}

	Reference::class(StringInterner::intern(lib_name.stripped_name))
}
