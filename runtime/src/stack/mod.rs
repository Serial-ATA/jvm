use std::sync::OnceLock;

use common::size::Unit;

pub mod local_stack;
pub mod operand_stack;

/// The Java thread stack size
///
/// Controlled with `-Xss`
pub(crate) static STACK_SIZE: OnceLock<usize> = OnceLock::new();

/// Ensure that the stack constants are initialized
pub(crate) fn ensure_initialized() {
	// TODO: Should actually estimate based on the system config
	let _ = STACK_SIZE.set(1 * Unit::Megabytes.scale());
}
