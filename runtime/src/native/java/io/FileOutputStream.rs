use crate::classpath::classloader::ClassLoader;
use crate::native::Reference;

use std::cell::SyncUnsafeCell;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, Ordering};

use ::jni::env::JniEnv;
use ::jni::sys::{jboolean, jint};
use symbols::sym;

include_generated!("native/java/io/def/FileOutputStream.definitions.rs");

/// `java.io.FileInputStream#fd` field offset
static fd: SyncUnsafeCell<usize> = SyncUnsafeCell::new(0);

// throws FileNotFoundException
pub fn open0(_: NonNull<JniEnv>, _this: Reference, _name: Reference /* java.lang.String */) {
	unimplemented!("java.io.FileOutputStream#open0");
}

// throws IOException
pub fn write(_: NonNull<JniEnv>, _this: Reference, _b: jint, _append: jboolean) {
	unimplemented!("java.io.FileOutputStream#write");
}

// throws IOException
pub fn writeBytes(
	_: NonNull<JniEnv>,
	_this: Reference,
	_b: Reference, // byte[]
	_off: jint,
	_len: jint,
	_append: jboolean,
) {
	unimplemented!("java.io.FileOutputStream#write");
}

pub fn initIDs(_: NonNull<JniEnv>) {
	static ONCE: AtomicBool = AtomicBool::new(false);
	if ONCE
		.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
		.is_err()
	{
		// TODO
		panic!("java.io.FileOutputStream#initIDs: attempt to initialize more than once.");
	}

	let class = ClassLoader::lookup_class(sym!(java_io_FileOutputStream)).unwrap();
	unsafe {
		crate::globals::classes::set_java_io_FileOutputStream(class);
	}

	let mut field_set = false;
	for (index, field) in class.fields().enumerate() {
		if field.name == sym!(fd) {
			unsafe {
				*fd.get() = index;
			}
			field_set = true;
			break;
		}
	}

	assert!(field_set, "Field must be present");
}
