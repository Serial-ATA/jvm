#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operand {
	ConstM1,
	Const0,
	Const1,
	Const2,
	Const3,
	Const4,
	Const5,
	Byte(i8),
	Short(i16),
	Char(u16),
	Int(i32),
	Float(f32),
	Double(f64),
	Long(i64),
	// TODO: References
}

impl Operand {
	/// Convert int to byte
	pub fn i2b(&mut self) {
		match self {
			Operand::Int(i) => {
				*self = Operand::Byte(*i as i8)
			},
			_ => panic!("Invalid operand type for `i2b` instruction: {:?}", self)
		}
	}

	/// Convert int to char
	pub fn i2c(&mut self) {
		match self {
			Operand::Int(i) => {
				*self = Operand::Char(*i as u16)
			},
			_ => panic!("Invalid operand type for `i2c` instruction: {:?}", self)
		}
	}

	/// Convert int to double
	pub fn i2d(&mut self) {
		match self {
			Operand::Int(i) => {
				*self = Operand::Double(f64::from(*i))
			},
			_ => panic!("Invalid operand type for `i2d` instruction: {:?}", self)
		}
	}

	/// Convert int to float
	pub fn i2f(&mut self) {
		match self {
			Operand::Int(i) => {
				*self = Operand::Float(*i as f32)
			},
			_ => panic!("Invalid operand type for `i2f` instruction: {:?}", self)
		}
	}

	/// Convert int to long
	pub fn i2l(&mut self) {
		match self {
			Operand::Int(i) => {
				*self = Operand::Long(i64::from(*i))
			},
			_ => panic!("Invalid operand type for `i2l` instruction: {:?}", self)
		}
	}

	/// Convert int to short
	pub fn i2s(&mut self) {
		match self {
			Operand::Int(i) => {
				*self = Operand::Short(*i as i16)
			},
			_ => panic!("Invalid operand type for `i2s` instruction: {:?}", self)
		}
	}

	/// Convert long to int
	pub fn l2i(&mut self) {
		match self {
			Operand::Long(l) => {
				*self = Operand::Int(*l as i32)
			},
			_ => panic!("Invalid operand type for `l2i` instruction: {:?}", self)
		}
	}

	/// Convert long to double
	pub fn l2d(&mut self) {
		match self {
			Operand::Long(l) => {
				*self = Operand::Double(*l as f64)
			},
			_ => panic!("Invalid operand type for `l2d` instruction: {:?}", self)
		}
	}

	/// Convert long to float
	pub fn l2f(&mut self) {
		match self {
			Operand::Long(l) => {
				*self = Operand::Float(*l as f32)
			},
			_ => panic!("Invalid operand type for `l2f` instruction: {:?}", self)
		}
	}

	/// Convert double to float
	pub fn d2f(&mut self) {
		match self {
			Operand::Double(d) => {
				*self = Operand::Float(*d as f32)
			},
			_ => panic!("Invalid operand type for `d2f` instruction: {:?}", self)
		}
	}

	/// Convert double to int
	pub fn d2i(&mut self) {
		match self {
			Operand::Double(d) => {
				*self = Operand::Int(*d as i32)
			},
			_ => panic!("Invalid operand type for `d2i` instruction: {:?}", self)
		}
	}

	/// Convert double to long
	pub fn d2l(&mut self) {
		match self {
			Operand::Double(d) => {
				*self = Operand::Long(*d as i64)
			},
			_ => panic!("Invalid operand type for `d2l` instruction: {:?}", self)
		}
	}

	/// Convert float to double
	pub fn f2d(&mut self) {
		match self {
			Operand::Float(f) => {
				*self = Operand::Double(f64::from(*f))
			},
			_ => panic!("Invalid operand type for `f2d` instruction: {:?}", self)
		}
	}

	/// Convert float to int
	pub fn f2i(&mut self) {
		match self {
			Operand::Float(f) => {
				*self = Operand::Int(*f as i32)
			},
			_ => panic!("Invalid operand type for `f2i` instruction: {:?}", self)
		}
	}

	/// Convert float to long
	pub fn f2l(&mut self) {
		match self {
			Operand::Float(f) => {
				*self = Operand::Long(*f as i64)
			},
			_ => panic!("Invalid operand type for `f2l` instruction: {:?}", self)
		}
	}
}
