use super::{classref_from_jclass, IntoJni};
use crate::classpath::loader::ClassLoader;
use crate::objects::class::Class;
use crate::thread::exceptions::{throw_with_ret, Throws};
use crate::thread::JavaThread;

use core::ffi::{c_char, CStr};

use ::jni::sys::{jboolean, jbyte, jclass, jobject, jsize, JNIEnv};
use common::traits::PtrType;
use symbols::Symbol;

#[no_mangle]
pub unsafe extern "system" fn DefineClass(
	env: *mut JNIEnv,
	name: *const c_char,
	loader: jobject,
	buf: *const jbyte,
	len: jsize,
) -> jclass {
	unimplemented!("jni::DefineClass")
}

#[no_mangle]
pub unsafe extern "system" fn FindClass(env: *mut JNIEnv, name: *const c_char) -> jclass {
	let thread = JavaThread::current();
	assert_eq!(thread.env().raw(), env);

	let name = unsafe { CStr::from_ptr(name) };

	// Initially assume the system loader
	let mut loader = ClassLoader::bootstrap();

	let frame_stack = thread.frame_stack();
	if let Some(current_frame) = frame_stack.current() {
		// If we can find a caller frame, use its loader instead
		loader = current_frame.method().class().loader();
	}

	match loader.load(Symbol::intern_bytes(name.to_bytes())) {
		Throws::Ok(class) => return class.into_jni(),
		Throws::Exception(e) => e.throw(thread),
	}

	let ret = core::ptr::null::<&'static Class>() as jclass;
	throw_with_ret!(ret, thread, NoClassDefFoundError, name.to_string_lossy());
}

#[no_mangle]
pub unsafe extern "system" fn GetSuperclass(env: *mut JNIEnv, sub: jclass) -> jclass {
	// Comments from https://github.com/openjdk/jdk/blob/6c59185475eeca83153f085eba27cc0b3acf9bb4/src/java.base/share/classes/java/lang/Class.java#L1034-L1044

	let Some(sub) = (unsafe { classref_from_jclass(sub) }) else {
		panic!("Invalid arguments to `GetSuperclass`");
	};

	// If this `Class` represents either:
	// * the `Object` class
	if sub == crate::globals::classes::java_lang_Object()
        // * an interface
        || sub.is_interface()
        // * a primitive type, or void
        || sub.mirror().get().is_primitive()
	{
		// then null is returned
		return core::ptr::null::<&'static Class>() as jclass;
	}

	// If this `Class` object represents an array class
	if sub.is_array() {
		// then the `Class` object representing the `Object` class is returned
		return crate::globals::classes::java_lang_Object().into_jni();
	}

	if let Some(super_class) = sub.super_class {
		return super_class.into_jni();
	}

	return core::ptr::null::<&'static Class>() as jclass;
}

#[no_mangle]
pub unsafe extern "system" fn IsAssignableFrom(
	env: *mut JNIEnv,
	sub: jclass,
	sup: jclass,
) -> jboolean {
	let sub = unsafe { classref_from_jclass(sub) };
	let sup = unsafe { classref_from_jclass(sup) };

	let (Some(sub), Some(sup)) = (sub, sup) else {
		panic!("Invalid arguments to `IsAssignableFrom`");
	};

	if sub.mirror().get().is_primitive() && sup.mirror().get().is_primitive() {
		return sub == sup;
	}

	return sub.is_subclass_of(sup);
}
