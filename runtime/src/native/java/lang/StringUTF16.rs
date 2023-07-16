use crate::include_generated;

use common::int_types::s4;

include_generated!("native/java/lang/def/StringUTF16.definitions.rs");

pub fn isBigEndian() -> s4 {
	s4::from(cfg!(target_endian = "big"))
}
