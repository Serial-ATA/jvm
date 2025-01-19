use crate::objects::class::Class;

use std::ptr::NonNull;

use ::jni::env::JniEnv;
use common::int_types::{s8, u8};

include_generated!("native/java/lang/def/Double.definitions.rs");

pub fn doubleToRawLongBits(_env: JniEnv, _class: &'static Class, value: f64) -> s8 {
	value.to_bits() as s8
}

pub fn longBitsToDouble(_env: JniEnv, _class: &'static Class, bits: s8) -> f64 {
	f64::from_bits(bits as u8)
}
