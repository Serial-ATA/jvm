#![allow(non_snake_case)]

mod class;
pub mod intrinsics;
pub mod jni;
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

// Module marker, do not remove

pub(crate) mod java {
	pub(crate) mod io {
		pub(crate) mod FileDescriptor;
		pub(crate) mod FileInputStream;
		pub(crate) mod FileOutputStream;
		pub(crate) mod FileSystem;
		pub(crate) mod UnixFileSystem;
	}
	pub(crate) mod lang {
		pub(crate) mod invoke {
			pub(crate) mod MethodHandle;
			pub(crate) mod MethodHandleNatives;
		}
		pub(crate) mod r#ref {
			pub(crate) mod Finalizer;
			pub(crate) mod PhantomReference;
			pub(crate) mod Reference;
		}
		pub(crate) mod reflect {
			pub(crate) mod Array;
		}
		pub(crate) mod Class;
		pub(crate) mod ClassLoader;
		pub(crate) mod Double;
		pub(crate) mod Float;
		pub(crate) mod Module;
		pub(crate) mod NullPointerException;
		pub(crate) mod Object;
		pub(crate) mod Runtime;
		pub(crate) mod StackTraceElement;
		pub(crate) mod String;
		pub(crate) mod StringBuilder;
		pub(crate) mod StringUTF16;
		pub(crate) mod System;
		pub(crate) mod Thread;
		pub(crate) mod Throwable;
	}
	pub(crate) mod security {
		pub(crate) mod AccessController;
	}
}

pub(crate) mod jdk {
	pub(crate) mod internal {
		pub(crate) mod misc {
			pub(crate) mod CDS;
			pub(crate) mod ScopedMemoryAccess;
			pub(crate) mod Signal;
			pub(crate) mod Unsafe;
			pub(crate) mod VM;
		}
		pub(crate) mod util {
			pub(crate) mod SystemProps;
		}
		pub(crate) mod loader {
			pub(crate) mod BootLoader;
			pub(crate) mod NativeLibraries;
		}
		pub(crate) mod reflect {
			pub(crate) mod DirectConstructorHandleAccessor;
			pub(crate) mod DirectMethodHandleAccessor;
			pub(crate) mod Reflection;
		}
	}
}
