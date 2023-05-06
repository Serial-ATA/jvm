#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub mod other;

use crate::native::lib::macros::conditional;

conditional! {
	#[cfg(any(
		target_arch = "x86",
		target_arch = "x86_64"
	))]

	mod x86;
	pub use x86::*;
}
