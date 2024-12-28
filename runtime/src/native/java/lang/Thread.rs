use crate::objects::reference::Reference;
use crate::thread::pool::ThreadPool;
use crate::thread::{java_lang_Thread, JavaThread, JavaThreadBuilder};

use std::cmp;
use std::ptr::NonNull;
use std::sync::atomic::AtomicUsize;

use crate::thread::java_lang_Thread::ThreadStatus;
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

pub fn start0(_env: NonNull<JniEnv>, this: Reference /* java.lang.Thread */) {
	{
		let existing_thread = unsafe { ThreadPool::find_from_obj(this.clone()) };
		if existing_thread.is_some() {
			JavaThread::current()
				.throw_exception(todo!("Throw java.lang.IllegalThreadStateException"))
		}
	}

	let stack_size_raw = java_lang_Thread::holder::stack_size(&this);

	let mut thread_builder = JavaThreadBuilder::new()
		.obj(this)
		.entry_point(JavaThread::default_entry_point);

	if stack_size_raw > 0 {
		let stack_size = cmp::min(stack_size_raw as usize, u32::MAX as usize);
		thread_builder = thread_builder.stack_size(stack_size);
	}

	let thread = thread_builder.finish();

	let obj = thread.obj().expect("current thread object should exist");
	java_lang_Thread::holder::set_thread_status(obj, ThreadStatus::Runnable);
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
	this: Reference, // java.lang.Thread
	new_priority: jint,
) {
	java_lang_Thread::holder::set_priority(this.clone(), new_priority);

	let java_thread = unsafe { ThreadPool::find_from_obj(this) };
	let Some(thread) = java_thread else {
		return;
	};

	// Thread is alive...
	let _thread_ref = unsafe { &*thread };
	todo!("Set priority on JavaThread?")
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
	// https://github.com/openjdk/jdk/blob/a3b58ee5cd1ec0ea78649d4128d272458b05eb13/src/java.base/share/classes/java/lang/Thread.java#L624-L627
	const INITIAL_THREAD_ID: usize = 3;
	static NEXT_THREAD_ID: AtomicUsize = AtomicUsize::new(INITIAL_THREAD_ID);

	NEXT_THREAD_ID.as_ptr() as jlong
}
