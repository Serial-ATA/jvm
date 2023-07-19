/// An XMM register. The 128-bit XMM registers are: XMM0 through XMM7; XMM8 through XMM15 are
/// available using REX.R in 64-bit mode.
#[rustfmt::skip]
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum XMM {
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
