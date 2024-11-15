use crate::reference::Reference;

use std::ptr::NonNull;
use std::time::{SystemTime, UNIX_EPOCH};

use ::jni::env::JniEnv;
use ::jni::sys::{jint, jlong};
use common::traits::PtrType;

include_generated!("native/java/lang/def/System.registerNatives.rs");
include_generated!("native/java/lang/def/System.definitions.rs");

pub fn setIn0(_: NonNull<JniEnv>, _in_: Reference /* java.io.PrintStream */) {
	unimplemented!("System#setIn0")
}
pub fn setOut0(_env: NonNull<JniEnv>, _out: Reference /* java.io.PrintStream */) {
	unimplemented!("System#setOut0")
}
pub fn setErr0(_env: NonNull<JniEnv>, _err: Reference /* java.io.PrintStream */) {
	unimplemented!("System#setErr0")
}

pub fn currentTimeMillis(_env: NonNull<JniEnv>) -> jlong {
	unimplemented!("System#currentTimeMillis")
}

pub fn nanoTime(_env: NonNull<JniEnv>) -> jlong {
	let time_nanos = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.expect("current system time should not be before the UNIX epoch")
		.as_nanos();

	time_nanos as jlong
}

pub fn arraycopy(
	_env: NonNull<JniEnv>,
	src: Reference, // java.lang.Object
	src_pos: jint,
	dest: Reference, // java.lang.Object
	dest_pos: jint,
	length: jint,
) {
	if src.is_null() || dest.is_null() {
		// TODO
		panic!("NullPointerException")
	}

	let src_array = src.extract_array();
	let dest_array = dest.extract_array();

	if src_pos < 0
		|| dest_pos < 0
		|| length < 0
		|| src_pos + length > src_array.get().elements.element_count() as jint
		|| dest_pos + length > dest_array.get().elements.element_count() as jint
	{
		// TODO
		panic!("IndexOutOfBoundsException")
	}

	if src_array.as_raw() == dest_array.as_raw() {
		unimplemented!("arraycopy on same instance")
	}

	src_array.get().elements.copy_into(
		src_pos as usize,
		&mut dest_array.get_mut().elements,
		dest_pos as usize,
		length as usize,
	);
}

pub fn identityHashCode(_env: NonNull<JniEnv>, _x: Reference /* java.lang.Object */) -> jlong {
	unimplemented!("System#identityHashCode")
}

pub fn mapLibraryName(_env: NonNull<JniEnv>, _libname: Reference) -> Reference {
	unimplemented!("System#mapLibraryName")
}
