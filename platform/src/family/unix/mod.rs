use crate::macros::match_cfg_meta;

// OS specific modules

match_cfg_meta! {
	match cfg(target_os) {
		"linux" => {
			mod linux;
		},
		"macos" => {
			mod macos;
		},
		_ => {
			compile_error!("target OS is not supported!");
		}
	}
}

// Exports

pub mod io;
pub mod properties;
pub(super) mod signals;
