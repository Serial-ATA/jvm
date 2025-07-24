use crate::classes;
use crate::native::java::lang::String::StringInterner;
use crate::objects::class::ClassPtr;
use crate::objects::instance::array::Array;
use crate::objects::instance::object::Object;
use crate::objects::reference::Reference;
use crate::symbols::sym;
use crate::thread::JavaThread;
use crate::thread::exceptions::{Throws, throw, throw_and_return_null};

use std::time::{SystemTime, UNIX_EPOCH};

use ::jni::env::JniEnv;
use ::jni::sys::{jint, jlong};
use instructions::Operand;
use platform::{JNI_LIB_PREFIX, JNI_LIB_SUFFIX};

include_generated!("native/java/lang/def/System.registerNatives.rs");
include_generated!("native/java/lang/def/System.definitions.rs");

pub fn setIn0(
	_: JniEnv,
	class: ClassPtr,
	in_: Reference, // java.io.InputStream
) {
	let field = class
		.fields()
		.find(|field| field.name == sym!(r#in) && field.descriptor.is_class(b"java/io/InputStream"))
		.expect("java/lang/System#in field should exist");
	field.set_static_value(Operand::Reference(in_));
}
pub fn setOut0(
	_env: JniEnv,
	class: ClassPtr,
	out: Reference, // java.io.PrintStream
) {
	let field = class
		.fields()
		.find(|field| field.name == sym!(out) && field.descriptor.is_class(b"java/io/PrintStream"))
		.expect("java/lang/System#out field should exist");
	field.set_static_value(Operand::Reference(out));
}
pub fn setErr0(
	_env: JniEnv,
	class: ClassPtr,
	err: Reference, // java.io.PrintStream
) {
	let field = class
		.fields()
		.find(|field| field.name == sym!(err) && field.descriptor.is_class(b"java/io/PrintStream"))
		.expect("java/lang/System#err field should exist");
	field.set_static_value(Operand::Reference(err));
}

pub fn currentTimeMillis(_env: JniEnv, _class: ClassPtr) -> jlong {
	unimplemented!("System#currentTimeMillis")
}

pub fn nanoTime(_env: JniEnv, _class: ClassPtr) -> jlong {
	let time_nanos = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.expect("current system time should not be before the UNIX epoch")
		.as_nanos();

	time_nanos as jlong
}

pub fn arraycopy(
	env: JniEnv,
	_class: ClassPtr,
	src: Reference, // java.lang.Object
	src_pos: jint,
	dest: Reference, // java.lang.Object
	dest_pos: jint,
	length: jint,
) {
	unsafe fn do_copy<T: Array>(src: T, src_pos: usize, dest: T, dest_pos: usize, length: usize) {
		unsafe {
			src.copy_into(src_pos, &dest, dest_pos, length);
		}
	}

	unsafe fn do_copy_within<T: Array>(src: T, src_pos: usize, dest_pos: usize, length: usize) {
		unsafe {
			src.copy_within(src_pos, dest_pos, length);
		}
	}

	if src.is_null() || dest.is_null() {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		throw!(thread, NullPointerException);
	}

	let src_len = match src.array_length() {
		Throws::Ok(len) => len,
		Throws::Exception(e) => {
			let thread = unsafe { &*JavaThread::for_env(env.raw()) };
			e.throw(thread);
			return;
		},
	};
	let dest_len = match dest.array_length() {
		Throws::Ok(len) => len,
		Throws::Exception(e) => {
			let thread = unsafe { &*JavaThread::for_env(env.raw()) };
			e.throw(thread);
			return;
		},
	};

	// TODO: Verify component types

	if src_pos < 0
		|| dest_pos < 0
		|| length < 0
		|| src_pos + length > src_len as jint
		|| dest_pos + length > dest_len as jint
	{
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		throw!(thread, IndexOutOfBoundsException);
	}

	if length == 0 {
		return;
	}

	if src == dest {
		if src.is_object_array() {
			unsafe {
				do_copy_within(
					src.extract_object_array(),
					src_pos as usize,
					dest_pos as usize,
					length as usize,
				)
			}
		} else {
			unsafe {
				do_copy_within(
					src.extract_primitive_array(),
					src_pos as usize,
					dest_pos as usize,
					length as usize,
				)
			}
		}

		return;
	}

	if src.is_object_array() {
		unsafe {
			do_copy(
				src.extract_object_array(),
				src_pos as usize,
				dest.extract_object_array(),
				dest_pos as usize,
				length as usize,
			)
		}
	} else {
		unsafe {
			do_copy(
				src.extract_primitive_array(),
				src_pos as usize,
				dest.extract_primitive_array(),
				dest_pos as usize,
				length as usize,
			)
		}
	}
}

pub fn identityHashCode(
	env: JniEnv,
	_class: ClassPtr,
	x: Reference, // java.lang.Object
) -> jint {
	crate::native::java::lang::Object::hashCode(env, x)
}

pub fn mapLibraryName(env: JniEnv, _class: ClassPtr, libname: Reference) -> Reference {
	if libname.is_null() {
		let thread = unsafe { &*JavaThread::for_env(env.raw()) };
		throw_and_return_null!(thread, NullPointerException);
	}

	let libname = classes::java::lang::String::extract(libname.extract_class());
	Reference::class(StringInterner::intern(format!(
		"{JNI_LIB_PREFIX}{libname}{JNI_LIB_SUFFIX}"
	)))
}
