use crate::objects::field::Field;
use crate::objects::monitor::Monitor;
use crate::objects::reference::Reference;
use crate::thread::JavaThread;

use std::cell::Cell;
use std::ptr::NonNull;
use std::sync::Arc;

use instructions::Operand;
use jni::sys::jint;

pub trait Instance {
	fn header(&self) -> &Header;

	#[doc(hidden)] // Shouldn't ever need to be accessed directly
	fn monitor(&self) -> Arc<Monitor>;

	fn monitor_enter(&self, thread: &'static JavaThread) {
		self.monitor().enter(thread);
	}

	fn monitor_exit(&self, thread: &'static JavaThread) {
		self.monitor().exit(thread);
	}

	fn notify_all(&self) {
		self.monitor().notify_all();
	}

	fn get_field_value(&self, field: &Field) -> Operand<Reference>;
	fn get_field_value0(&self, field_idx: usize) -> Operand<Reference>;
	fn put_field_value(&mut self, field: &Field, value: Operand<Reference>);
	fn put_field_value0(&mut self, field_idx: usize, value: Operand<Reference>);
	unsafe fn get_field_value_raw(&self, field_idx: usize) -> NonNull<Operand<Reference>>;
}

pub trait CloneableInstance {
	type ReferenceTy;

	/// Clone the object that this reference points to
	///
	/// # Safety
	///
	/// The caller **must** verify that the object is cloneable. This should rarely, if ever, be
	/// used directly.
	unsafe fn clone(&self) -> Self::ReferenceTy;
}

#[derive(Copy, Clone, Default, Debug, PartialEq)]
struct HeaderFlags {
	locked: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Header {
	flags: Cell<HeaderFlags>,
	hash: Cell<Option<jint>>,
}

// SAFETY: Synchronization is handled manually
unsafe impl Sync for Header {}

impl Header {
	pub fn new() -> Self {
		Header {
			flags: Cell::new(HeaderFlags::default()),
			hash: Cell::new(None),
		}
	}

	pub fn hash(&self) -> Option<jint> {
		self.hash.get()
	}

	// TODO: This only supports the HotSpot default hash code generation, there are still 4 other possible algorithms.
	// https://github.com/openjdk/jdk/blob/807f6f7fb868240cba5ba117c7059216f69a53f9/src/hotspot/share/runtime/synchronizer.cpp#L935
	pub fn generate_hash(&self, thread: &'static JavaThread) -> jint {
		if let Some(hash) = self.hash() {
			return hash;
		}

		let hash = thread.marsaglia_xor_shift_hash() as jint;
		self.hash.set(Some(hash));

		hash
	}
}
