#![cfg_attr(rustfmt, rustfmt_skip)]

// Properties shared between all `target_family = "unix"`

pub const FILE_SEPARATOR: &str = "/";
pub const LINE_SEPARATOR: &str = "\n";
pub const OS_ARCH: &str = std::env::consts::ARCH;
pub const PATH_SEPARATOR: &str = ":";
pub const UNICODE_ENCODING: &str = if cfg!(target_endian = "big") { "UnicodeBig" } else { "UnicodeLittle" };

// Export OS specific properties

#[cfg(target_os = "linux")]
pub use super::linux::properties::*;
#[cfg(target_os = "macos")]
pub use super::macos::properties::*;