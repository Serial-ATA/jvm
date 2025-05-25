use crate::thread::JavaThread;
use crate::thread::exceptions::{Throws, throw};

use std::cell::{Cell, UnsafeCell};
use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Condvar, Mutex, ReentrantLock, ReentrantLockGuard};
use std::time::Duration;

type OwnerGuard<'a> = ReentrantLockGuard<'a, Cell<Option<&'static JavaThread>>>;

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
	owner: ReentrantLock<Cell<Option<&'static JavaThread>>>,
	// Always hold 1 live reference to the owner guard
	_owner_guard: UnsafeCell<Option<OwnerGuard<'static>>>,
	count: AtomicUsize,
	_notify_lock: Mutex<()>,
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
			owner: ReentrantLock::new(Cell::new(None)),
			_owner_guard: UnsafeCell::new(None),
			count: AtomicUsize::new(0),
			_notify_lock: Mutex::new(()),
			_cond: Condvar::new(),
		}
	}

	/// Enter a monitor
	///
	/// This will block if another thread owns this monitor.
	pub fn enter(&self, thread: &'static JavaThread) {
		let owner = self.owner.lock();

		// sanity
		assert!(owner.get() == Some(thread) || owner.get().is_none());

		if owner.get().is_none() {
			owner.set(Some(thread));
			self.count.store(1, Ordering::SeqCst);
			unsafe { self.set_guard(owner) };
			return;
		}

		if owner.get() == Some(thread) {
			self.count.fetch_add(1, Ordering::Acquire);
			return;
		}

		self.count.store(1, Ordering::SeqCst);
		unsafe { self.set_guard(owner) };
	}

	/// Exit a monitor
	///
	/// NOTE: This will do nothing if `thread` is not the owner.
	///
	/// This will decrement the count for the owner thread, dropping the lock if it is the last entry.
	pub fn exit(&self, thread: &'static JavaThread) {
		let Some(owner) = self.owner.try_lock() else {
			return;
		};

		// sanity
		assert!(owner.get() == Some(thread));

		let count = self.count.fetch_sub(1, Ordering::Acquire);
		if count == 1 {
			// We are the last holder
			owner.set(None);
			unsafe { self.drop_guard() }
		}
	}

	/// Notify one of the blocked threads that the `Monitor` is available
	pub fn notify(&self, thread: &'static JavaThread) -> Throws<()> {
		self.verify_current_thread_owner(thread)?;

		self._cond.notify_one();
		Throws::Ok(())
	}

	/// Notify all blocked threads that the `Monitor` is available
	pub fn notify_all(&self, thread: &'static JavaThread) -> Throws<()> {
		self.verify_current_thread_owner(thread)?;

		self._cond.notify_all();
		Throws::Ok(())
	}

	/// Wait `timeout` millis for the `Monitor` to become available
	pub fn wait(&self, thread: &'static JavaThread, timeout: Option<Duration>) -> Throws<()> {
		self.verify_current_thread_owner(thread)?;

		let count = self.count.swap(0, Ordering::Relaxed);
		let owner = self.owner.lock().take().expect("should exist");

		let guard = self._notify_lock.lock().unwrap();
		match timeout {
			Some(timeout) => {
				drop(self._cond.wait_timeout(guard, timeout).unwrap());
			},
			None => {
				drop(self._cond.wait(guard).unwrap());
			},
		}

		self.count.store(count, Ordering::Relaxed);
		self.owner.lock().replace(Some(owner));

		Throws::Ok(())
	}

	fn verify_current_thread_owner(&self, thread: &'static JavaThread) -> Throws<()> {
		let current_thread_is_owner = self
			.owner
			.try_lock()
			.is_some_and(|t| t.get() == Some(thread));

		if !current_thread_is_owner {
			throw!(@DEFER IllegalMonitorStateException, "current thread is not owner");
		}

		Throws::Ok(())
	}

	unsafe fn set_guard(&self, guard: OwnerGuard<'_>) {
		// SAFETY: The lifetime extension is fine, since the guard is never
		//         used outside of the `Monitor` itself
		unsafe {
			*self._owner_guard.get() = Some(core::mem::transmute(guard));
		}
	}

	/// Drop the current thread's mutex guard
	///
	/// # Safety
	///
	/// The caller **must** ensure that the thread has given the guard up
	unsafe fn drop_guard(&self) {
		let _ = core::mem::replace(unsafe { &mut *self._owner_guard.get() }, None);
	}
}
