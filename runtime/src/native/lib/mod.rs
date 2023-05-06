use crate::native::lib::macros::conditional;

pub mod arch;
mod macros;
pub mod properties;

// `target_family` specific exports

conditional! {
	#[cfg(target_family = "unix")]

	mod unix;
	pub use unix::os_arch;
}

conditional! {
	#[cfg(target_family = "windows")]

	mod windows;
	pub use windows::os_arch;
}
