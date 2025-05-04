#![allow(non_upper_case_globals)]

use crate::classes;
use crate::objects::array::Array;
use crate::objects::class::Class;
use crate::objects::reference::Reference;
use crate::thread::exceptions::throw_with_ret;
use crate::thread::JavaThread;

use std::io::{Read, Seek};
use std::mem::ManuallyDrop;
use std::os::fd::{FromRawFd, RawFd};
use std::sync::atomic::{AtomicBool, Ordering};

use ::jni::env::JniEnv;
use ::jni::sys::{jboolean, jint, jlong};
use common::traits::PtrType;

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
	env: JniEnv,
	this: Reference,
	b: Reference, // byte[]
	off: jint,
	len: jint,
) -> jint {
	if b.is_null() {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		throw_with_ret!(0, thread, NullPointerException);
	}

	let b = b.extract_primitive_array();
	if off < 0 || len < 0 || (off + len) as usize > b.get().len() {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		throw_with_ret!(0, thread, IndexOutOfBoundsException);
	}

	if len == 0 {
		return 0;
	}

	// Need to convert the jbyte[] to a &[u8]
	let mut window = &mut b.get_mut().as_bytes_mut()[off as usize..(off + len) as usize];

	let current_fd = classes::java::io::FileInputStream::fd(&this);
	if current_fd == -1 {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		throw_with_ret!(-1, thread, IOException, "stream closed");
	}

	// Wrap in `ManuallyDrop` so the file descriptor doesn't get closed
	let mut file = ManuallyDrop::new(unsafe { std::fs::File::from_raw_fd(current_fd as RawFd) });

	match file.read(&mut window[off as usize..]) {
		Ok(n) => n as jint,
		Err(e) => {
			let thread = unsafe { &*JavaThread::for_env(env.raw()) };
			throw_with_ret!(-1, thread, IOException, "{e}");
		},
	}
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
pub fn available0(env: JniEnv, this: Reference) -> jint {
	let current_fd = classes::java::io::FileInputStream::fd(&this);
	if current_fd == -1 {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		throw_with_ret!(0, thread, IOException, "stream closed");
	}

	// Wrap in `ManuallyDrop` so the file descriptor doesn't get closed
	let mut file = ManuallyDrop::new(unsafe { std::fs::File::from_raw_fd(current_fd as RawFd) });

	let Ok(current) = file.stream_position() else {
		return 0;
	};

	let Ok(metadata) = file.metadata() else {
		return 0;
	};

	(metadata.len() - current) as jint
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
		panic!("java.io.FileInputStream#initIDs: attempt to initialize more than once.");
	}

	unsafe {
		crate::globals::classes::set_java_io_FileInputStream(class);
		classes::java::io::FileInputStream::init_offsets();
	}
}
