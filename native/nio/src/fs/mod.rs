cfg_select! {
	unix => {
		mod unix;
		pub use unix::*;
	}
	windows => {
		mod windows;
		pub use windows::*;
	}
	_ => {
		compile_error!("Unsupported platform for libnio");
	}
}
