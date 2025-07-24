use super::{IntoJni, reference_from_jobject};
use crate::classpath::loader::ClassLoader;
use crate::objects::class::Class;
use crate::objects::reference::Reference;
use crate::symbols::Symbol;
use crate::thread::JavaThread;
use crate::thread::exceptions::{Throws, throw_with_ret};

use core::ffi::{CStr, c_char};

use ::jni::sys::{JNIEnv, jboolean, jbyte, jclass, jobject, jsize};

#[unsafe(no_mangle)]
pub unsafe extern "system" fn DefineClass(
	env: *mut JNIEnv,
	name: *const c_char,
	loader: jobject,
	buf: *const jbyte,
	len: jsize,
) -> jclass {
	unimplemented!("jni::DefineClass")
}

#[unsafe(no_mangle)]
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

	match loader.load(Symbol::intern(name.to_bytes())) {
		Throws::Ok(class) => return class.into_jni(),
		Throws::Exception(e) => e.throw(thread),
	}

	let ret = core::ptr::null::<Reference>() as jclass;
	throw_with_ret!(
		ret,
		thread,
		NoClassDefFoundError,
		"{}",
		name.to_string_lossy()
	);
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetSuperclass(env: *mut JNIEnv, sub: jclass) -> jclass {
	// Comments from https://github.com/openjdk/jdk/blob/6c59185475eeca83153f085eba27cc0b3acf9bb4/src/java.base/share/classes/java/lang/Class.java#L1034-L1044

	let Some(sub_obj) = (unsafe { reference_from_jobject(sub) }) else {
		panic!("Invalid arguments to `GetSuperclass`");
	};

	let sub = sub_obj.extract_target_class();

	// If this `Class` represents either:
	// * the `Object` class
	if sub == crate::globals::classes::java_lang_Object()
        // * an interface
        || sub.is_interface()
        // * a primitive type, or void
        || sub.mirror().is_primitive()
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

#[unsafe(no_mangle)]
pub unsafe extern "system" fn IsAssignableFrom(
	env: *mut JNIEnv,
	sub: jclass,
	sup: jclass,
) -> jboolean {
	let sub_obj = unsafe { reference_from_jobject(sub) };
	let sup_obj = unsafe { reference_from_jobject(sup) };

	let (Some(sub), Some(sup)) = (sub_obj, sup_obj) else {
		panic!("Invalid arguments to `IsAssignableFrom`");
	};

	let sub = sub.extract_target_class();
	let sup = sup.extract_target_class();

	if sub.mirror().is_primitive() && sup.mirror().is_primitive() {
		return sub == sup;
	}

	return sub.is_subclass_of(sup) || sub.implements(sup);
}
