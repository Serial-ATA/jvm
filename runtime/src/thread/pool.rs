use super::JavaThread;
use crate::objects::instance::Instance;
use crate::objects::reference::Reference;

use std::sync::RwLock;

static VM_THREAD_POOL: RwLock<ThreadPool> = RwLock::new(ThreadPool::new());

pub struct ThreadPool {
	threads: Vec<&'static JavaThread>,
}

impl ThreadPool {
	const fn new() -> ThreadPool {
		Self {
			threads: Vec::new(),
		}
	}

	/// Add a thread to the pool
	pub fn push(thread: JavaThread) -> &'static JavaThread {
		let mut guard = VM_THREAD_POOL.write().unwrap();
		let thread = Box::leak(Box::new(thread));
		guard.threads.push(thread);
		thread
	}

	/// Whether `thread` is in this pool
	pub fn contains(thread: &JavaThread) -> bool {
		VM_THREAD_POOL.read().unwrap().threads.contains(&thread)
	}

	/// Find the [`JavaThread`] associated with `obj`
	///
	/// This is the only safe way to relate `java.lang.Thread` objects to their internal [`JavaThread`]
	/// counterparts. However, if the associated [`JavaThread`] is *not* in this pool, this method will
	/// not be able to find it.
	pub fn find_from_obj(obj: Reference) -> Option<&'static JavaThread> {
		let eetop_offset = crate::globals::field_offsets::java_lang_Thread::eetop_field_offset();
		let field_value_ptr = unsafe { obj.get_field_value_raw(eetop_offset) };
		let field_value = unsafe { field_value_ptr.as_ref() };
		let eetop = field_value.expect_long();
		if eetop == 0 {
			// Thread is not alive
			return None;
		}

		// SAFETY: This isn't really safe, but we have to assume that the pointer at `eetop` is a valid
		//         one that we set.
		let java_thread = unsafe { &*(eetop as *mut JavaThread) };

		let current = JavaThread::current();
		if java_thread == current {
			return Some(current);
		}

		if ThreadPool::contains(&java_thread) {
			return Some(java_thread);
		}

		None
	}
}
