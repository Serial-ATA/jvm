use crate::objects::class::ClassPtr;
use crate::objects::monitor::{Monitor, MonitorMap};
use crate::thread::JavaThread;
use crate::thread::exceptions::Throws;

use jni::sys::jint;
use std::alloc;
use std::alloc::Layout;
use std::time::Duration;

/// A heap allocated object instance
pub trait Object: Sized {
	type Descriptor: Sized;

	unsafe fn allocate(descriptor: Self::Descriptor, fields_size: usize) -> *mut Self::Descriptor {
		let instance_size = size_of::<Self::Descriptor>() + fields_size;
		let layout = Layout::array::<u8>(instance_size).expect("valid layout");

		let instance_ptr;
		unsafe {
			// SAFETY: Every operand type has a specified default value of 0
			let mem = alloc::alloc_zeroed(layout);
			instance_ptr = mem.cast::<Self::Descriptor>();
			instance_ptr.write(descriptor);
		}

		instance_ptr
	}

	/// Fetch or generate a hash for this object
	///
	/// Once a hash is generated, it will be cached in the object header for future reads.
	///
	/// In the event that another thread is already generating a hash, this thread will spin until it finishes.
	fn hash(&self, thread: &'static JavaThread) -> jint;

	#[doc(hidden)] // Shouldn't ever need to be accessed directly
	fn monitor(&self, thread: &'static JavaThread) -> &'static Monitor {
		MonitorMap::find_or_add(self, thread)
	}

	fn monitor_enter(&self, thread: &'static JavaThread) {
		self.monitor(thread).enter(thread);
	}

	fn monitor_exit(&self, thread: &'static JavaThread) {
		self.monitor(thread).exit(thread);
	}

	fn notify(&self, thread: &'static JavaThread) -> Throws<()> {
		self.monitor(thread).notify(thread)
	}

	fn notify_all(&self, thread: &'static JavaThread) -> Throws<()> {
		self.monitor(thread).notify_all(thread)
	}

	fn wait(&self, thread: &'static JavaThread, timeout: Option<Duration>) -> Throws<()> {
		self.monitor(thread).wait(thread, timeout)
	}

	/// The class backing the object
	///
	/// NOTE: For mirrors this will always be `java.lang.Class`, **NOT** the class that the mirror *targets*.
	///
	/// In the following:
	///
	/// ```java
	/// var c = String.class;
	/// ```
	///
	/// `c` is an instance of `Class<?>`, which backs the mirror.
	///
	/// To get the class that this mirror is targeting (in this case, `java.lang.String`), use [`MirrorInstance::target_class`].
	fn class(&self) -> ClassPtr;

	#[inline]
	fn is_object_array(&self) -> bool {
		false
	}

	#[inline]
	fn is_primitive_array(&self) -> bool {
		false
	}

	#[inline]
	fn is_class(&self) -> bool {
		false
	}

	#[inline]
	fn is_mirror(&self) -> bool {
		false
	}

	/// Returns a pointer to the start of the object's allocation
	unsafe fn raw(&self) -> *mut ();

	/// Returns a pointer to the start of the object's fields
	unsafe fn field_base(&self) -> *mut u8;

	/// Write `value` to `offset`
	///
	/// NOTE: `offset` is the **byte offset** from the base of the object's fields, **NOT** the start of the object.
	///
	/// # Safety
	///
	/// The caller must verify that the field at `offset` is of type `T`, at the risk of overwriting
	/// other fields or violating a field's volatility.
	unsafe fn put<T: Copy>(&self, value: T, offset: usize) {
		unsafe {
			self.get_raw::<T>(offset).write_unaligned(value);
		};
	}

	/// Read a `T` from `offset`
	///
	/// NOTE: `offset` is the **byte offset** from the base of the object's fields, **NOT** the start of the object.
	///
	/// # Safety
	///
	/// The caller must verify that the field at `offset` is of type `T`, at the risk of reading the
	/// data of other fields or violating a field's volatility.
	unsafe fn get<T: Copy>(&self, offset: usize) -> T {
		unsafe { self.get_raw::<T>(offset).read_unaligned() }
	}

	/// Get a pointer to a `T` at `offset`
	///
	/// NOTE: `offset` is the **byte offset** from the base of the object's fields, **NOT** the start of the object.
	///
	/// # Safety
	///
	/// The caller must verify that the field at `offset` is of type `T`, and that it is valid for
	/// reads/writes in the current context.
	unsafe fn get_raw<T: Copy>(&self, offset: usize) -> *mut T {
		unsafe {
			let value_base = self.field_base().add(offset);
			value_base.cast::<T>()
		}
	}
}
