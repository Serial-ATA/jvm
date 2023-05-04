mod linux;
mod macos;
mod shared;

pub mod properties {
	#[cfg(target_os = "linux")]
	pub use linux::properties::*;
	#[cfg(target_os = "macos")]
	pub use macos::properties::*;

	pub use super::shared::*;
}
