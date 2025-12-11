#![allow(non_snake_case)]

use std::cell::SyncUnsafeCell;
use std::sync::Once;

use common::sync::ForceSendSync;
use platform::env::SystemPaths;
use platform::libs::Library;

mod class;
pub mod intrinsics;
pub mod jni;
pub mod jvm;
pub mod lookup;
pub mod method;

#[macro_export]
macro_rules! include_generated {
	($path:literal) => {
		include!(std::concat!(
			env!("CARGO_MANIFEST_DIR"),
			"/../generated/",
			$path
		));
	};
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

	unsafe { (&*LIB_JAVA.get()) }
		.as_ref()
		.expect("Should be set at this point")
}

// Module marker, do not remove

pub(crate) mod java {
	pub(crate) mod io {
		pub(crate) mod FileInputStream;
		pub(crate) mod UnixFileSystem;
		pub(crate) mod FileDescriptor;
		pub(crate) mod FileSystem;
		pub(crate) mod FileOutputStream;
	}
	pub(crate) mod lang {
		pub(crate) mod invoke {
			pub(crate) mod MethodHandleNatives;
			pub(crate) mod MethodHandle;
		}
		pub(crate) mod r#ref {
			pub(crate) mod PhantomReference;
			pub(crate) mod Reference;
			pub(crate) mod Finalizer;
		}
		pub(crate) mod reflect {
			pub(crate) mod Array;
		}
		pub(crate) mod StringBuilder;
		pub(crate) mod Runtime;
		pub(crate) mod StringUTF16;
		pub(crate) mod Float;
		pub(crate) mod Module;
		pub(crate) mod ClassLoader;
		pub(crate) mod Double;
		pub(crate) mod Throwable;
		pub(crate) mod Thread;
		pub(crate) mod String;
		pub(crate) mod Class;
	}
	pub(crate) mod security {
		pub(crate) mod AccessController;
	}
}

pub(crate) mod jdk {
	pub(crate) mod internal {
		pub(crate) mod misc {
			pub(crate) mod ScopedMemoryAccess;
			pub(crate) mod CDS;
			pub(crate) mod VM;
			pub(crate) mod Unsafe;
			pub(crate) mod Signal;
		}
		pub(crate) mod util {
			pub(crate) mod SystemProps;
		}
		pub(crate) mod loader {
			pub(crate) mod NativeLibraries;
			pub(crate) mod BootLoader;
			pub(crate) mod NativeLibrary;
		}
		pub(crate) mod reflect {
			pub(crate) mod DirectConstructorHandleAccessor;
			pub(crate) mod DirectMethodHandleAccessor;
			pub(crate) mod Reflection;
		}
	}
}

