// OS specific modules

cfg_if::cfg_if! {
	if #[cfg(target_os = "linux")] {
		mod linux;
	} else if #[cfg(target_os = "macos")] {
		mod macos;
	} else {
		compile_error!("target OS is not supported!");
	}
}

// Exports

pub mod io;
pub(crate) mod locale;
pub mod mem;
pub mod properties;
pub(super) mod signals;
