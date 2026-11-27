use std::sync::OnceLock;

use common::size::Unit;

/// The initial Java heap size
///
/// Controlled with `-Xms`
pub(crate) static INITIAL_HEAP_SIZE: OnceLock<usize> = OnceLock::new();
/// The maximum Java heap size
///
/// Controlled with `-Xmx`
pub(crate) static MAX_HEAP_SIZE: OnceLock<usize> = OnceLock::new();

/// Ensure that the heap constants are initialized
pub(crate) fn ensure_initialized() {
	// TODO: Should actually estimate based on the system resources
	let _ = INITIAL_HEAP_SIZE.set(8 * Unit::Megabytes.scale());
	let _ = MAX_HEAP_SIZE.set(16 * Unit::Gigabytes.scale());
}
