/// An x87 floating-point register
#[rustfmt::skip]
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum St {
    ST0,
    ST1,
    ST2,
    ST3,
    ST4,
    ST5,
    ST6,
    ST7,
    ST8,
}
