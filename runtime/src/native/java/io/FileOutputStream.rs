#![allow(non_upper_case_globals)]

use crate::classes;
use crate::objects::class::ClassPtr;
use crate::objects::reference::Reference;
use crate::thread::JavaThread;
use crate::thread::exceptions::throw;

use std::io::Write;
use std::mem::ManuallyDrop;
use std::os::fd::{FromRawFd, RawFd};
use std::sync::atomic::{AtomicBool, Ordering};

use ::jni::env::JniEnv;
use ::jni::sys::{jboolean, jint};

include_generated!("native/java/io/def/FileOutputStream.definitions.rs");

// throws FileNotFoundException
pub fn open0(_: JniEnv, _this: Reference, _name: Reference /* java.lang.String */) {
	unimplemented!("java.io.FileOutputStream#open0");
}

// throws IOException
pub fn write(_: JniEnv, _this: Reference, _b: jint, _append: jboolean) {
	unimplemented!("java.io.FileOutputStream#write");
}

// throws IOException
#[allow(trivial_numeric_casts)]
pub fn writeBytes(
	env: JniEnv,
	this: Reference,
	b: Reference, // byte[]
	off: jint,
	len: jint,
	_append: jboolean,
) {
	if b.is_null() {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		throw!(thread, NullPointerException);
	}

	let array_instance = b.extract_primitive_array();
	let array_content = array_instance.as_bytes();
	if off < 0 || len < 0 || (off + len) as usize > array_content.len() {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		throw!(thread, IndexOutOfBoundsException);
	}

	if len == 0 {
		return;
	}

	let mut window = &array_content[off as usize..(off + len) as usize];

	let mut offset = 0;
	let mut len = len;
	while len > 0 {
		let current_fd = classes::java::io::FileOutputStream::fd(&this);
		if current_fd == -1 {
			let thread = unsafe { &*JavaThread::for_env(env.raw()) };
			throw!(thread, IOException, "stream closed");
		}

		// Wrap in `ManuallyDrop` so the file descriptor doesn't get closed
		let mut file =
			ManuallyDrop::new(unsafe { std::fs::File::from_raw_fd(current_fd as RawFd) });

		let Ok(n) = file.write(&window[offset..]) else {
			let thread = unsafe { &*JavaThread::for_env(env.raw()) };
			throw!(thread, IOException);
		};

		offset += n;
		len -= n as i32;
		window = &window[n..];
	}
}

pub fn initIDs(_: JniEnv, class: ClassPtr) {
	static ONCE: AtomicBool = AtomicBool::new(false);
	if ONCE
		.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
		.is_err()
	{
		panic!("java.io.FileOutputStream#initIDs: attempt to initialize more than once.");
	}

	unsafe {
		crate::globals::classes::set_java_io_FileOutputStream(class);
		classes::java::io::FileOutputStream::init_offsets();
	}
}
