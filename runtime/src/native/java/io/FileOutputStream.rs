#![allow(non_upper_case_globals)]

use crate::native::jni::{field_ref_from_jfieldid, IntoJni};
use crate::native::Reference;
use crate::objects::class::Class;
use crate::objects::instance::Instance;
use crate::thread::JavaThread;

use std::cell::SyncUnsafeCell;
use std::io::Write;
use std::os::fd::{FromRawFd, RawFd};
use std::ptr::{self, NonNull};
use std::sync::atomic::{AtomicBool, Ordering};

use ::jni::env::JniEnv;
use ::jni::sys::{jboolean, jfieldID, jint};
use common::sync::ForceSync;
use common::traits::PtrType;
use symbols::sym;

include_generated!("native/java/io/def/FileOutputStream.definitions.rs");

/// `java.io.FileOutputStream#fd` field offset
static fd: SyncUnsafeCell<ForceSync<jfieldID>> =
	SyncUnsafeCell::new(ForceSync::new(ptr::null_mut() as _));
fn get_fd(this: &Reference) -> jint {
	// `fd` is a reference to a `java.io.FileDescriptor`
	let fd_value = unsafe { &*fd.get() };
	let field =
		unsafe { field_ref_from_jfieldid(fd_value.0) }.expect("field should always be present");
	let file_descriptor_ref = this.get_field_value(field).expect_reference();

	super::FileDescriptor::get_fd(&file_descriptor_ref)
}

// throws FileNotFoundException
pub fn open0(_: NonNull<JniEnv>, _this: Reference, _name: Reference /* java.lang.String */) {
	unimplemented!("java.io.FileOutputStream#open0");
}

// throws IOException
pub fn write(_: NonNull<JniEnv>, _this: Reference, _b: jint, _append: jboolean) {
	unimplemented!("java.io.FileOutputStream#write");
}

// throws IOException
#[allow(trivial_numeric_casts)]
pub fn writeBytes(
	env: NonNull<JniEnv>,
	this: Reference,
	b: Reference, // byte[]
	off: jint,
	len: jint,
	_append: jboolean,
) {
	if b.is_null() {
		let _thread = unsafe { JavaThread::for_env(env.as_ptr()) };
		panic!("NullPointerException"); // TODO
	}

	let array_instance = b.extract_array();
	let array_content = array_instance.get().get_content().expect_byte();
	if off < 0 || len < 0 || (off + len) as usize > array_content.len() {
		let _thread = unsafe { JavaThread::for_env(env.as_ptr()) };
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
			let _thread = unsafe { JavaThread::for_env(env.as_ptr()) };
			panic!("IOException, stream closed"); // TODO
		}

		let mut file = unsafe { std::fs::File::from_raw_fd(current_fd as RawFd) };

		let Ok(n) = file.write(&window[offset..]) else {
			let _thread = unsafe { JavaThread::for_env(env.as_ptr()) };
			panic!("IOException"); // TODO
		};

		offset += n;
		len -= n as i32;
		window = &window[n..];
	}
}

// TODO: Move logic to globals
pub fn initIDs(_: NonNull<JniEnv>, class: &'static Class) {
	static ONCE: AtomicBool = AtomicBool::new(false);
	if ONCE
		.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
		.is_err()
	{
		// TODO
		panic!("java.io.FileOutputStream#initIDs: attempt to initialize more than once.");
	}

	unsafe {
		crate::globals::classes::set_java_io_FileOutputStream(class);
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
