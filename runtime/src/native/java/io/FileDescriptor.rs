#![allow(non_upper_case_globals)]

use crate::objects::class::Class;
use crate::objects::instance::Instance;
use crate::objects::reference::Reference;

use std::sync::atomic::{AtomicBool, Ordering};

use ::jni::env::JniEnv;
use ::jni::sys::{jboolean, jint, jlong};

include_generated!("native/java/io/def/FileDescriptor.definitions.rs");

pub fn get_fd(this: &Reference) -> jint {
	let fd_field_offset = crate::globals::fields::java_io_FileDescriptor::fd_field_offset();
	this.get_field_value0(fd_field_offset).expect_int()
}

// throws SyncFailedException
pub fn sync0(_: JniEnv, _this: Reference) {
	unimplemented!("java.io.FileDescriptor#sync0");
}

pub fn initIDs(_: JniEnv, class: &'static Class) {
	static ONCE: AtomicBool = AtomicBool::new(false);
	if ONCE
		.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
		.is_err()
	{
		panic!("java.io.FileDescriptor#initIDs: attempt to initialize more than once.");
	}

	unsafe {
		crate::globals::classes::set_java_io_FileDescriptor(class);
		crate::globals::fields::java_io_FileDescriptor::init_offsets();
	}
}

#[cfg(windows)]
pub fn getHandle(_: JniEnv, _class: &'static Class, _d: jint) -> jlong {
	unimplemented!("java.io.FileDescriptor#getHandle");
}

// Only windows uses the `handle` field.
#[cfg(unix)]
pub fn getHandle(_: JniEnv, _class: &'static Class, _d: jint) -> jlong {
	-1
}

#[cfg(windows)]
pub fn getAppend(_: JniEnv, _class: &'static Class, _fd: jint) -> jboolean {
	unimplemented!("java.io.FileDescriptor#getAppend");
}

#[cfg(unix)]
pub fn getAppend(_: JniEnv, _class: &'static Class, fd_: jint) -> jboolean {
	use libc::{F_GETFL, O_APPEND};

	let flags = unsafe { libc::fcntl(fd_, F_GETFL) };
	(flags & O_APPEND) == 0
}

// throws IOException
pub fn close0(_: JniEnv, _this: Reference) {
	unimplemented!("java.io.FileDescriptor#close0");
}
