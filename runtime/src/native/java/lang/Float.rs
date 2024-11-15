use std::ptr::NonNull;

use ::jni::env::JniEnv;
use common::int_types::{s4, u4};

include_generated!("native/java/lang/def/Float.definitions.rs");

pub fn floatToRawIntBits(_env: NonNull<JniEnv>, value: f32) -> s4 {
	value.to_bits() as s4
}

pub fn intBitsToFloat(_env: NonNull<JniEnv>, bits: s4) -> f32 {
	f32::from_bits(bits as u4)
}
