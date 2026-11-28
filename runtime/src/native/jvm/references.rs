use crate::classes;
use crate::native::jni::references::JObjectExt;
use crate::objects::reference::Reference;
use crate::thread::JavaThread;

use ::jni::env::JniEnv;
use ::jni::objects::JObject;
use ::jni::sys::jboolean;
use native_macros::jni_call;

#[jni_call]
pub extern "C" fn JVM_GetAndClearReferencePendingList(_env: JniEnv) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_HasReferencePendingList(_env: JniEnv) -> jboolean {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_WaitForReferencePendingList(_env: JniEnv) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_ReferenceGet(env: JniEnv, reference: JObject) -> JObject {
	let thread = JavaThread::current();
	assert_eq!(thread.env(), env);

	let Some(reference) = (unsafe { reference.to_reference() }) else {
		panic!("null object passed to `JVM_ReferenceGet`");
	};

	let ret = classes::java::lang::r#ref::Reference::referent(reference);
	thread.jni_refs().allocate_wrapped(ret)
}

#[jni_call]
pub extern "C" fn JVM_ReferenceRefersTo(env: JniEnv, reference: JObject, obj: JObject) -> jboolean {
	let thread = JavaThread::current();
	assert_eq!(thread.env(), env);

	let Some(reference) = (unsafe { reference.to_reference() }) else {
		panic!("null object passed to `JVM_ReferenceRefersTo`");
	};

	let obj = (unsafe { obj.to_reference_maybe_null() });

	classes::java::lang::r#ref::Reference::referent(reference) == obj
}

#[jni_call]
pub extern "C" fn JVM_ReferenceClear(_env: JniEnv, reference: JObject) {
	let Some(this) = (unsafe { reference.to_reference() }) else {
		panic!("null object passed to `JVM_ReferenceRefersTo`");
	};

	classes::java::lang::r#ref::Reference::set_referent(this, Reference::null());
}

#[jni_call]
pub extern "C" fn JVM_PhantomReferenceRefersTo(
	_env: JniEnv,
	_reference: JObject,
	_obj: JObject,
) -> jboolean {
	todo!()
}
