#![cfg_attr(rustfmt, rustfmt_skip)]

// Properties shared between all `target_family`s

pub const CPU_ENDIAN: &str = if cfg!(target_endian = "big") { "big" } else { "little" };

// Export family specific properties

#[cfg(target_family = "unix")]
pub use super::unix::properties::*;
#[cfg(target_family = "windows")]
pub use super::windows::properties::*;