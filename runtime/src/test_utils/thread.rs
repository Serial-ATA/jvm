use crate::thread::JavaThread;

use std::cell::SyncUnsafeCell;
use std::sync::atomic::Ordering;

static CURRENT_JAVA_THREAD: SyncUnsafeCell<Option<&'static JavaThread>> = SyncUnsafeCell::new(None);

impl JavaThread {
	pub fn seal(&self) {
		self.sealed.store(true, Ordering::SeqCst);
	}

	pub fn is_sealed(&self) -> bool {
		self.sealed.load(Ordering::SeqCst)
	}

	pub fn assert_not_sealed(&self) {
		if self.is_sealed() {
			panic!("Attempting to run Java code on a sealed thread!")
		}
	}

	pub unsafe fn set_shared_thread(thread: &'static Self) {
		let current = CURRENT_JAVA_THREAD.get();

		// SAFETY: The thread is an `Option`, so it's always initialized with *something*
		let opt = unsafe { &*current };
		assert!(
			opt.is_none(),
			"attempting to overwrite an existing JavaThread"
		);

		unsafe {
			*current = Some(thread);
		}
	}

	pub fn shared() -> &'static Self {
		let current = CURRENT_JAVA_THREAD.get();
		let opt = unsafe { &*current };
		opt.expect("Shared JavaThread should be available")
	}

	pub fn current() -> &'static JavaThread {
		Self::current_opt().unwrap_or_else(|| unsafe {
			let shared_thread = Self::shared();
			JavaThread::set_current_thread(shared_thread);
			shared_thread
		})
	}
}
