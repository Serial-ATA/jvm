use crate::objects::class::Class;

use ::jni::env::JniEnv;
use common::int_types::{s4, u4};

include_generated!("native/java/lang/def/Float.definitions.rs");

pub fn floatToRawIntBits(_env: JniEnv, _class: &'static Class, value: f32) -> s4 {
	value.to_bits() as s4
}

pub fn intBitsToFloat(_env: JniEnv, _class: &'static Class, bits: s4) -> f32 {
	f32::from_bits(bits as u4)
}
