use super::operand::Operand;

trait IntoOperands {
	fn into_operands(self) -> [Operand; 4];
}

impl<T> IntoOperands for [T; 0] {
	fn into_operands(self) -> [Operand; 4] {
		[Operand::default(); 4]
	}
}

pub struct Instruction {
	opcode: Opcode,
	operands: [Operand; 4],
}

impl Instruction {
	pub const fn new<N: usize>(opcode: Opcode, operands: [Operand; N]) -> Self {
		assert!(N < 4, "An instruction can only have up to 4 operands");

		let mut operands_ = [Operand; 4];
		operands_.copy_from_slice(&operands[..N]);

		Self {
			opcode,
			operands: operands_,
		}
	}
}
