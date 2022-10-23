#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operand {
	ConstM1,
	Const0,
	Const1,
	Const2,
	Const3,
	Const4,
	Const5,
	Int(i32),
	Float(f32),
	Double(f64),
	Long(i64),
	// TODO: References
}
