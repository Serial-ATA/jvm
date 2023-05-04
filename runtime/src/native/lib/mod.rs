mod shared;
mod unix;
mod windows;

pub mod properties {
	#[cfg(target_family = "unix")]
	pub use unix::properties::*;
	#[cfg(target_family = "windows")]
	pub use windows::properties::*;

	pub use shared::*;
}
