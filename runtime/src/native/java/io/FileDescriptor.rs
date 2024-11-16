use crate::classpath::classloader::ClassLoader;
use crate::reference::{ClassRef, Reference};

use std::cell::UnsafeCell;
use std::ptr::NonNull;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::SeqCst;

use ::jni::env::JniEnv;
use ::jni::sys::{jboolean, jint, jlong};
use symbols::sym;

include_generated!("native/java/io/def/FileDescriptor.definitions.rs");

static mut fd: UnsafeCell<usize> = UnsafeCell::new(0);
#[cfg(windows)]
static mut handle: UnsafeCell<usize> = UnsafeCell::new(0);
static mut append: UnsafeCell<usize> = UnsafeCell::new(0);

// throws SyncFailedException
pub fn sync0(_: NonNull<JniEnv>, _this: Reference) {
	unimplemented!("java.io.FileDescriptor#sync0");
}

pub fn initIDs(_: NonNull<JniEnv>) {
	static ONCE: AtomicBool = AtomicBool::new(false);
	if ONCE.compare_exchange(false, true, SeqCst, SeqCst).is_err() {
		// TODO
		panic!("java.io.FileDescriptor#initIDs: attempt to initialize more than once.");
	}

	let class = ClassLoader::lookup_class(sym!(java_io_FileDescriptor)).unwrap();
	unsafe {
		crate::globals::classes::set_java_io_FileDescriptor(ClassRef::clone(&class));
	}

	for (index, field) in class.fields().enumerate() {
		match &*field.name {
			b"fd" => unsafe {
				*fd.get_mut() = index;
			},
			#[cfg(windows)]
			b"handle" => unsafe {
				*handle.get_mut() = index;
			},
			b"append" => unsafe {
				*append.get_mut() = index;
			},
			_ => {},
		}
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
