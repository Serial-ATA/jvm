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

use crate::classpath::loader::ClassLoader;
use crate::native::jni::references::JObjectExt;
use crate::symbols::sym;
use crate::thread::JavaThread;
use crate::thread::exceptions::{Throws, throw_with_ret};
use crate::{classes, globals};

use core::ffi::c_void;
use std::sync::OnceLock;

use jni::sys::{JNIEnv, jlong, jobject};

fn init(thread: &'static JavaThread) -> bool {
	static ONCE: OnceLock<bool> = OnceLock::new();

	*ONCE.get_or_init(|| {
		fn inner(thread: &'static JavaThread) -> Throws<()> {
			let class = ClassLoader::bootstrap().load(sym!(sun_nio_ch_DirectBuffer))?;
			unsafe {
				globals::classes::set_sun_nio_ch_DirectBuffer(class);
			}

			let class = ClassLoader::bootstrap().load(sym!(java_nio_Buffer))?;
			unsafe {
				globals::classes::set_java_nio_Buffer(class);
				classes::java::nio::Buffer::init_offsets();
			}

			Throws::Ok(())
		}

		match inner(thread) {
			Throws::Ok(_) => true,
			Throws::Exception(e) => {
				e.throw(thread);
				false
			},
		}
	})
}

pub extern "system" fn NewDirectByteBuffer(
	env: *mut JNIEnv,
	address: *mut c_void,
	capacity: jlong,
) -> jobject {
	unimplemented!("jni::NewDirectByteBuffer")
}

#[unsafe(no_mangle)]
pub extern "system" fn GetDirectBufferAddress(env: *mut JNIEnv, buf: jobject) -> *mut c_void {
	let thread = JavaThread::current();
	assert_eq!(thread.env().raw(), env);

	if !init(thread) {
		return std::ptr::null_mut();
	}

	let Some(obj) = (unsafe { buf.to_reference() }) else {
		throw_with_ret!(std::ptr::null_mut(), thread, NullPointerException);
	};

	if !obj.is_instance_of(globals::classes::sun_nio_ch_DirectBuffer()) {
		return std::ptr::null_mut();
	}

	classes::java::nio::Buffer::address(obj.extract_class()) as *mut c_void
}

#[unsafe(no_mangle)]
pub extern "system" fn GetDirectBufferCapacity(env: *mut JNIEnv, buf: jobject) -> jlong {
	let thread = JavaThread::current();
	assert_eq!(thread.env().raw(), env);

	if !init(thread) {
		return 0;
	}

	let Some(obj) = (unsafe { buf.to_reference() }) else {
		return -1;
	};

	if !obj.is_instance_of(globals::classes::sun_nio_ch_DirectBuffer()) {
		return -1;
	}

	classes::java::nio::Buffer::capacity(obj.extract_class()) as jlong
}
