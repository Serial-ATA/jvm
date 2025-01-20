use crate::include_generated;
use crate::objects::class::Class;

use ::jni::env::JniEnv;
use common::int_types::s4;

include_generated!("native/java/lang/def/StringUTF16.definitions.rs");

pub fn isBigEndian(_env: JniEnv, _class: &'static Class) -> s4 {
	s4::from(cfg!(target_endian = "big"))
}
