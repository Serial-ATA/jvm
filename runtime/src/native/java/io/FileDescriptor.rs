#![allow(non_upper_case_globals)]

use crate::classes;
use crate::objects::class::ClassPtr;
use crate::objects::reference::Reference;

use std::os::fd::FromRawFd;
use std::sync::atomic::{AtomicBool, Ordering};

use ::jni::env::JniEnv;
use ::jni::sys::{jboolean, jint, jlong};

include_generated!("native/java/io/def/FileDescriptor.definitions.rs");

// throws SyncFailedException
pub fn sync0(_: JniEnv, _this: Reference) {
	unimplemented!("java.io.FileDescriptor#sync0");
}

pub fn initIDs(_: JniEnv, class: ClassPtr) {
	static ONCE: AtomicBool = AtomicBool::new(false);
	if ONCE
		.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
		.is_err()
	{
		panic!("java.io.FileDescriptor#initIDs: attempt to initialize more than once.");
	}

	unsafe {
		crate::globals::classes::set_java_io_FileDescriptor(class);
		classes::java::io::FileDescriptor::init_offsets();
	}
}

#[cfg(windows)]
pub fn getHandle(_: JniEnv, _class: ClassPtr, _d: jint) -> jlong {
	unimplemented!("java.io.FileDescriptor#getHandle");
}

// Only windows uses the `handle` field.
#[cfg(unix)]
pub fn getHandle(_: JniEnv, _class: ClassPtr, _d: jint) -> jlong {
	-1
}

#[cfg(windows)]
pub fn getAppend(_: JniEnv, _class: ClassPtr, _fd: jint) -> jboolean {
	unimplemented!("java.io.FileDescriptor#getAppend");
}

#[cfg(unix)]
pub fn getAppend(_: JniEnv, _class: ClassPtr, fd_: jint) -> jboolean {
	use libc::{F_GETFL, O_APPEND};

	let flags = unsafe { libc::fcntl(fd_, F_GETFL) };
	(flags & O_APPEND) == 0
}

// throws IOException
pub fn close0(_: JniEnv, this: Reference) {
	let current_fd = classes::java::io::FileDescriptor::fd(this);

	classes::java::io::FileDescriptor::set_fd(this, -1);

	#[cfg(windows)]
	classes::java::io::FileDescriptor::set_handle(this, -1);

	// Drop impl closes the file
	let _ = unsafe { std::fs::File::from_raw_fd(current_fd) };
}
