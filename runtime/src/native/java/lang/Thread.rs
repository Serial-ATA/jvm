use crate::reference::Reference;
use crate::JavaThread;

use std::ptr::NonNull;

use ::jni::env::JniEnv;
use ::jni::sys::{jboolean, jint, jlong};

include_generated!("native/java/lang/def/Thread.registerNatives.rs");
include_generated!("native/java/lang/def/Thread.definitions.rs");

pub fn findScopedValueBindings(_env: NonNull<JniEnv>) -> Reference /* java.lang.Object */ {
	unimplemented!("java.lang.Thread#findScopedValueBindings");
}

pub fn currentCarrierThread(_env: NonNull<JniEnv>) -> Reference /* java.lang.Thread */ {
	unimplemented!("java.lang.Thread#currentCarrierThread");
}

pub fn currentThread(env: NonNull<JniEnv>) -> Reference /* java.lang.Thread */ {
	unsafe {
		let thread = JavaThread::for_env(env.as_ptr() as _);
		(*thread).obj().expect("current thread should exist")
	}
}

pub fn setCurrentThread(
	_env: NonNull<JniEnv>,
	_this: Reference,   // java.lang.Thread
	_thread: Reference, // java.lang.Thread
) {
	unimplemented!("java.lang.Thread#setCurrentThread");
}

pub fn scopedValueCache(_env: NonNull<JniEnv>) -> Reference /* []java.lang.Object */ {
	unimplemented!("java.lang.Thread#scopedValueCache");
}

pub fn setScopedValueCache(_env: NonNull<JniEnv>, _cache: Reference /* []java.lang.Object */) {
	unimplemented!("java.lang.Thread#setScopedValueCache");
}

pub fn ensureMaterializedForStackWalk(
	_env: NonNull<JniEnv>,
	_o: Reference, // java.lang.Object
) {
	unimplemented!("java.lang.Thread#ensureMaterializedForStackWalk");
}

pub fn yield0(_env: NonNull<JniEnv>) {
	std::thread::yield_now();
}

// throws InterruptedException
pub fn sleepNanos0(_env: NonNull<JniEnv>, _nanos: jlong) {
	unimplemented!("java.lang.Thread#sleepNanos0");
}

pub fn start0(_env: NonNull<JniEnv>, _this: Reference /* java.lang.Thread */) {
	unimplemented!("java.lang.Thread#start0");
}

pub fn holdsLock(_env: NonNull<JniEnv>, _obj: Reference /* java.lang.Object */) -> jboolean {
	unimplemented!("java.lang.Thread#HoldsLock");
}

pub fn getStackTrace0(
	_env: NonNull<JniEnv>,
	_this: Reference, // java.lang.Thread
) -> Reference /* java.lang.Object */
{
	unimplemented!("java.lang.Thread#getStackTrace0");
}

pub fn dumpThreads(
	_env: NonNull<JniEnv>,
	_threads: Reference, // []java.lang.Thread
) -> Reference /* [][]java.lang.StackTraceElement */
{
	unimplemented!("java.lang.Thread#dumpThreads");
}

pub fn getThreads(_env: NonNull<JniEnv>) -> Reference /* []java.lang.Thread */ {
	unimplemented!("java.lang.Thread#getThreads");
}

pub fn setPriority0(
	_env: NonNull<JniEnv>,
	_this: Reference, // java.lang.Thread
	_new_priority: jint,
) {
	unimplemented!("java.lang.Thread#setPriority0");
}

pub fn interrupt0(_env: NonNull<JniEnv>, _this: Reference /* java.lang.Thread */) {
	unimplemented!("java.lang.Thread#interrupt");
}

pub fn clearInterruptEvent(_env: NonNull<JniEnv>) {
	unimplemented!("java.lang.Thread#clearInterruptEvent");
}

pub fn setNativeName(
	_env: NonNull<JniEnv>,
	_this: Reference, // java.lang.Thread
	_name: Reference, // java.lang.String
) {
	unimplemented!("java.lang.Thread#setNativeName");
}

pub fn getNextThreadIdOffset(_env: NonNull<JniEnv>) -> jlong {
	unimplemented!("java.lang.Thread#getNextThreadIdOffset");
}
