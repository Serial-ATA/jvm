use crate::x86::operand::Operand;

trait ByteRegister: Into<Operand> {
	fn is_rh(self) -> bool;
}

/// One of the byte general-purpose registers: AL, CL, DL, BL, BPL, SPL, DIL, and SIL; or
/// one of the byte registers (R8B - R15B) available when using REX.R and 64-bit mode.
///
/// The Rh registers (AH, CH, DH, BH) have been split out to [`Rh`]. They can be used in the
/// same contexts as an [`R8`].
#[repr(u8)]
#[rustfmt::skip]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum R8 {
	AL,
	CL,
	DL,
	BL,

	SPL,
	BPL,
	SIL,
	DIL,

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

impl Into<Operand> for R8 {
	fn into(self) -> Operand {
		todo!()
	}
}

impl ByteRegister for R8 {
	#[inline]
	fn is_rh(self) -> bool {
		false
	}
}

/// One of the byte general-purpose registers: AH, CH, DH, and BH
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Rh {
	AH,
	CH,
	DH,
	BH,
}

impl Into<Operand> for Rh {
	fn into(self) -> Operand {
		todo!()
	}
}

impl ByteRegister for Rh {
	#[inline]
	fn is_rh(self) -> bool {
		true
	}
}

/// One of the word general-purpose registers: AX, CX, DX, BX, SP, BP, SI, DI; or one of the word registers
/// (R8-R15) available when using REX.R and 64-bit mode.
#[repr(u8)]
#[rustfmt::skip]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum R16 {
	AX,
	CX,
	DX,
	BX,
	SP,
	BP,
	SI,
	DI,

	// x86_64

	#[cfg(target_arch = "x86_64")] R8W,
	#[cfg(target_arch = "x86_64")] R9W,
	#[cfg(target_arch = "x86_64")] R10W,
	#[cfg(target_arch = "x86_64")] R11W,
	#[cfg(target_arch = "x86_64")] R12W,
	#[cfg(target_arch = "x86_64")] R13W,
	#[cfg(target_arch = "x86_64")] R14W,
	#[cfg(target_arch = "x86_64")] R15W,
}

impl Into<Operand> for R16 {
	fn into(self) -> Operand {
		todo!()
	}
}

/// One of the doubleword general-purpose registers: EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI; or one of
/// the doubleword registers (R8D - R15D) available when using REX.R in 64-bit mode.
#[repr(u8)]
#[rustfmt::skip]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum R32 {
	EAX,
	ECX,
	EDX,
	EBX,
	ESP,
	EBP,
	ESI,
	EDI,

	// x86_64

	#[cfg(target_arch = "x86_64")] R8D,
	#[cfg(target_arch = "x86_64")] R9D,
	#[cfg(target_arch = "x86_64")] R10D,
	#[cfg(target_arch = "x86_64")] R11D,
	#[cfg(target_arch = "x86_64")] R12D,
	#[cfg(target_arch = "x86_64")] R13D,
	#[cfg(target_arch = "x86_64")] R14D,
	#[cfg(target_arch = "x86_64")] R15D,
}

impl Into<Operand> for R32 {
	fn into(self) -> Operand {
		todo!()
	}
}

/// One of the quadword general-purpose registers: RAX, RBX, RCX, RDX, RDI, RSI, RBP, RSP, R8–R15.
/// These are available when using REX.R and 64-bit mode.
#[repr(u8)]
#[rustfmt::skip]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[cfg(target_arch = "x86_64")]
pub enum R64 {
	RAX,
	RBX,
	RCX,
	RDX,
	RDI,
	RSI,
	RBP,
	RSP,
	
	R8,
	R9,
	R10,
	R11,
	R12,
	R13,
	R14,
	R15,
}

impl Into<Operand> for R64 {
	fn into(self) -> Operand {
		todo!()
	}
}
