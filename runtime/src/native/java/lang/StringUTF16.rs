use crate::objects::class::ClassPtr;

use ::jni::env::JniEnv;
use common::int_types::s4;

include_generated!("native/java/lang/def/StringUTF16.definitions.rs");

pub fn isBigEndian(_env: JniEnv, _class: ClassPtr) -> s4 {
	s4::from(cfg!(target_endian = "big"))
}
