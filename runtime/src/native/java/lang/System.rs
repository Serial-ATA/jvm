use crate::objects::class::Class;
use crate::objects::reference::Reference;
use crate::thread::JavaThread;

use std::ptr::NonNull;
use std::time::{SystemTime, UNIX_EPOCH};

use ::jni::env::JniEnv;
use ::jni::sys::{jint, jlong};
use common::traits::PtrType;
use instructions::Operand;
use symbols::sym;

include_generated!("native/java/lang/def/System.registerNatives.rs");
include_generated!("native/java/lang/def/System.definitions.rs");

pub fn setIn0(
	_: JniEnv,
	class: &'static Class,
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
	class: &'static Class,
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
	class: &'static Class,
	err: Reference, // java.io.PrintStream
) {
	let field = class
		.fields()
		.find(|field| field.name == sym!(err) && field.descriptor.is_class(b"java/io/PrintStream"))
		.expect("java/lang/System#err field should exist");
	field.set_static_value(Operand::Reference(err));
}

pub fn currentTimeMillis(_env: JniEnv, _class: &'static Class) -> jlong {
	unimplemented!("System#currentTimeMillis")
}

pub fn nanoTime(_env: JniEnv, _class: &'static Class) -> jlong {
	let time_nanos = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.expect("current system time should not be before the UNIX epoch")
		.as_nanos();

	time_nanos as jlong
}

pub fn arraycopy(
	env: JniEnv,
	_class: &'static Class,
	src: Reference, // java.lang.Object
	src_pos: jint,
	dest: Reference, // java.lang.Object
	dest_pos: jint,
	length: jint,
) {
	if src.is_null() || dest.is_null() {
		let _thread = unsafe { &*JavaThread::for_env(env.raw()) };
		todo!("NullPointerException")
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

pub fn identityHashCode(
	env: JniEnv,
	_class: &'static Class,
	x: Reference, // java.lang.Object
) -> jint {
	crate::native::java::lang::Object::hashCode(env, x)
}

pub fn mapLibraryName(_env: JniEnv, _class: &'static Class, _libname: Reference) -> Reference {
	unimplemented!("System#mapLibraryName")
}
