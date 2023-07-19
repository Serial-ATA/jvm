use crate::x86::operand::Operand;
use crate::x86::types::gpr::R64;
use crate::x86::types::immediate::Imm32;
use crate::x86::types::segment_register::SReg;

#[rustfmt::skip]
mod masks {
    pub const DISPLACEMENT   : u64 = 0x00000000FFFFFFFF;
    pub const BASE           : u64 = 0x0000001F00000000;
    pub const INDEX          : u64 = 0x00001F0000000000;
    pub const SCALE          : u64 = 0x0003000000000000;
    pub const SEG            : u64 = 0x0700000000000000;
    pub const ADDR_OR        : u64 = 0x1000000000000000;
    pub const RIP            : u64 = 0x2000000000000000;
}

#[rustfmt::skip]
mod indexes {
    pub const DISP   : u64 =  0;
    pub const BASE   : u64 = 32;
    pub const INDEX  : u64 = 40;
    pub const SCALE  : u64 = 48;
    pub const SEG    : u64 = 56;
    pub const ADDR_OR: u64 = 60;
    pub const RIP    : u64 = 61;
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Scale {
    /// `Index*1`
    Times1 = 0,
    /// `Index*2`
    Times2 = 1,
    /// `Index*4`
    Times4 = 2,
    /// `Index*8`
    Times8 = 3,
}

trait MemoryOperand {
    fn segment_register(&self) -> Option<SReg>;
    fn base(&self) -> Option<R64>;
    fn index(&self) -> Option<R64>;
    fn scale(&self) -> Scale;
    fn displacement(&self) -> Imm32;
}

fn create(displacement: Imm32, base: R, index: R,
scale: Scale, segment_register: SReg, addr_or: u64, rip: u64) -> u64 {
    ((displacement.0 as u64) & masks::DISPLACEMENT) |
        ((uint64_t)b << indexes::BASE) |
        ((uint64_t)i << indexes::INDEX) |
        ((scale as u64) << indexes::SCALE) |
        ((segment_register as u64) << indexes::SEG) |
        (addr_or << indexes::ADDR_OR) |
        (rip << indexes::RIP)
}

pub(crate) struct Memory(Operand);

impl Memory {
    fn new(displacement: Imm32, base: R, index: R, scale: Scale) -> Self {
        let val = displacement as u64
        Operand::new_with_type(ty, )
    }
}

/// A 16-, 32- or 64-bit operand in memory.
pub enum M {
    M16(M16),
    M32(M32),
    M64(M64),
}

/// A byte operand in memory, usually expressed as a variable or array name, but pointed to by the
/// DS:(E)SI or ES:(E)DI registers. In 64-bit mode, it is pointed to by the RSI or RDI registers.
pub struct M8(Memory(Operand));

