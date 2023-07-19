/// A segment register. The segment register bit assignments are
/// ES = 0, CS = 1, SS = 2, DS = 3, FS = 4, and GS = 5.
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SReg {
	ES = 0,
	CS = 1,
	SS = 2,
	DS = 3,
	FS = 4,
	GS = 5,
}
