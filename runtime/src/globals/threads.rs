use crate::objects::reference::Reference;

use std::cell::SyncUnsafeCell;
use std::mem::MaybeUninit;

static THREAD_GROUP: SyncUnsafeCell<MaybeUninit<Reference>> =
	SyncUnsafeCell::new(MaybeUninit::uninit());

/// Get the main thread group
///
/// # Safety
///
/// This assumes that `set_thread_group` has been called prior. Otherwise, the reference will
/// point to uninitialized memory.
pub unsafe fn main_thread_group() -> Reference {
	unsafe {
		let r = (*THREAD_GROUP.get()).assume_init_ref();
		Reference::clone(r)
	}
}

/// Set the main thread group
///
/// **THIS MUST ONLY BE CALLED ONCE**
///
/// # Safety
///
/// All responsibility is placed on the caller to ensure this is called once.
pub unsafe fn set_main_thread_group(reference: Reference) {
	unsafe {
		THREAD_GROUP.get().write(MaybeUninit::new(reference));
	}
}
