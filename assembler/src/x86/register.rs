/// An x86_64 general-purpose register
#[rustfmt::skip]
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum GPRegister {
    RAX = 0,
    RBX = 1,
    RCX = 2,
    RDX = 3,
    RSI = 4,
    RDI = 5,
    RBP = 6,
    RSP = 7,

    // x86_64

    #[cfg(target_arch = "x86_64")] R8 = 8,
    #[cfg(target_arch = "x86_64")] R9 = 9,
    #[cfg(target_arch = "x86_64")] R10 = 10,
    #[cfg(target_arch = "x86_64")] R11 = 11,
    #[cfg(target_arch = "x86_64")] R12 = 12,
    #[cfg(target_arch = "x86_64")] R13 = 13,
    #[cfg(target_arch = "x86_64")] R14 = 14,
    #[cfg(target_arch = "x86_64")] R15 = 15,
}

/// An x87 floating-point register
#[rustfmt::skip]
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum FloatRegister {
    ST0 = 0,
    ST1 = 1,
    ST2 = 2,
    ST3 = 3,
    ST4 = 4,
    ST5 = 5,
    ST6 = 6,
    ST7 = 7,
    ST8 = 8,
}

/// 128-bit x86_64 SIMD registers
#[rustfmt::skip]
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SSERegister {
    XMM0 = 0,
    XMM1 = 1,
    XMM2 = 2,
    XMM3 = 3,
    XMM4 = 4,
    XMM5 = 5,
    XMM6 = 6,
    XMM7 = 7,

    // x86_64

    #[cfg(target_arch = "x86_64")] XMM8  = 8,
    #[cfg(target_arch = "x86_64")] XMM9  = 9,
    #[cfg(target_arch = "x86_64")] XMM10 = 10,
    #[cfg(target_arch = "x86_64")] XMM11 = 11,
    #[cfg(target_arch = "x86_64")] XMM12 = 12,
    #[cfg(target_arch = "x86_64")] XMM13 = 13,
    #[cfg(target_arch = "x86_64")] XMM14 = 14,
    #[cfg(target_arch = "x86_64")] XMM15 = 15,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] XMM16 = 16,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] XMM17 = 17,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] XMM18 = 18,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] XMM19 = 19,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] XMM20 = 20,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] XMM21 = 21,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] XMM22 = 22,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] XMM23 = 23,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] XMM24 = 24,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] XMM25 = 25,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] XMM26 = 26,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] XMM27 = 27,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] XMM28 = 28,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] XMM29 = 29,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] XMM30 = 30,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] XMM31 = 31,
}

/// 128-bit x86_64 SIMD registers
#[rustfmt::skip]
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum MaskRegister {
    K0 = 0,
    K1 = 1,
    K2 = 2,
    K3 = 3,
    K4 = 4,
    K5 = 5,
    K6 = 6,
    K7 = 7,
}
