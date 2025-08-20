mod signals;
pub use signals::*;

pub mod env;
pub mod libs;
pub mod properties;

// `target_family` specific exports

cfg_if::cfg_if! {
	if #[cfg(unix)] {
		mod unix;
		use unix as imp;
		pub use unix::*;
		pub use unix::signals::*;
	} else if #[cfg(windows)] {
		mod windows;
		use windows as imp;
		pub use windows::*;
		pub use windows::signals::*;
	} else {
		compile_error!("target family is not supported!");
	}
}
