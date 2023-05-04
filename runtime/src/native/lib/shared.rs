#![rustfmt::skip]

pub const CPU_ENDIAN: &str = if cfg!(target_endian = "big") { "big" } else { "little" };