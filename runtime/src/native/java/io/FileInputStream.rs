#![allow(non_upper_case_globals)]

use crate::objects::class::Class;
use crate::objects::reference::Reference;

use std::sync::atomic::{AtomicBool, Ordering};

use ::jni::env::JniEnv;
use ::jni::sys::{jboolean, jint, jlong};

include_generated!("native/java/io/def/FileInputStream.definitions.rs");

// throws FileNotFoundException
pub fn open0(_: JniEnv, _this: Reference, _name: Reference /* java.lang.String */) {
	unimplemented!("java.io.FileInputStream#open0");
}

// throws IOException
pub fn read0(_: JniEnv, _this: Reference) -> jint {
	unimplemented!("java.io.FileInputStream#read0");
}

// throws IOException
pub fn readBytes(
	_: JniEnv,
	_this: Reference,
	_b: Reference, // byte[]
	_off: jint,
	_len: jint,
) -> jint {
	unimplemented!("java.io.FileInputStream#readbytes");
}

// throws IOException
pub fn length0(_: JniEnv, _this: Reference) -> jlong {
	unimplemented!("java.io.FileInputStream#length0");
}

// throws IOException
pub fn position0(_: JniEnv, _this: Reference) -> jlong {
	unimplemented!("java.io.FileInputStream#position0");
}

// throws IOException
pub fn skip0(_: JniEnv, _this: Reference, _n: jlong) -> jlong {
	unimplemented!("java.io.FileInputStream#skip0");
}

// throws IOException
pub fn available0(_: JniEnv, _this: Reference) -> jint {
	unimplemented!("java.io.FileInputStream#available0");
}

pub fn isRegularFile0(
	_: JniEnv,
	_this: Reference,
	_fd: Reference, // java.io.FileDescriptor
) -> jboolean {
	unimplemented!("java.io.FileInputStream#isRegularFile0");
}

pub fn initIDs(_: JniEnv, class: &'static Class) {
	static ONCE: AtomicBool = AtomicBool::new(false);
	if ONCE
		.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
		.is_err()
	{
		// TODO
		panic!("java.io.FileInputStream#initIDs: attempt to initialize more than once.");
	}

	unsafe {
		crate::globals::classes::set_java_io_FileInputStream(class);
		crate::globals::fields::java_io_FileInputStream::init_offsets();
	}
}
