use crate::reference::Reference;

use std::ptr::NonNull;

use ::jni::env::JniEnv;
use ::jni::sys::{jboolean, jint, jlong};

include_generated!("native/java/io/def/FileInputStream.definitions.rs");

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

pub fn initIDs(_: NonNull<JniEnv>) {
	unimplemented!("java.io.FileInputStream#initIDs");
}
