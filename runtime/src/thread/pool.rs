use super::JavaThread;
use crate::classes;
use crate::objects::reference::Reference;

use std::cell::SyncUnsafeCell;
use std::collections::LinkedList;
use std::sync::{LazyLock, Mutex};

static VM_THREAD_POOL: LazyLock<ThreadPool> = LazyLock::new(|| ThreadPool {
	list: SyncUnsafeCell::new(LinkedList::new()),
	write_mutex: Mutex::new(()),
});

pub struct ThreadPool {
	// A list of all currently alive threads.
	//
	// This is a `LinkedList`, as reads will do a lifetime-extension, and are not guarded. We cannot
	// risk a realloc invalidating a reference.
	list: SyncUnsafeCell<LinkedList<JavaThread>>,
	write_mutex: Mutex<()>,
}

impl ThreadPool {
	/// Add a thread to the pool
	pub fn push(thread: JavaThread) -> &'static JavaThread {
		let _guard = VM_THREAD_POOL.write_mutex.lock().unwrap();

		let list = unsafe { &mut *VM_THREAD_POOL.list.get() };
		list.push_back(thread);

		list.back().unwrap()
	}

	/// Whether `thread` is in this pool
	pub fn contains(thread: &'static JavaThread) -> bool {
		let list = unsafe { &mut *VM_THREAD_POOL.list.get() };
		list.iter().any(|t| t.env == thread.env)
	}

	/// Find the [`JavaThread`] associated with `obj`
	///
	/// This is the only safe way to relate `java.lang.Thread` objects to their internal [`JavaThread`]
	/// counterparts. However, if the associated [`JavaThread`] is *not* in this pool, this method will
	/// not be able to find it.
	pub fn find_from_obj(obj: Reference) -> Option<&'static JavaThread> {
		let eetop = classes::java::lang::Thread::eetop(obj.extract_class());
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

		if ThreadPool::contains(java_thread) {
			return Some(java_thread);
		}

		None
	}
}
