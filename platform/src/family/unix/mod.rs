use crate::macros::match_cfg_meta;

// OS specific modules

match_cfg_meta! {
	match cfg(target_os) {
		"linux" => {
			mod linux;

			/// Items for specific OS + architecture combinations
			pub use linux::os_arch as os_arch;
		},
		"macos" => {
			mod macos;

			/// Items for specific OS + architecture combinations
			pub use macos::os_arch as os_arch;
		},
		_ => {
			compile_error!("target OS is not supported!");
		}
	}
}

// Exports

pub mod properties;
