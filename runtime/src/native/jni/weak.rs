//! # Weak Global References
//!
//! Weak global references are a special kind of global reference. Unlike normal global references,
//! a weak global reference allows the underlying Java object to be garbage collected.
//!
//! Weak global references may be used in any situation where global or local references are used.
//!
//! Weak global references are related to Java phantom references (`java.lang.ref.PhantomReference`).
//! A weak global reference to a specific object is treated as a phantom reference referring to that object when determining whether the object is phantom reachable (see `java.lang.ref`).
//! Such a weak global reference will become functionally equivalent to `NULL` at the same time as a `PhantomReference` referring to that same object would be cleared by the garbage collector.
//!
//! Since garbage collection may occur while native methods are running, objects referred to by weak global references can be freed at any time.
//! While weak global references can be used where global references are used, it is generally inappropriate to do so, as they may become functionally equivalent to `NULL` without notice.
//!
//! `IsSameObject` can be used to compare a weak global reference to a non-`NULL` local or global reference.
//! If the objects are the same, the weak global reference won't become functionally equivalent to `NULL` so long as the other reference has not been deleted.
//!
//! `IsSameObject` can also be used to compare a weak global reference to `NULL` to determine whether the underlying object has been freed.
//! However, programmers should not rely on this check to determine whether a weak global reference may be used (as a non-`NULL` reference) in any future JNI function call,
//! since an intervening garbage collection could change the weak global reference.
//!
//! Instead, it is recommended that a (strong) local or global reference to the underlying object be acquired using one of the JNI functions NewLocalRef or NewGlobalRef.
//! These functions will return `NULL` if the object has been freed. Otherwise, the new reference will prevent the underlying object from being freed.
//! The new reference, if non-`NULL`, can then be used to access the underlying object, and deleted when such access is no longer needed.

use jni::sys::{JNIEnv, jobject, jweak};

#[unsafe(no_mangle)]
pub extern "system" fn NewWeakGlobalRef(env: *mut JNIEnv, obj: jobject) -> jweak {
	unimplemented!("jni::NewWeakGlobalRef");
}

#[unsafe(no_mangle)]
pub extern "system" fn DeleteWeakGlobalRef(env: *mut JNIEnv, ref_: jweak) {
	unimplemented!("jni::DeleteWeakGlobalRef");
}
