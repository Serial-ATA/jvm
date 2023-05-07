use crate::macros::conditional;

// OS specific modules

conditional! {
	#[cfg(target_os = "linux")]

	mod linux;

	/// Items for specific OS + architecture combinations
	pub use linux::os_arch as os_arch;
}

conditional! {
	#[cfg(target_os = "macos")]

	mod macos;

	/// Items for specific OS + architecture combinations
	pub use macos::os_arch as os_arch;
}

// Exports

pub mod properties;
