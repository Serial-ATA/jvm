use crate::objects::reference::Reference;
use crate::thread::pool::ThreadPool;
use crate::thread::JavaThread;

use std::thread;

/// A builder for a `JavaThread`
///
/// This is the only way to construct a `JavaThread`, and is responsible for spawning the associated
/// OS thread, if applicable.
#[derive(Default)]
pub struct JavaThreadBuilder {
	obj: Option<Reference>,
	entry_point: Option<Box<dyn Fn(&JavaThread) + Send + Sync + 'static>>,
	stack_size: usize,
}

impl JavaThreadBuilder {
	/// Create a new `JavaThreadBuilder`
	///
	/// This is equivalent to [`Self::default`].
	pub fn new() -> JavaThreadBuilder {
		Self {
			obj: None,
			entry_point: None,
			stack_size: 0, // TODO: Default -Xss
		}
	}

	// TODO: Unsafe? The object is not verified to be of the correct class.
	/// Set the `java.lang.Thread` associated with this `JavaThread`
	///
	/// It is up to the caller to verify that `obj` is *actually* of the correct type.
	pub fn obj(mut self, obj: Reference) -> Self {
		self.obj = Some(obj);
		self
	}

	/// Set the entrypoint of this `JavaThread`
	///
	/// Setting this will spawn an OS thread to run `entry`. This is really only used with [`JavaThread::default_entry_point`],
	/// which calls `java.lang.Thread#run` on the associated [`obj`].
	///
	/// [`obj`]: Self::obj
	pub fn entry_point(mut self, entry: impl Fn(&JavaThread) + Send + Sync + 'static) -> Self {
		self.entry_point = Some(Box::new(entry));
		self
	}

	/// Set the stack size of the associated OS thread
	///
	/// This will have no effect if there is no [`entry_point`] set.
	///
	/// [`entry_point`]: Self::entry_point
	pub fn stack_size(mut self, size: usize) -> Self {
		self.stack_size = size;
		self
	}

	/// Construct the `JavaThread`
	///
	/// This will also spawn an OS thread if applicable.
	///
	/// The return type of this depends on whether an OS thread has been spawned. If no [`entry_point`]
	/// was set, it is safe to unwrap it as [`MaybeArc::Not`].
	///
	/// [`entry_point`]: Self::entry_point
	pub fn finish(self) -> &'static JavaThread {
		let thread = JavaThread::new(self.obj);

		let thread = ThreadPool::push(thread);
		if let Some(entry_point) = self.entry_point {
			let mut os_thread = thread::Builder::new();
			if self.stack_size > 0 {
				os_thread = os_thread.stack_size(self.stack_size);
			}

			let os_thread_ptr = thread.os_thread.get();

			// TODO: Error handling
			let handle = os_thread.spawn(move || entry_point(thread)).unwrap();
			unsafe {
				*os_thread_ptr = Some(handle);
			}
		}

		thread
	}
}
