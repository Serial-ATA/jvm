mod linux;
mod macos;
mod shared;

pub mod properties {
	#[cfg(target_os = "linux")]
	pub use super::linux::properties::*;
	#[cfg(target_os = "macos")]
	pub use super::macos::properties::*;

	pub use super::shared::*;
}
