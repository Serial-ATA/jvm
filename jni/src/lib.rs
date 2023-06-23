#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

mod error;

/// `jni_sys` reexports
pub mod sys {
	pub use jni_sys::*;
}
