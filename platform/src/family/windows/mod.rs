pub const JNI_LIB_PREFIX: &str = "";
pub const JNI_LIB_SUFFIX: &str = ".dll";

pub mod io;
pub(super) mod libs;
pub mod mem;
pub mod properties;
pub(super) mod signals;
