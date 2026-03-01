#![native_macros::jni_fn_module]

use crate::classes;
use crate::classes::java::lang::Thread::ThreadStatus;
use crate::native::jni::{IntoJni, reference_from_jobject};
use crate::thread::exceptions::throw;
use crate::thread::pool::ThreadPool;
use crate::thread::{JavaThread, JavaThreadBuilder};

use std::cmp;
use std::sync::atomic::AtomicUsize;

use ::jni::env::JniEnv;
use ::jni::objects::{JClass, JObject, JObjectArray, JString};
use ::jni::sys::{jboolean, jint, jlong};
use native_macros::jni_call;

#[jni_call]
pub extern "C" fn JVM_StartThread(_env: JniEnv, this: JObject) {
	let Some(this) = (unsafe { reference_from_jobject(this.raw()) }) else {
		return; // TODO: Exception?
	};

	{
		if let Some(existing_thread) = ThreadPool::find_from_obj(this) {
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

#[jni_call]
pub extern "C" fn JVM_SetThreadPriority(_env: JniEnv, this: JObject, priority: jint) {
	let Some(this) = (unsafe { reference_from_jobject(this.raw()) }) else {
		return; // TODO: Exception?
	};

	let holder = classes::java::lang::Thread::holder(this.extract_class());
	classes::java::lang::Thread::holder::set_priority(holder.extract_class(), priority);

	let java_thread = ThreadPool::find_from_obj(this);
	let Some(_thread) = java_thread else {
		return;
	};

	// Thread is alive...
	todo!("Set priority on JavaThread?")
}

#[jni_call]
pub extern "C" fn JVM_Yield(_env: JniEnv, _class: JClass) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_SleepNanos(_env: JniEnv, _class: JClass, _nanos: jlong) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_CurrentCarrierThread(_env: JniEnv, _class: JClass) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_CurrentThread(env: JniEnv, _class: JClass) -> JObject {
	let thread = unsafe { &*JavaThread::for_env(env.raw().cast_const()) };
	thread
		.obj()
		.expect("current thread should exist")
		.into_jni_safe()
}

#[jni_call]
pub extern "C" fn JVM_SetCurrentThread(_env: JniEnv, _this: JObject, _thread: JObject) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetNextThreadIdOffset(_env: JniEnv, _class: JClass) -> jlong {
	// https://github.com/openjdk/jdk/blob/a3b58ee5cd1ec0ea78649d4128d272458b05eb13/src/java.base/share/classes/java/lang/Thread.java#L624-L627
	const INITIAL_THREAD_ID: usize = 3;
	static NEXT_THREAD_ID: AtomicUsize = AtomicUsize::new(INITIAL_THREAD_ID);

	NEXT_THREAD_ID.as_ptr() as jlong
}

#[jni_call]
pub extern "C" fn JVM_Interrupt(_env: JniEnv, _thread: JObject) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_HoldsLock(_env: JniEnv, _class: JClass, _thread: JObject) -> jboolean {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetStackTrace(_env: JniEnv, _thread: JObject) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_CreateThreadSnapshot(_env: JniEnv, _thread: JObject) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_SetNativeThreadName(_env: JniEnv, _thread: JObject, _name: JString) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_ScopedValueCache(_env: JniEnv, _class: JClass) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_SetScopedValueCache(_env: JniEnv, _class: JClass, _cache: JObject) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_GetAllThreads(_env: JniEnv, _dummy: JClass) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_DumpThreads(
	_env: JniEnv,
	_thread_class: JClass,
	_threads: JObjectArray,
) -> JObjectArray {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_VirtualThreadEndFirstTransition(_env: JniEnv, _vthread: JObject) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_VirtualThreadStartFinalTransition(_env: JniEnv, _vthread: JObject) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_VirtualThreadStartTransition(
	_env: JniEnv,
	_vthread: JObject,
	_is_mount: jboolean,
) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_VirtualThreadEndTransition(
	_env: JniEnv,
	_vthread: JObject,
	_is_mount: jboolean,
) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_VirtualThreadDisableSuspend(_env: JniEnv, _class: JClass, _enter: jboolean) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_VirtualThreadPinnedEvent(_env: JniEnv, _class: JClass, _op: JString) {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_TakeVirtualThreadListToUnblock(_env: JniEnv, _class: JClass) -> JObject {
	todo!()
}

#[jni_call]
pub extern "C" fn JVM_EnsureMaterializedForStackWalk_func(
	_env: JniEnv,
	_vthread: JObject,
	_value: JObject,
) {
	// Nothing to do
}
