//! # NIO Support
//!
//! The NIO-related entry points allow native code to access `java.nio` direct buffers.
//! The contents of a direct buffer can, potentially, reside in native memory outside of the ordinary garbage-collected heap.
//!
//! For information about direct buffers, please see New I/O APIs and the specification of the `java.nio.ByteBuffer` class.
//!
//! Three new functions introduced in JDK/JRE 1.4 allow JNI code to create, examine, and manipulate direct buffers:
//!
//!  * NewDirectByteBuffer
//!  * GetDirectBufferAddress
//!  * GetDirectBufferCapacity
//!
//! Every implementation of the Java virtual machine must support these functions, but not every implementation is required to support JNI access to direct buffers.
//! If a JVM does not support such access then the `NewDirectByteBuffer` and `GetDirectBufferAddress` functions must always return `NULL`, and the `GetDirectBufferCapacity` function must always return -1.
//! If a JVM does support such access then these three functions must be implemented to return the appropriate values.

use core::ffi::c_void;
use jni::{jlong, jobject, JNIEnv};

extern "system" fn NewDirectByteBuffer(
	env: *mut JNIEnv,
	address: *mut c_void,
	capacity: jlong,
) -> jobject {
	unimplemented!("jni::NewDirectByteBuffer")
}

pub extern "system" fn GetDirectBufferAddress(env: *mut JNIEnv, buf: jobject) -> *mut c_void {
	unimplemented!("jni::GetDirectBufferAddress");
}

pub extern "system" fn GetDirectBufferCapacity(env: *mut JNIEnv, buf: jobject) -> jlong {
	unimplemented!("jni::GetDirectBufferCapacity");
}
