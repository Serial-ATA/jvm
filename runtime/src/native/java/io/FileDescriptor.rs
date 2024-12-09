#![allow(non_upper_case_globals)]

use crate::classpath::classloader::ClassLoader;
use crate::reference::Reference;

use std::cell::SyncUnsafeCell;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, Ordering};

use ::jni::env::JniEnv;
use ::jni::sys::{jboolean, jint, jlong};
use symbols::sym;

include_generated!("native/java/io/def/FileDescriptor.definitions.rs");

/// `java.io.FileDescriptor#fd` field offset
static fd: SyncUnsafeCell<usize> = SyncUnsafeCell::new(0);
/// `java.io.FileDescriptor#handle` field offset
#[cfg(windows)]
static handle: SyncUnsafeCell<usize> = SyncUnsafeCell::new(0);
/// `java.io.FileDescriptor#append` field offset
static append: SyncUnsafeCell<usize> = SyncUnsafeCell::new(0);

// throws SyncFailedException
pub fn sync0(_: NonNull<JniEnv>, _this: Reference) {
	unimplemented!("java.io.FileDescriptor#sync0");
}

pub fn initIDs(_: NonNull<JniEnv>) {
	static ONCE: AtomicBool = AtomicBool::new(false);
	if ONCE
		.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
		.is_err()
	{
		// TODO
		panic!("java.io.FileDescriptor#initIDs: attempt to initialize more than once.");
	}

	let class = ClassLoader::lookup_class(sym!(java_io_FileDescriptor)).unwrap();
	unsafe {
		crate::globals::classes::set_java_io_FileDescriptor(class);
	}

	let mut fields = 0;
	for (index, field) in class.fields().enumerate() {
		match field.name.as_str() {
			"fd" => unsafe {
				assert!(fields & 1 << 3 == 0, "Field can only occur once");
				*fd.get() = index;
				fields |= 1 << 2;
			},
			#[cfg(windows)]
			"handle" => unsafe {
				*handle.get() = index;
				fields |= 1 << 1;
			},
			"append" => unsafe {
				*append.get() = index;
				fields |= 1;
			},
			_ => {},
		}
	}

	if cfg!(windows) {
		assert_eq!(fields, 0b111, "All fields must be present");
	} else {
		assert_eq!(fields, 0b101, "All fields must be present");
	}
}

#[cfg(windows)]
pub fn getHandle(_: NonNull<JniEnv>, _d: jint) -> jlong {
	unimplemented!("java.io.FileDescriptor#getHandle");
}

// Only windows uses the `handle` field.
#[cfg(unix)]
pub fn getHandle(_: NonNull<JniEnv>, _d: jint) -> jlong {
	-1
}

#[cfg(windows)]
pub fn getAppend(_: NonNull<JniEnv>, _fd: jint) -> jboolean {
	unimplemented!("java.io.FileDescriptor#getAppend");
}

#[cfg(unix)]
pub fn getAppend(_: NonNull<JniEnv>, fd_: jint) -> jboolean {
	use libc::{F_GETFL, O_APPEND};

	let flags = unsafe { libc::fcntl(fd_, F_GETFL) };
	(flags & O_APPEND) == 0
}

// throws IOException
pub fn close0(_: NonNull<JniEnv>, _this: Reference) {
	unimplemented!("java.io.FileDescriptor#close0");
}
