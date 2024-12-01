mod signals;
pub use signals::*;

use crate::macros::match_cfg_meta;

// `target_family` specific exports

match_cfg_meta! {
	match cfg(target_family) {
		"unix" => {
			mod unix;
			pub use unix::*;
			pub use unix::signals::*;
		},
		"windows" => {
			mod windows;
			pub use windows::*;
			pub use windows::signals::*;
		},
		_ => {
			compile_error!("target family is not supported!");
		}
	}
}
