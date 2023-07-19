/// A mask register used as a regular operand (either destination or source). The 64-bit k registers are: k0
/// through k7.
#[rustfmt::skip]
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum K1 {
    K0,
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
}
