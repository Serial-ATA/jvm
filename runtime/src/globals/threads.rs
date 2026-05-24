use crate::objects::reference::Reference;

use std::sync::OnceLock;

static SYSTEM_THREAD_GROUP: OnceLock<Reference> = OnceLock::new();
static MAIN_THREAD_GROUP: OnceLock<Reference> = OnceLock::new();

/// Set the system thread group
///
/// # Panics
///
/// Panics if called more than once
pub fn set_system_thread_group(reference: Reference) {
	SYSTEM_THREAD_GROUP
		.set(reference)
		.expect("attempted to set system thread group more than once")
}

/// Get the main thread group
///
/// # Panics
///
/// Panics if [`set_main_thread_group()`] hasn't been called prior.
pub fn main_thread_group() -> Reference {
	*MAIN_THREAD_GROUP
		.get()
		.expect("main thread group not initialized")
}

/// Set the main thread group
///
/// # Panics
///
/// Panics if called more than once
pub fn set_main_thread_group(reference: Reference) {
	MAIN_THREAD_GROUP
		.set(reference)
		.expect("attempted to set main thread group more than once")
}
