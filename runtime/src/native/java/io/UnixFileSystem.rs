#![allow(non_upper_case_globals)]

use crate::classpath::classloader::ClassLoader;
use crate::native::jni::jfieldid_from_field_ref;
use crate::objects::reference::Reference;

use std::cell::SyncUnsafeCell;
use std::ptr::{self, NonNull};
use std::sync::atomic::{AtomicBool, Ordering};

use ::jni::env::JniEnv;
use ::jni::sys::{jboolean, jfieldID, jint, jlong};
use common::sync::ForceSync;
use symbols::sym;

include_generated!("native/java/io/def/UnixFileSystem.definitions.rs");

/// `java.io.File#path` field offset
static path: SyncUnsafeCell<ForceSync<jfieldID>> =
	SyncUnsafeCell::new(ForceSync::new(ptr::null_mut() as _));

pub fn canonicalize0(
	_: NonNull<JniEnv>,
	_this: Reference,
	_path: Reference, // java.lang.String
) -> Reference {
	unimplemented!("java.io.UnixFileSystem#canonicalize0");
}

pub fn getBooleanAttributes0(
	_: NonNull<JniEnv>,
	_this: Reference,
	_f: Reference, // java.io.File
) -> jint {
	unimplemented!("java.io.UnixFileSystem#getBooleanAttributes0");
}

pub fn checkAccess0(
	_: NonNull<JniEnv>,
	_this: Reference,
	_f: Reference, // java.io.File
	_access: jint,
) -> jboolean {
	unimplemented!("java.io.UnixFileSystem#checkAccess0");
}

pub fn getLastModifiedTime0(
	_: NonNull<JniEnv>,
	_this: Reference,
	_f: Reference, // java.io.File
) -> jlong {
	unimplemented!("java.io.UnixFileSystem#getLastModifiedTime0");
}

pub fn getLength0(
	_: NonNull<JniEnv>,
	_this: Reference,
	_f: Reference, // java.io.File
) -> jlong {
	unimplemented!("java.io.UnixFileSystem#getLength0");
}

pub fn setPermission0(
	_: NonNull<JniEnv>,
	_this: Reference,
	_f: Reference, // java.io.File
	_access: jint,
	_enable: jboolean,
	_owneronly: jboolean,
) -> jboolean {
	unimplemented!("java.io.UnixFileSystem#setPermission0");
}

pub fn createFileExclusively0(
	_: NonNull<JniEnv>,
	_this: Reference,
	_path: Reference, // java.lang.String
) -> jboolean {
	unimplemented!("java.io.UnixFileSystem#createFileExclusively0");
}

pub fn delete0(
	_: NonNull<JniEnv>,
	_this: Reference,
	_f: Reference, // java.io.File
) -> jboolean {
	unimplemented!("java.io.UnixFileSystem#delete0");
}

pub fn list0(
	_: NonNull<JniEnv>,
	_this: Reference,
	_f: Reference, // java.io.File
) -> Reference {
	unimplemented!("java.io.UnixFileSystem#list0");
}

pub fn createDirectory0(
	_: NonNull<JniEnv>,
	_this: Reference,
	_f: Reference, // java.io.File
) -> jboolean {
	unimplemented!("java.io.UnixFileSystem#createDirectory0");
}

pub fn rename0(
	_: NonNull<JniEnv>,
	_this: Reference,
	_f1: Reference, // java.io.File
	_f2: Reference, // java.io.File
) -> jboolean {
	unimplemented!("java.io.UnixFileSystem#rename0");
}

pub fn setLastModifiedTime0(
	_: NonNull<JniEnv>,
	_this: Reference,
	_f: Reference, // java.io.File
	_time: jlong,
) -> jboolean {
	unimplemented!("java.io.UnixFileSystem#setLastModifiedTime0");
}

pub fn setReadOnly0(
	_: NonNull<JniEnv>,
	_this: Reference,
	_f: Reference, // java.io.File
) -> jboolean {
	unimplemented!("java.io.UnixFileSystem#setReadOnly0");
}

pub fn getSpace0(
	_: NonNull<JniEnv>,
	_this: Reference,
	_f: Reference, // java.io.File
	_t: jint,
) -> jlong {
	unimplemented!("java.io.UnixFileSystem#getSpace0");
}

pub fn getNameMax0(
	_: NonNull<JniEnv>,
	_this: Reference,
	_path: Reference, // java.lang.String
) -> jlong {
	unimplemented!("java.io.UnixFileSystem#getNameMax0");
}

pub fn initIDs(_: NonNull<JniEnv>) {
	static ONCE: AtomicBool = AtomicBool::new(false);
	if ONCE
		.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
		.is_err()
	{
		// TODO
		panic!("java.io.UnixFileSystem#initIDs: attempt to initialize more than once.");
	}

	let class = ClassLoader::lookup_class(sym!(java_io_File)).unwrap();
	unsafe {
		crate::globals::classes::set_java_io_File(class);
	}

	let mut field_set = false;
	for field in class.fields() {
		if field.name == sym!(path) {
			unsafe {
				*path.get() = ForceSync::new(jfieldid_from_field_ref(field));
			}
			field_set = true;
			break;
		}
	}

	assert!(field_set, "Field must be present");
}
