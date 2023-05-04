mod shared;
mod unix;
mod windows;

pub mod properties {
	#[cfg(target_family = "unix")]
	pub use super::unix::properties::*;
	#[cfg(target_family = "windows")]
	pub use super::windows::properties::*;

	pub use super::shared::*;
}
