use super::references::JObjectExt;
use crate::objects::instance::class::ClassInstance;
use crate::objects::reference::Reference;
use crate::thread::JavaThread;

use std::ffi::VaList;
use std::ptr;

use ::jni::sys::{JNIEnv, jboolean, jclass, jmethodID, jobject, jobjectRefType, jvalue};
use instructions::Operand;

fn allocate_object(thread: &'static JavaThread, class: jclass) -> Option<jobject> {
	let class_obj = unsafe { class.to_reference() };
	let Some(class_obj) = class_obj else {
		return None;
	};

	let class = class_obj.extract_target_class();
	let obj = Reference::class(ClassInstance::new(class));

	Some(thread.jni_refs().allocate(obj))
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn AllocObject(env: *mut JNIEnv, clazz: jclass) -> jobject {
	unimplemented!("jni::AllocObject");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn IsSameObject(
	env: *mut JNIEnv,
	obj1: jobject,
	obj2: jobject,
) -> jboolean {
	unimplemented!("jni::IsSameObject");
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn NewObject(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	_: ...
) -> jobject {
	unimplemented!("jni::NewObject");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn NewObjectV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_>,
) -> jobject {
	let thread = JavaThread::current();
	assert_eq!(thread.env().raw(), env);

	let Some(obj_jni) = allocate_object(thread, clazz) else {
		return ptr::null_mut();
	};

	let receiver = unsafe { obj_jni.to_reference() }.expect("was just allocated");
	thread.stack().push_op(Operand::Reference(receiver));

	unsafe { super::method::call_with_va_list_args(thread, methodID, args) };
	obj_jni
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn NewObjectA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jobject {
	let thread = JavaThread::current();
	assert_eq!(thread.env().raw(), env);

	let Some(obj_jni) = allocate_object(thread, clazz) else {
		return ptr::null_mut();
	};

	let mut args_with_receiver = vec![jvalue { l: obj_jni }];
	for i in 0usize.. {
		if unsafe { args.add(i) }.is_null() {
			break;
		}

		args_with_receiver.push(unsafe { *args });
	}

	unsafe {
		super::method::call_with_c_array_args(thread, clazz, methodID, args_with_receiver.as_ptr())
	};
	obj_jni
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetObjectClass(env: *mut JNIEnv, obj: jobject) -> jclass {
	let thread = JavaThread::current();
	assert_eq!(thread.env().raw(), env);

	let Some(obj) = (unsafe { obj.to_reference() }) else {
		panic!("Calling GetObjectClass with a null object");
	};

	thread.jni_refs().allocate(obj.extract_instance_class())
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn IsInstanceOf(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
) -> jboolean {
	let obj = unsafe { clazz.to_reference() };
	let Some(obj) = obj else {
		return false;
	};

	let class_obj = unsafe { clazz.to_reference() };
	let Some(class_obj) = class_obj else {
		return false;
	};

	obj.is_instance_of(class_obj.extract_instance_class())
}

pub unsafe extern "system" fn GetObjectRefType(env: *mut JNIEnv, obj: jobject) -> jobjectRefType {
	unimplemented!("jni::GetObjectRefType");
}
