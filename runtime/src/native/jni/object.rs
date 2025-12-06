use crate::native::jni::{IntoJni, reference_from_jobject};
use crate::objects::instance::class::ClassInstance;
use crate::objects::reference::Reference;
use jni::sys::{JNIEnv, jboolean, jclass, jmethodID, jobject, jobjectRefType, jvalue, va_list};
use std::ptr;

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
	...
) -> jobject {
	unimplemented!("jni::NewObject");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn NewObjectV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jobject {
	unimplemented!("jni::NewObjectV")
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn NewObjectA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jobject {
	let class_obj = unsafe { reference_from_jobject(clazz) };
	let Some(class_obj) = class_obj else {
		return ptr::null_mut();
	};

	let class = class_obj.extract_target_class();
	let obj = Reference::class(ClassInstance::new(class));

	let mut args_with_receiver = vec![jvalue { l: obj.into_jni() }];
	for i in 0usize.. {
		if unsafe { args.add(i) }.is_null() {
			break;
		}

		args_with_receiver.push(unsafe { *args });
	}

	unsafe {
		super::method::call_with_c_array_args(env, clazz, methodID, args_with_receiver.as_ptr())
	};
	obj.into_jni()
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn GetObjectClass(env: *mut JNIEnv, obj: jobject) -> jclass {
	unimplemented!("jni::GetObjectClass");
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn IsInstanceOf(
	env: *mut JNIEnv,
	obj: jobject,
	clazz: jclass,
) -> jboolean {
	let obj = unsafe { reference_from_jobject(obj) };
	let Some(obj) = obj else {
		return false;
	};

	let class_obj = unsafe { reference_from_jobject(clazz) };
	let Some(class_obj) = class_obj else {
		return false;
	};

	obj.is_instance_of(class_obj.extract_instance_class())
}

pub unsafe extern "system" fn GetObjectRefType(env: *mut JNIEnv, obj: jobject) -> jobjectRefType {
	unimplemented!("jni::GetObjectRefType");
}
