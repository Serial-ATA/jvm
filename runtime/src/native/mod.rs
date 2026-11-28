#![allow(non_snake_case)]

use crate::symbols::Symbol;

use std::cell::SyncUnsafeCell;
use std::ffi::{CStr, c_char, c_void};
use std::sync::Once;

use common::sync::ForceSendSync;
use common::unicode;
use platform::env::SystemPaths;
use platform::libs::Library;

pub mod java;
mod jdk;
pub mod jni;
pub mod jvm;
pub mod jvmti;
pub mod lookup;

/// Pointer to a native method implementation
#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
pub struct NativeMethodPtr(*const c_void);

unsafe impl Send for NativeMethodPtr {}
unsafe impl Sync for NativeMethodPtr {}

impl From<*const c_void> for NativeMethodPtr {
	fn from(value: *const c_void) -> Self {
		Self(value)
	}
}

impl From<*mut c_void> for NativeMethodPtr {
	fn from(value: *mut c_void) -> Self {
		Self(value.cast_const())
	}
}

impl From<NativeMethodPtr> for *const c_void {
	fn from(value: NativeMethodPtr) -> Self {
		value.0
	}
}

static LIB_JAVA: SyncUnsafeCell<Option<ForceSendSync<Library>>> = SyncUnsafeCell::new(None);

/// Get a handle to the loaded `libjava`
///
/// On the first call, this will try to load the library, aborting the process if not found.
pub(crate) fn lib_java() -> &'static Library {
	static ONCE: Once = Once::new();

	let paths = SystemPaths::init().expect("should be initialized way before this point");

	ONCE.call_once(
		|| match Library::load_from_path(&paths.boot_library_path, "java") {
			Ok(lib) => unsafe { *LIB_JAVA.get() = Some(ForceSendSync(lib)) },
			Err(e) => {
				eprintln!("Failed to load java library: {e}");
				std::process::abort();
			},
		},
	);

	unsafe { &*LIB_JAVA.get() }
		.as_ref()
		.expect("Should be set at this point")
}

trait RawSymbolExt {
	/// Intern a modified UTF-8 encoded C string into a `Symbol`
	unsafe fn intern_mutf_c_str(s: *const c_char) -> Symbol;
}

impl RawSymbolExt for Symbol {
	unsafe fn intern_mutf_c_str(s: *const c_char) -> Symbol {
		assert!(!s.is_null());

		let c = unsafe { CStr::from_ptr(s) };
		let decoded =
			unicode::decode(c.to_bytes()).expect("expected a valid modified UTF-8 string");
		Symbol::intern(decoded)
	}
}
