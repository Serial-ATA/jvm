use crate::include_generated;
use crate::native::JNIEnv;

use common::int_types::s4;

include_generated!("native/java/lang/def/StringUTF16.definitions.rs");

pub fn isBigEndian(_env: JNIEnv) -> s4 {
	s4::from(cfg!(target_endian = "big"))
}
