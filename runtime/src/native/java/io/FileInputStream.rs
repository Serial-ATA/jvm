#![allow(non_upper_case_globals)]

use crate::native::java::lang::String::StringInterner;
use crate::objects::class::ClassPtr;
use crate::objects::instance::array::Array;
use crate::objects::reference::Reference;
use crate::thread::JavaThread;
use crate::thread::exceptions::{throw, throw_with_ret};
use crate::{classes, native};

use std::fs;
use std::io::{Read, Seek};
use std::mem::ManuallyDrop;
use std::os::fd::{AsRawFd, FromRawFd};
use std::sync::atomic::{AtomicBool, Ordering};

use ::jni::env::JniEnv;
use ::jni::sys::{jboolean, jint, jlong};

include_generated!("native/java/io/def/FileInputStream.definitions.rs");

// throws FileNotFoundException
pub fn open0(env: JniEnv, this: Reference, name: Reference /* java.lang.String */) {
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	assert_eq!(thread.env(), env);

	if name.is_null() {
		throw!(thread, NullPointerException);
	}

	let path = classes::java::lang::String::extract(name.extract_class());

	let file;
	match fs::OpenOptions::new().read(true).open(path) {
		Ok(f) => file = ManuallyDrop::new(f),
		Err(e) => {
			let path_jstring = thread.jni_refs().allocate_wrapped(name);
			let reason = Reference::class(StringInterner::intern(e.to_string()));
			let reason_jstring = thread.jni_refs().allocate_wrapped(reason);

			if let Some(exception) = native::class::construct_class(
				thread,
				"java/io/FileNotFoundException",
				"(Ljava/lang/String;Ljava/lang/String;)V",
				[path_jstring, reason_jstring],
			) {
				thread.set_pending_exception(exception);
			}

			return;
		},
	}

	classes::java::io::FileInputStream::set_fd(this, file.as_raw_fd())
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
	let thread = unsafe { &*JavaThread::for_env(env.raw()) };
	assert_eq!(thread.env(), env);

	if b.is_null() {
		throw_with_ret!(0, thread, NullPointerException);
	}

	let b = b.extract_primitive_array();
	if off < 0 || len < 0 || (off + len) as usize > b.len() {
		throw_with_ret!(0, thread, IndexOutOfBoundsException);
	}

	if len == 0 {
		return 0;
	}

	// Need to convert the jbyte[] to a &[u8]
	let window = &mut b.as_bytes_mut()[off as usize..(off + len) as usize];

	let current_fd = classes::java::io::FileInputStream::fd(this);
	if current_fd == -1 {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		throw_with_ret!(-1, thread, IOException, "stream closed");
	}

	// Wrap in `ManuallyDrop` so the file descriptor doesn't get closed
	let mut file = ManuallyDrop::new(unsafe { fs::File::from_raw_fd(current_fd) });

	match file.read(window) {
		Ok(n) => n as jint,
		Err(e) => {
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
	let current_fd = classes::java::io::FileInputStream::fd(this);
	if current_fd == -1 {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		throw_with_ret!(0, thread, IOException, "stream closed");
	}

	// Wrap in `ManuallyDrop` so the file descriptor doesn't get closed
	let mut file = ManuallyDrop::new(unsafe { fs::File::from_raw_fd(current_fd) });

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

pub fn initIDs(_: JniEnv, class: ClassPtr) {
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
