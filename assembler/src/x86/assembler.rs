pub struct Assembler {}

impl Assembler {
	/// REX prefix
	const REX: u8 = 0x40;
	/// REX.W prefix
	const REX_B: u8 = 0x41;
	/// REX.X prefix
	const REX_X: u8 = 0x42;
	/// REX.R prefix
	const REX_R: u8 = 0x44;
	/// REX.W prefix
	const REX_W: u8 = 0x48;

	fn write_byte(&mut self, _byte: u8) {}

	/// Emits a one byte opcode
	fn opcode(&mut self, code: u8) {
		self.write_byte(code);
	}

	/// Emits a two byte opcode
	fn opcode2(&mut self, code: u8, code2: u8) {
		self.write_byte(code);
		self.write_byte(code2);
	}

	/// Emits a three byte opcode
	fn opcode3(&mut self, code: u8, code2: u8, code3: u8) {
		self.write_byte(code);
		self.write_byte(code2);
		self.write_byte(code3);
	}
}
