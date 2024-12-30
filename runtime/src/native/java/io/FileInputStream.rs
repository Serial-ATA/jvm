#![allow(non_upper_case_globals)]

use crate::classpath::classloader::ClassLoader;
use crate::native::jni::IntoJni;
use crate::objects::reference::Reference;

use std::cell::SyncUnsafeCell;
use std::ptr::{self, NonNull};
use std::sync::atomic::{AtomicBool, Ordering};

use ::jni::env::JniEnv;
use ::jni::sys::{jboolean, jfieldID, jint, jlong};
use common::sync::ForceSync;
use symbols::sym;

include_generated!("native/java/io/def/FileInputStream.definitions.rs");

/// `java.io.FileInputStream#fd` field offset
static fd: SyncUnsafeCell<ForceSync<jfieldID>> =
	SyncUnsafeCell::new(ForceSync::new(ptr::null_mut() as _));

// throws FileNotFoundException
pub fn open0(_: NonNull<JniEnv>, _this: Reference, _name: Reference /* java.lang.String */) {
	unimplemented!("java.io.FileInputStream#open0");
}

// throws IOException
pub fn read0(_: NonNull<JniEnv>, _this: Reference) -> jint {
	unimplemented!("java.io.FileInputStream#read0");
}

// throws IOException
pub fn readBytes(
	_: NonNull<JniEnv>,
	_this: Reference,
	_b: Reference, // byte[]
	_off: jint,
	_len: jint,
) -> jint {
	unimplemented!("java.io.FileInputStream#readbytes");
}

// throws IOException
pub fn length0(_: NonNull<JniEnv>, _this: Reference) -> jlong {
	unimplemented!("java.io.FileInputStream#length0");
}

// throws IOException
pub fn position0(_: NonNull<JniEnv>, _this: Reference) -> jlong {
	unimplemented!("java.io.FileInputStream#position0");
}

// throws IOException
pub fn skip0(_: NonNull<JniEnv>, _this: Reference, _n: jlong) -> jlong {
	unimplemented!("java.io.FileInputStream#skip0");
}

// throws IOException
pub fn available0(_: NonNull<JniEnv>, _this: Reference) -> jint {
	unimplemented!("java.io.FileInputStream#available0");
}

pub fn isRegularFile0(
	_: NonNull<JniEnv>,
	_this: Reference,
	_fd: Reference, // java.io.FileDescriptor
) -> jboolean {
	unimplemented!("java.io.FileInputStream#isRegularFile0");
}

// TODO: Move logic to globals
pub fn initIDs(_: NonNull<JniEnv>) {
	static ONCE: AtomicBool = AtomicBool::new(false);
	if ONCE
		.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
		.is_err()
	{
		// TODO
		panic!("java.io.FileInputStream#initIDs: attempt to initialize more than once.");
	}

	let class = ClassLoader::lookup_class(sym!(java_io_FileInputStream)).unwrap();
	unsafe {
		crate::globals::classes::set_java_io_FileInputStream(class);
	}

	let mut field_set = false;
	for field in class.fields() {
		if field.name == sym!(fd) {
			unsafe {
				*fd.get() = ForceSync::new(field.into_jni());
			}
			field_set = true;
			break;
		}
	}

	assert!(field_set, "Field must be present");
}
