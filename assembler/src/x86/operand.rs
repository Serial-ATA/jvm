use crate::x86::types::fpu::St;
use crate::x86::types::gpr::{Rh, R16, R32, R64, R8};
use crate::x86::types::immediate::{Imm16, Imm32, Imm64, Imm8};
use crate::x86::types::memory::M8;
use crate::x86::types::relative_address::{Rel16, Rel32, Rel8};
use crate::x86::types::segment_register::SReg;
use crate::x86::types::ymm::Ymm;

#[rustfmt::skip]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Operand {
	#[doc(hidden)]
	/// Exists only for empty operand storage
	NONE,

	// Relative address

	/// A relative address in the range from 128 bytes before the end of the instruction to 127 bytes after the end of the instruction.
	REL8(Rel8),
	/// A relative address within the same code segment as the instruction assembled.
	/// The rel16 symbol applies to instructions with an operand-size attribute of 16 bits
	REL16(Rel16),
	/// A relative address within the same code segment as the instruction assembled.
	/// The rel32 symbol applies to instructions with an operand-size attribute of 32 bits.
	REL32(Rel32),

	// Far pointers

	/// A far pointer, typically to a code segment different from that of the instruction. The
	/// notation 16:16 indicates that the value of the pointer has two parts. The value to the left of the colon is a 16-
	/// bit selector or value destined for the code segment register. The value to the right corresponds to the offset
	/// within the destination segment. The ptr16:16 symbol is used when the instruction's operand-size attribute is
	/// 16 bits.
	FARPTR16_16(FarPtr16_16),
	/// A far pointer, typically to a code segment different from that of the instruction. The
	/// notation 16:32 indicates that the value of the pointer has two parts. The value to the left of the colon is a 16-
	/// bit selector or value destined for the code segment register. The value to the right corresponds to the offset
	/// within the destination segment. The ptr16:32 symbol is used when the operand-size attribute is 32 bits.
	FARPTR16_32(FarPtr16_32),
	
	// General-purpose registers

	/// One of the byte general-purpose registers: AL, CL, DL, BL, BPL, SPL, DIL, and SIL; or
	/// one of the byte registers (R8B - R15B) available when using REX.R and 64-bit mode.
	R8(R8),
	RH(Rh),
	/// One of the word general-purpose registers: AX, CX, DX, BX, SP, BP, SI, DI; or one of the word registers
	/// (R8-R15) available when using REX.R and 64-bit mode.
	R16(R16),
	/// One of the doubleword general-purpose registers: EAX, ECX, EDX, EBX, ESP, EBP, ESI, EDI; or one of
	/// the doubleword registers (R8D - R15D) available when using REX.R in 64-bit mode.
	R32(R32),
	/// One of the quadword general-purpose registers: RAX, RBX, RCX, RDX, RDI, RSI, RBP, RSP, R8–R15.
	/// These are available when using REX.R and 64-bit mode.
	R64(R64),
	
	// Immediate value

	/// An immediate byte value. The imm8 symbol is a signed number between –128 and +127 inclusive.
	/// For instructions in which imm8 is combined with a word or doubleword operand, the immediate value is sign-
	/// extended to form a word or doubleword. The upper byte of the word is filled with the topmost bit of the
	/// immediate value.
	IMM8(Imm8),
	/// An immediate word value used for instructions whose operand-size attribute is 16 bits. This is a
	/// number between –32,768 and +32,767 inclusive.
	IMM16(Imm16),
	/// An immediate doubleword value used for instructions whose operand-size attribute is 32
	/// bits. It allows the use of a number between +2,147,483,647 and –2,147,483,648 inclusive.
	IMM32(Imm32),
	/// An immediate quadword value used for instructions whose operand-size attribute is 64 bits.
	/// The value allows the use of a number between +9,223,372,036,854,775,807 and –
	/// 9,223,372,036,854,775,808 inclusive.
	IMM64(Imm64),

	// In-memory operands

	/// A byte operand in memory, usually expressed as a variable or array name, but pointed to by the
	/// DS:(E)SI or ES:(E)DI registers. In 64-bit mode, it is pointed to by the RSI or RDI registers.
	M8(M8),
	/// A word operand in memory, usually expressed as a variable or array name, but pointed to by the
	/// DS:(E)SI or ES:(E)DI registers. This nomenclature is used only with the string instructions.
	M16(M16),
	/// A doubleword operand in memory. The contents of memory are found at the address provided by the
	/// effective address computation.
	M32(M32),
	/// A memory quadword operand in memory.
	M64(M64),
	/// A memory double quadword operand in memory.
	M128(M128),
	/// A 32-byte operand in memory. This nomenclature is used only with AVX instructions.
	M256(M256),
	/// A 64-byte operand in memory.
	M512(M512),

	/// A Binary Coded Decimal (BCD) operand in memory, 80 bits.
	M80BCD(M80Bcd),
	
	/// A simple memory variable (memory offset) of type byte used by some variants of the MOV
	/// instruction. The actual address is given by a simple offset relative to the segment base.
	/// No ModR/M byte is used in the instruction. The number shown with moffs indicates its size,
	/// which is determined by the address-size attribute of the instruction.
	MOFFS8(MOffs8),
	/// A simple memory variable (memory offset) of type word used by some variants of the MOV
	/// instruction. The actual address is given by a simple offset relative to the segment base.
	/// No ModR/M byte is used in the instruction. The number shown with moffs indicates its size,
	/// which is determined by the address-size attribute of the instruction.
	MOFFS16(MOffs16),
	/// A simple memory variable (memory offset) of type doubleword used by some variants of the MOV
	/// instruction. The actual address is given by a simple offset relative to the segment base.
	/// No ModR/M byte is used in the instruction. The number shown with moffs indicates its size,
	/// which is determined by the address-size attribute of the instruction.
	MOFFS32(MOffs32),
	/// A simple memory variable (memory offset) of type quadword used by some variants of the MOV
	/// instruction. The actual address is given by a simple offset relative to the segment base.
	/// No ModR/M byte is used in the instruction. The number shown with moffs indicates its size,
	/// which is determined by the address-size attribute of the instruction.
	MOFFS64(MOffs64),

	/// A single precision floating-point operand in memory that designates a floating-point
	/// value used as an operand for x87 FPU floating-point instructions.
	M32FP(M32Fp),
	/// A double precision floating-point operand in memory that designates a floating-point
	/// value used as an operand for x87 FPU floating-point instructions.
	M64FP(M64Fp),
	/// A double extended-precision floating-point operand in memory that designates a floating-point
	/// value used as an operand for x87 FPU floating-point instructions.
	M80FP(M80Fp),

	/// A word operand in memory that designates an integer that is used as an operand for
	/// x87 FPU integer instructions.
	M16INT(M16Int),
	/// A doubleword operand in memory that designates an integer that is used as an operand for
	/// x87 FPU integer instructions.
	M32INT(M32Int),
	/// A quadword operand in memory that designates an integer that is used as an operand for
	/// x87 FPU integer instructions.
	M64INT(M64Int),

	/// A segment register. The segment register bit assignments are ES = 0, CS = 1, SS = 2, DS = 3,
	/// FS = 4, and GS = 5.
	SREG(SReg),
	
	/// The ith element from the top of the FPU register stack (i := 0 through 7).
	ST(St),
	
	// SIMD registers
	
	/// An MMX register. The 64-bit MMX registers are: MM0 through MM7.
	MM(Mm),
	/// An XMM register. The 128-bit XMM registers are: XMM0 through XMM7; XMM8 through XMM15 are
	/// available using REX.R in 64-bit mode.
	XMM(Xmm),
	/// A YMM register. The 256-bit YMM registers are: YMM0 through YMM7; YMM8 through YMM15 are
	/// available in 64-bit mode.
	YMM(Ymm),
}
