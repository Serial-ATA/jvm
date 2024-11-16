use crate::macros::match_cfg_meta;

// `target_family` specific exports

match_cfg_meta! {
	match cfg(target_family) {
		"unix" => {
			mod unix;
		},
		"windows" => {
			mod windows;
		},
		_ => {
			compile_error!("target family is not supported!");
		}
	}
}
