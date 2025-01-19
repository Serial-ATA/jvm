#![allow(non_upper_case_globals)]

use crate::objects::class::Class;
use crate::objects::instance::Instance;
use crate::objects::reference::Reference;
use crate::string_interner::StringInterner;

use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, Ordering};

use ::jni::env::JniEnv;
use ::jni::sys::{jboolean, jint, jlong};
use common::traits::PtrType;
use symbols::sym;

include_generated!("native/java/io/def/UnixFileSystem.definitions.rs");

fn get_file_path(file: Reference) -> String {
	let path_field_offset = crate::globals::fields::java_io_File::path_field_offset();
	let f = file.extract_class();
	let value = f
		.get()
		.get_field_value0(path_field_offset)
		.expect_reference();

	StringInterner::rust_string_from_java_string(value.extract_class())
}

pub fn canonicalize0(
	_: JniEnv,
	_this: Reference,
	path: Reference, // java.lang.String
) -> Reference /* java.lang.String */ {
	if path.is_null() {
		panic!("NullPointerException"); // TODO
	}

	let path_str = StringInterner::rust_string_from_java_string(path.extract_class());

	let Ok(path) = std::path::Path::new(&path_str).canonicalize() else {
		panic!("IOException"); // TODO
	};

	Reference::class(StringInterner::intern_string(
		path.to_string_lossy().into_owned(),
	))
}

#[cfg(unix)]
pub fn getBooleanAttributes0(
	_: JniEnv,
	_this: Reference,
	f: Reference, // java.io.File
) -> jint {
	use super::FileSystem::{BA_DIRECTORY, BA_EXISTS, BA_REGULAR};

	use std::os::unix::fs::MetadataExt;
	let path = get_file_path(f);

	let Ok(metadata) = std::fs::metadata(path) else {
		return 0;
	};

	let mode = metadata.mode();
	let fmt = mode & libc::S_IFMT;

	let mut ret = BA_EXISTS;
	if fmt == libc::S_IFREG {
		ret |= BA_REGULAR;
	}

	if fmt == libc::S_IFDIR {
		ret |= BA_DIRECTORY;
	}

	ret
}

#[cfg(not(unix))]
pub fn getBooleanAttributes0(
	_: JniEnv,
	_this: Reference,
	_f: Reference, // java.io.File
) -> jint {
	0
}

pub fn checkAccess0(
	_: JniEnv,
	_this: Reference,
	_f: Reference, // java.io.File
	_access: jint,
) -> jboolean {
	unimplemented!("java.io.UnixFileSystem#checkAccess0");
}

pub fn getLastModifiedTime0(
	_: JniEnv,
	_this: Reference,
	_f: Reference, // java.io.File
) -> jlong {
	unimplemented!("java.io.UnixFileSystem#getLastModifiedTime0");
}

pub fn getLength0(
	_: JniEnv,
	_this: Reference,
	_f: Reference, // java.io.File
) -> jlong {
	unimplemented!("java.io.UnixFileSystem#getLength0");
}

pub fn setPermission0(
	_: JniEnv,
	_this: Reference,
	_f: Reference, // java.io.File
	_access: jint,
	_enable: jboolean,
	_owneronly: jboolean,
) -> jboolean {
	unimplemented!("java.io.UnixFileSystem#setPermission0");
}

pub fn createFileExclusively0(
	_: JniEnv,
	_this: Reference,
	_path: Reference, // java.lang.String
) -> jboolean {
	unimplemented!("java.io.UnixFileSystem#createFileExclusively0");
}

pub fn delete0(
	_: JniEnv,
	_this: Reference,
	_f: Reference, // java.io.File
) -> jboolean {
	unimplemented!("java.io.UnixFileSystem#delete0");
}

pub fn list0(
	_: JniEnv,
	_this: Reference,
	_f: Reference, // java.io.File
) -> Reference /* java.lang.String[] */ {
	unimplemented!("java.io.UnixFileSystem#list0");
}

pub fn createDirectory0(
	_: JniEnv,
	_this: Reference,
	_f: Reference, // java.io.File
) -> jboolean {
	unimplemented!("java.io.UnixFileSystem#createDirectory0");
}

pub fn rename0(
	_: JniEnv,
	_this: Reference,
	_f1: Reference, // java.io.File
	_f2: Reference, // java.io.File
) -> jboolean {
	unimplemented!("java.io.UnixFileSystem#rename0");
}

pub fn setLastModifiedTime0(
	_: JniEnv,
	_this: Reference,
	_f: Reference, // java.io.File
	_time: jlong,
) -> jboolean {
	unimplemented!("java.io.UnixFileSystem#setLastModifiedTime0");
}

pub fn setReadOnly0(
	_: JniEnv,
	_this: Reference,
	_f: Reference, // java.io.File
) -> jboolean {
	unimplemented!("java.io.UnixFileSystem#setReadOnly0");
}

pub fn getSpace0(
	_: JniEnv,
	_this: Reference,
	_f: Reference, // java.io.File
	_t: jint,
) -> jlong {
	unimplemented!("java.io.UnixFileSystem#getSpace0");
}

pub fn getNameMax0(
	_: JniEnv,
	_this: Reference,
	_path: Reference, // java.lang.String
) -> jlong {
	unimplemented!("java.io.UnixFileSystem#getNameMax0");
}

pub fn initIDs(_: JniEnv, class: &'static Class) {
	static ONCE: AtomicBool = AtomicBool::new(false);
	if ONCE
		.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
		.is_err()
	{
		// TODO
		panic!("java.io.UnixFileSystem#initIDs: attempt to initialize more than once.");
	}

	let file_class = class.loader().load(sym!(java_io_File)).unwrap();
	unsafe {
		crate::globals::classes::set_java_io_File(file_class);
		crate::globals::fields::java_io_File::init_offsets();
	}
}
