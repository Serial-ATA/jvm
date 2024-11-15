use jni::sys::{jboolean, jclass, jmethodID, jobject, jobjectRefType, jvalue, va_list, JNIEnv};

#[no_mangle]
pub extern "system" fn AllocObject(env: *mut JNIEnv, clazz: jclass) -> jobject {
	unimplemented!("jni::AllocObject");
}

#[no_mangle]
pub extern "system" fn IsSameObject(env: *mut JNIEnv, obj1: jobject, obj2: jobject) -> jboolean {
	unimplemented!("jni::IsSameObject");
}

#[no_mangle]
pub unsafe extern "C" fn NewObject(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jobject {
	unimplemented!("jni::NewObject");
}

#[no_mangle]
pub unsafe extern "system" fn NewObjectV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: va_list,
) -> jobject {
	unimplemented!("jni::NewObjectV")
}

#[no_mangle]
pub extern "system" fn NewObjectA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jobject {
	unimplemented!("jni::NewObjectA")
}

#[no_mangle]
pub extern "system" fn GetObjectClass(env: *mut JNIEnv, obj: jobject) -> jclass {
	unimplemented!("jni::GetObjectClass");
}

#[no_mangle]
pub extern "system" fn IsInstanceOf(env: *mut JNIEnv, obj: jobject, clazz: jclass) -> jboolean {
	unimplemented!("jni::IsInstanceOf");
}

pub extern "system" fn GetObjectRefType(env: *mut JNIEnv, obj: jobject) -> jobjectRefType {
	unimplemented!("jni::GetObjectRefType");
}
