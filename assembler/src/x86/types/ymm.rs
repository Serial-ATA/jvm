/// A YMM register. The 256-bit YMM registers are: YMM0 through YMM7; YMM8 through YMM15 are
/// available in 64-bit mode.
#[rustfmt::skip]
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Ymm {
    YMM0 = 0,
    YMM1 = 1,
    YMM2 = 2,
    YMM3 = 3,
    YMM4 = 4,
    YMM5 = 5,
    YMM6 = 6,
    YMM7 = 7,

    // x86_64

    #[cfg(target_arch = "x86_64")] YMM8  = 8,
    #[cfg(target_arch = "x86_64")] YMM9  = 9,
    #[cfg(target_arch = "x86_64")] YMM10 = 10,
    #[cfg(target_arch = "x86_64")] YMM11 = 11,
    #[cfg(target_arch = "x86_64")] YMM12 = 12,
    #[cfg(target_arch = "x86_64")] YMM13 = 13,
    #[cfg(target_arch = "x86_64")] YMM14 = 14,
    #[cfg(target_arch = "x86_64")] YMM15 = 15,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] YMM16 = 16,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] YMM17 = 17,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] YMM18 = 18,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] YMM19 = 19,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] YMM20 = 20,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] YMM21 = 21,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] YMM22 = 22,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] YMM23 = 23,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] YMM24 = 24,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] YMM25 = 25,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] YMM26 = 26,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] YMM27 = 27,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] YMM28 = 28,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] YMM29 = 29,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] YMM30 = 30,
    #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] YMM31 = 31,
}
