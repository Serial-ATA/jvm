use crate::thread::JavaThread;

use std::cell::UnsafeCell;
use std::fmt::Debug;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Condvar, Mutex, MutexGuard};

/// An object instance monitor, used for synchronization
///
/// The rules for this are simple:
///
/// * If the monitor has an owner (thread), it is locked.
///   * Otherwise, it is unclaimed
///
/// When a thread tries to take ownership:
///
/// * Does this thread already own this monitor?
///   * Yes - Increment the count
///   * No - Is there another owner?:
///     * Yes - This thread blocks, waiting for the count to be zero
///     * No - This thread becomes the owner, and the count is 1
pub struct Monitor {
	owner: Mutex<Option<&'static JavaThread>>,
	count: AtomicUsize,
	_lock: Mutex<()>,
	// Guard for `_lock`, need to make sure this only gets dropped when we want.
	_guard: UnsafeCell<MaybeUninit<MutexGuard<'static, ()>>>,
	_cond: Condvar,
}

// SAFETY: The `Monitor` handles locking internally
unsafe impl Send for Monitor {}
unsafe impl Sync for Monitor {}

impl Debug for Monitor {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.debug_struct("Monitor")
			.field("count", &self.count)
			.finish()
	}
}

impl Monitor {
	/// Create a new `Monitor`
	pub fn new() -> Monitor {
		Self {
			owner: Mutex::new(None),
			count: AtomicUsize::new(0),
			_lock: Mutex::new(()),
			_guard: UnsafeCell::new(MaybeUninit::uninit()),
			_cond: Condvar::new(),
		}
	}

	/// Enter a monitor
	///
	/// This will block if another thread owns this monitor.
	pub fn enter(&self, thread: &'static JavaThread) {
		{
			let mut owner = self.owner.lock().unwrap();
			if owner.is_none() {
				let _guard = self._lock.lock().unwrap();

				*owner = Some(thread);
				self.count.store(1, Ordering::SeqCst);
				unsafe { self.set_guard(_guard) };
				return;
			}

			if *owner == Some(thread) {
				self.count.fetch_add(1, Ordering::Acquire);
				return;
			}
		}

		let _guard = self._lock.lock().unwrap();

		let mut owner = self.owner.lock().unwrap();
		*owner = Some(thread);
		self.count.store(1, Ordering::SeqCst);
		unsafe { self.set_guard(_guard) };
	}

	/// Exit a monitor
	///
	/// NOTE: This will do nothing if `thread` is not the owner.
	///
	/// This will decrement the count for the owner thread, dropping the lock if it is the last entry.
	pub fn exit(&self, thread: &JavaThread) {
		let mut owner = self.owner.lock().unwrap();
		if *owner != Some(thread) {
			return;
		}

		let count = self.count.fetch_sub(1, Ordering::Acquire);
		if count == 1 {
			// We are the last holder
			*owner = None;
			unsafe { self.drop_guard() }
		}
	}

	/// Notify all blocked threads that the `Monitor` is available
	pub fn notify_all(&self) {
		self._cond.notify_all();
	}

	unsafe fn set_guard(&self, guard: MutexGuard<'_, ()>) {
		// SAFETY: The lifetime extension is fine, since the guard is never
		//         used outside of the `Monitor` itself
		unsafe {
			*self._guard.get() = MaybeUninit::new(core::mem::transmute(guard));
		}
	}

	/// Drop the current thread's mutex guard
	///
	/// # Safety
	///
	/// The caller **must** ensure that there is:
	///
	/// 1. A guard present
	/// 2. The thread has given the guard up
	unsafe fn drop_guard(&self) {
		let current_guard =
			core::mem::replace(unsafe { &mut *self._guard.get() }, MaybeUninit::uninit());
		drop(unsafe { current_guard.assume_init() });
	}
}
