cfg_select! {
	target_arch = "x86" => {
		mod x86;
		pub use x86::*;
	}
	target_arch = "x86_64" => {
		mod x86;
		pub use x86::*;
	}
	_ => {
		compile_error!("target architecture is not supported!");
	}
}
