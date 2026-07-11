use crate::objects::reference::Reference;
use crate::thread::JavaThread;
use crate::thread::exceptions::Throws;
use crate::thread::pool::ThreadPool;

/// A builder for a `JavaThread`
///
/// This is the only way to construct a `JavaThread`, and is responsible for spawning the associated
/// OS thread, if applicable.
#[derive(Default)]
pub struct JavaThreadBuilder {
	obj: Option<Reference>,
	stack_size: usize,
}

impl JavaThreadBuilder {
	/// Create a new `JavaThreadBuilder`
	///
	/// This is equivalent to [`Self::default`].
	pub fn new() -> JavaThreadBuilder {
		Self {
			obj: None,
			stack_size: 1024, // TODO: Default -Xss
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

	/// Set the stack size of the thread
	pub fn stack_size(mut self, size: usize) -> Self {
		self.stack_size = size;
		self
	}

	/// Construct the `JavaThread`
	///
	/// This will also spawn an OS thread if applicable.
	///
	/// [`entry_point`]: Self::entry_point
	pub fn finish(self, start: bool) -> Throws<&'static JavaThread> {
		let thread = JavaThread::new(self.obj, self.stack_size)?;

		let thread = ThreadPool::push(thread);
		if start {
			thread.start();
		}

		Throws::Ok(thread)
	}
}
