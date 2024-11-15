use crate::native::JniEnv;

use std::ptr::NonNull;

use common::int_types::{s8, u8};

include_generated!("native/java/lang/def/Double.definitions.rs");

pub fn doubleToRawLongBits(_env: NonNull<JniEnv>, value: f64) -> s8 {
	value.to_bits() as s8
}

pub fn longBitsToDouble(_env: NonNull<JniEnv>, bits: s8) -> f64 {
	f64::from_bits(bits as u8)
}
