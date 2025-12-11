use crate::objects::class::ClassPtr;
use crate::objects::reference::Reference;

use ::jni::env::JniEnv;
use ::jni::sys::{jboolean, jlong};

include_generated!("native/java/lang/def/Thread.definitions.rs");

pub fn findScopedValueBindings(_env: JniEnv, _class: ClassPtr) -> Reference /* java.lang.Object */
{
	unimplemented!("java.lang.Thread#findScopedValueBindings");
}

pub fn currentCarrierThread(_env: JniEnv, _class: ClassPtr) -> Reference /* java.lang.Thread */
{
	unimplemented!("java.lang.Thread#currentCarrierThread");
}

pub fn setCurrentThread(
	_env: JniEnv,
	_this: Reference,   // java.lang.Thread
	_thread: Reference, // java.lang.Thread
) {
	unimplemented!("java.lang.Thread#setCurrentThread");
}

pub fn scopedValueCache(_env: JniEnv, _class: ClassPtr) -> Reference /* []java.lang.Object */
{
	unimplemented!("java.lang.Thread#scopedValueCache");
}

pub fn setScopedValueCache(
	_env: JniEnv,
	_class: ClassPtr,
	_cache: Reference, // []java.lang.Object
) {
	unimplemented!("java.lang.Thread#setScopedValueCache");
}

pub fn yield0(_env: JniEnv, _class: ClassPtr) {
	std::thread::yield_now();
}

// throws InterruptedException
pub fn sleepNanos0(_env: JniEnv, _class: ClassPtr, _nanos: jlong) {
	unimplemented!("java.lang.Thread#sleepNanos0");
}

pub fn holdsLock(
	_env: JniEnv,
	_class: ClassPtr,
	_obj: Reference, // java.lang.Object
) -> jboolean {
	unimplemented!("java.lang.Thread#HoldsLock");
}

pub fn getStackTrace0(
	_env: JniEnv,
	_this: Reference, // java.lang.Thread
) -> Reference /* java.lang.Object */
{
	unimplemented!("java.lang.Thread#getStackTrace0");
}

pub fn dumpThreads(
	_env: JniEnv,
	_class: ClassPtr,
	_threads: Reference, // []java.lang.Thread
) -> Reference /* [][]java.lang.StackTraceElement */
{
	unimplemented!("java.lang.Thread#dumpThreads");
}

pub fn getThreads(_env: JniEnv, _class: ClassPtr) -> Reference /* []java.lang.Thread */
{
	unimplemented!("java.lang.Thread#getThreads");
}

pub fn interrupt0(_env: JniEnv, _this: Reference /* java.lang.Thread */) {
	unimplemented!("java.lang.Thread#interrupt");
}

pub fn clearInterruptEvent(_env: JniEnv, _class: ClassPtr) {
	unimplemented!("java.lang.Thread#clearInterruptEvent");
}

pub fn setNativeName(
	_env: JniEnv,
	_this: Reference, // java.lang.Thread
	_name: Reference, // java.lang.String
) {
	unimplemented!("java.lang.Thread#setNativeName");
}
