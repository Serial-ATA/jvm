#![allow(non_upper_case_globals)]

use crate::objects::class::Class;
use crate::objects::instance::Instance;
use crate::objects::reference::Reference;
use crate::thread::JavaThread;

use std::io::Write;
use std::mem::ManuallyDrop;
use std::os::fd::{FromRawFd, RawFd};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::classes;
use ::jni::env::JniEnv;
use ::jni::sys::{jboolean, jint};
use common::traits::PtrType;
use jni::sys::jbyte;

include_generated!("native/java/io/def/FileOutputStream.definitions.rs");

fn get_fd(this: &Reference) -> jint {
	// `fd` is a reference to a `java.io.FileDescriptor`
	let fd_field_offset = classes::java_io_FileOutputStream::fd_field_offset();
	let file_descriptor_ref = this.get_field_value0(fd_field_offset).expect_reference();

	super::FileDescriptor::get_fd(&file_descriptor_ref)
}

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
		let _thread = unsafe { JavaThread::for_env(env.raw()) };
		panic!("NullPointerException"); // TODO
	}

	let array_instance = b.extract_primitive_array();
	let array_content = array_instance.get().as_slice::<jbyte>();
	if off < 0 || len < 0 || (off + len) as usize > array_content.len() {
		let _thread = unsafe { JavaThread::for_env(env.raw()) };
		panic!("IndexOutOfBoundsException"); // TODO
	}

	if len == 0 {
		return;
	}

	// Need to convert the jbyte[] to a &[u8]
	let window = &array_content[off as usize..(off + len) as usize];

	// SAFETY: `i8` and `u8` have the same size and alignment
	let mut window: &[u8] =
		unsafe { std::slice::from_raw_parts(window.as_ptr() as *const u8, window.len()) };

	let mut offset = 0;
	let mut len = len;
	while len > 0 {
		let current_fd = get_fd(&this);
		if current_fd == -1 {
			let _thread = unsafe { JavaThread::for_env(env.raw()) };
			panic!("IOException, stream closed"); // TODO
		}

		// Wrap in `ManuallyDrop` so the file descriptor doesn't get closed
		let mut file =
			ManuallyDrop::new(unsafe { std::fs::File::from_raw_fd(current_fd as RawFd) });

		let Ok(n) = file.write(&window[offset..]) else {
			let _thread = unsafe { JavaThread::for_env(env.raw()) };
			panic!("IOException"); // TODO
		};

		offset += n;
		len -= n as i32;
		window = &window[n..];
	}
}

pub fn initIDs(_: JniEnv, class: &'static Class) {
	static ONCE: AtomicBool = AtomicBool::new(false);
	if ONCE
		.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
		.is_err()
	{
		panic!("java.io.FileOutputStream#initIDs: attempt to initialize more than once.");
	}

	unsafe {
		crate::globals::classes::set_java_io_FileOutputStream(class);
		classes::java_io_FileOutputStream::init_offsets();
	}
}
