use core::ffi::VaList;
use jni::sys::{jboolean, jclass, jmethodID, jobject, jobjectRefType, jvalue, JNIEnv};

pub extern "system" fn AllocObject(env: *mut JNIEnv, clazz: jclass) -> jobject {
	unimplemented!("jni::AllocObject");
}

pub extern "system" fn IsSameObject(env: *mut JNIEnv, obj1: jobject, obj2: jobject) -> jboolean {
	unimplemented!("jni::IsSameObject");
}

pub unsafe extern "C" fn NewObject(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	...
) -> jobject {
	unimplemented!("jni::NewObject");
}

pub extern "system" fn NewObjectV(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: VaList<'_, '_>,
) -> jobject {
	unimplemented!("jni::NewObjectV")
}

pub extern "system" fn NewObjectA(
	env: *mut JNIEnv,
	clazz: jclass,
	methodID: jmethodID,
	args: *const jvalue,
) -> jobject {
	unimplemented!("jni::NewObjectA")
}

pub extern "system" fn GetObjectClass(env: *mut JNIEnv, obj: jobject) -> jclass {
	unimplemented!("jni::GetObjectClass");
}

pub extern "system" fn IsInstanceOf(env: *mut JNIEnv, obj: jobject, clazz: jclass) -> jboolean {
	unimplemented!("jni::IsInstanceOf");
}

pub extern "system" fn GetObjectRefType(env: *mut JNIEnv, obj: jobject) -> jobjectRefType {
	unimplemented!("jni::GetObjectRefType");
}
