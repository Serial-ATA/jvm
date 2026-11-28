mod signals;
pub use signals::*;

pub mod containers;
pub mod env;
pub mod libs;
pub mod properties;
// `target_family` specific exports

cfg_select! {
	unix => {
		mod unix;
		use unix as imp;
		pub use unix::signals::*;
		pub use unix::*;
	},
	windows => {
		mod windows;
		use windows as imp;
		pub use windows::signals::*;
		pub use windows::*;
	},
	_ => {
		compile_error!("target family is not supported!");
	},
}
