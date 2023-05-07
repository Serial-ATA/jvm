use crate::macros::conditional;

conditional! {
	#[cfg(any(
		target_arch = "x86",
		target_arch = "x86_64"
	))]

	mod x86;
	pub use x86::*;
}

conditional! {
	#[cfg(not(any(
		target_arch = "x86",
		target_arch = "x86_64"
	)))]

	mod other;
	pub use other::*;
}
