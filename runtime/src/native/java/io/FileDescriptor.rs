#![allow(non_upper_case_globals)]

use crate::classpath::classloader::ClassLoader;
use crate::native::jni::IntoJni;
use crate::objects::instance::Instance;
use crate::objects::reference::Reference;

use std::cell::SyncUnsafeCell;
use std::ptr::{self, NonNull};
use std::sync::atomic::{AtomicBool, Ordering};

use ::jni::env::JniEnv;
use ::jni::sys::{jboolean, jfieldID, jint, jlong};
use common::sync::ForceSync;
use symbols::sym;

include_generated!("native/java/io/def/FileDescriptor.definitions.rs");

/// `java.io.FileDescriptor#fd` field offset
static fd: SyncUnsafeCell<ForceSync<jfieldID>> =
	SyncUnsafeCell::new(ForceSync::new(ptr::null_mut() as _));
pub fn get_fd(this: &Reference) -> jint {
	let fd_value = unsafe { &*fd.get() };
	let field = unsafe { crate::native::jni::field_ref_from_jfieldid(fd_value.0) }
		.expect("field should always be present");
	this.get_field_value(field).expect_int()
}

/// `java.io.FileDescriptor#handle` field offset
#[cfg(windows)]
static handle: SyncUnsafeCell<ForceSync<jfieldID>> =
	SyncUnsafeCell::new(ForceSync::new(ptr::null_mut() as _));
/// `java.io.FileDescriptor#append` field offset
static append: SyncUnsafeCell<ForceSync<jfieldID>> =
	SyncUnsafeCell::new(ForceSync::new(ptr::null_mut() as _));

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
	for field in class.fields() {
		match field.name.as_str() {
			"fd" => unsafe {
				assert!(fields & 1 << 3 == 0, "Field can only occur once");
				*fd.get() = ForceSync::new(field.into_jni());
				fields |= 1 << 2;
			},
			#[cfg(windows)]
			"handle" => unsafe {
				*handle.get() = ForceSync::new(field.into_jni());
				fields |= 1 << 1;
			},
			"append" => unsafe {
				*append.get() = ForceSync::new(field.into_jni());
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
