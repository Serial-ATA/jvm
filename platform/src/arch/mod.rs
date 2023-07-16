use crate::macros::match_cfg_meta;

match_cfg_meta! {
	match cfg(target_arch) {
		"x86" => {
			mod x86;
			pub use x86::*;
		},
		"x86_64" => {
			mod x86_64;
			pub use x86_64::*;
		},
		_ => {
			compile_error!("target architecture is not supported!");
		}
	}
}
