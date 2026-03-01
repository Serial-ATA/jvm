// OS specific modules

cfg_select! {
	target_os = "linux" => {
		mod linux;
		use linux as imp;
	}
	target_os = "macos" => {
		mod macos;
		use macos as imp;
	}
	_ => {
		compile_error!("target OS is not supported!");
	}
}

// Exports

pub use imp::JNI_LIB_SUFFIX;
pub const JNI_LIB_PREFIX: &str = "lib";

pub mod io;
pub(super) mod libs;
pub(crate) mod locale;
pub mod mem;
pub mod properties;
pub(super) mod signals;
pub use imp::env;
