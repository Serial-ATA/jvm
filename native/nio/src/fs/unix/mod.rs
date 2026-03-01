#![allow(non_snake_case)]

cfg_select! {
	target_os = "linux" => {
		mod linux;
		pub use linux::*;
	}
	target_os = "macos" => {
		mod macos;
		pub use macos::*;
	}
}

pub mod UnixFileSystem;
pub mod UnixNativeDispatcher;
