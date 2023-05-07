use crate::macros::match_cfg_meta;

// `target_family` specific exports

match_cfg_meta! {
	match cfg(target_family) {
		"unix" => {
			mod unix;
			pub use unix::os_arch;
		},
		"windows" => {
			mod windows;
			pub use windows::os_arch;
		},
		_ => {
			compile_error!("target family is not supported!");
		}
	}
}
