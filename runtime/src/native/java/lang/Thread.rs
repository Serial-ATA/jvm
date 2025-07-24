use crate::classes;
use crate::classes::java::lang::Thread::ThreadStatus;
use crate::objects::class::ClassPtr;
use crate::objects::reference::Reference;
use crate::thread::exceptions::throw;
use crate::thread::pool::ThreadPool;
use crate::thread::{JavaThread, JavaThreadBuilder};

use std::cmp;
use std::sync::atomic::AtomicUsize;

use ::jni::env::JniEnv;
use ::jni::sys::{jboolean, jint, jlong};

include_generated!("native/java/lang/def/Thread.registerNatives.rs");
include_generated!("native/java/lang/def/Thread.definitions.rs");

pub fn findScopedValueBindings(_env: JniEnv, _class: ClassPtr) -> Reference /* java.lang.Object */
{
	unimplemented!("java.lang.Thread#findScopedValueBindings");
}

pub fn currentCarrierThread(_env: JniEnv, _class: ClassPtr) -> Reference /* java.lang.Thread */
{
	unimplemented!("java.lang.Thread#currentCarrierThread");
}

pub fn currentThread(env: JniEnv, _class: ClassPtr) -> Reference /* java.lang.Thread */
{
	unsafe {
		let thread = JavaThread::for_env(env.raw() as _);
		(*thread).obj().expect("current thread should exist")
	}
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

pub fn ensureMaterializedForStackWalk(
	_env: JniEnv,
	_class: ClassPtr,
	_o: Reference, // java.lang.Object
) {
	// Nothing to do
}

pub fn yield0(_env: JniEnv, _class: ClassPtr) {
	std::thread::yield_now();
}

// throws InterruptedException
pub fn sleepNanos0(_env: JniEnv, _class: ClassPtr, _nanos: jlong) {
	unimplemented!("java.lang.Thread#sleepNanos0");
}

pub fn start0(_env: JniEnv, this: Reference /* java.lang.Thread */) {
	{
		if let Some(existing_thread) = ThreadPool::find_from_obj(this.clone()) {
			throw!(existing_thread, IllegalThreadStateException);
		}
	}

	let holder = classes::java::lang::Thread::holder(this.extract_class());
	let stack_size_raw = classes::java::lang::Thread::holder::stackSize(holder.extract_class());

	let mut thread_builder = JavaThreadBuilder::new()
		.obj(this)
		.entry_point(JavaThread::default_entry_point);

	if stack_size_raw > 0 {
		let stack_size = cmp::min(stack_size_raw as usize, u32::MAX as usize);
		thread_builder = thread_builder.stack_size(stack_size);
	}

	let thread = thread_builder.finish();

	let obj = thread.obj().expect("current thread object should exist");
	let holder = classes::java::lang::Thread::holder(obj.extract_class());
	classes::java::lang::Thread::holder::set_threadStatus(
		holder.extract_class(),
		ThreadStatus::Runnable,
	);
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

pub fn setPriority0(
	_env: JniEnv,
	this: Reference, // java.lang.Thread
	new_priority: jint,
) {
	let holder = classes::java::lang::Thread::holder(this.extract_class());
	classes::java::lang::Thread::holder::set_priority(holder.extract_class(), new_priority);

	let java_thread = ThreadPool::find_from_obj(this);
	let Some(_thread) = java_thread else {
		return;
	};

	// Thread is alive...
	todo!("Set priority on JavaThread?")
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

pub fn getNextThreadIdOffset(_env: JniEnv, _class: ClassPtr) -> jlong {
	// https://github.com/openjdk/jdk/blob/a3b58ee5cd1ec0ea78649d4128d272458b05eb13/src/java.base/share/classes/java/lang/Thread.java#L624-L627
	const INITIAL_THREAD_ID: usize = 3;
	static NEXT_THREAD_ID: AtomicUsize = AtomicUsize::new(INITIAL_THREAD_ID);

	NEXT_THREAD_ID.as_ptr() as jlong
}
