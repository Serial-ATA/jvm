cfg_if::cfg_if! {
	if #[cfg(target_arch = "x86")] {
		mod x86;
		pub use x86::*;
	} else if #[cfg(target_arch = "x86_64")] {
		mod x86;
		pub use x86::*;
	} else {
		compile_error!("target architecture is not supported!");
	}
}
