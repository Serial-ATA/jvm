#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod x86;

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
compile_error!("There is no assembler available for this architecture!");

pub type Address = *mut u8;
