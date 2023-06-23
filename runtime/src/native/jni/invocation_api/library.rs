//! # Library and Version Management
//!
//! Once a native library is loaded, it is visible from all class loaders. Therefore two classes in different class loaders may link with the same native method. This leads to two problems:
//!
//!  * A class may mistakenly link with native libraries loaded by a class with the same name in a different class loader.
//!  * Native methods can easily mix classes from different class loaders. This breaks the name space separation offered by class loaders, and leads to type safety problems.
//!
//! Each class loader manages its own set of native libraries. The same JNI native library cannot be loaded into more than one class loader.
//! Doing so causes `UnsatisfiedLinkError` to be thrown. For example, `System.loadLibrary` throws an `UnsatisfiedLinkError` when used to load a native library into two class loaders.
//!
//! The benefits of the new approach are:
//!
//!  * Name space separation based on class loaders is preserved in native libraries. A native library cannot easily mix classes from different class loaders.
//!  * In addition, native libraries can be unloaded when their corresponding class loaders are garbage collected.

use core::ffi::c_void;
use jni::sys::{jint, JavaVM};

extern "system" {
	pub fn JNI_OnLoad(vm: *mut JavaVM, reserved: *mut c_void) -> jint;
	pub fn JNI_OnUnload(vm: *mut JavaVM, reserved: *mut c_void);
}
