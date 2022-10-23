use crate::operand::Operand;

pub trait StackLike {
	fn push_op(&mut self, op: Operand);
	fn push_int(&mut self, int: i32);
	fn push_float(&mut self, float: f32);
	fn push_double(&mut self, double: f64);
	fn push_long(&mut self, long: i64);

	fn pop(&mut self) -> Operand;
	fn pop2(&mut self);
	fn pop_int(&mut self) -> i32;
	fn pop_float(&mut self) -> f32;
	fn pop_double(&mut self) -> f64;
	fn pop_long(&mut self) -> i64;

	fn dup(&mut self);
	fn dup_x1(&mut self);
	fn dup_x2(&mut self);

	fn dup2(&mut self);
	fn dup2_x1(&mut self);
	fn dup2_x2(&mut self);

	fn swap(&mut self);
}

#[derive(Debug, Clone, PartialEq)]
pub struct DefaultStack {
	inner: Vec<Operand>
}

impl DefaultStack {
	pub fn new(capacity: usize) -> Self {
		Self {
			inner: Vec::with_capacity(capacity)
		}
	}
}

impl StackLike for DefaultStack {
	fn push_op(&mut self, op: Operand) {
		self.inner.push(op);
	}

	fn push_int(&mut self, int: i32) {
		self.inner.push(Operand::Int(int));
	}

	fn push_float(&mut self, float: f32) {
		self.inner.push(Operand::Float(float));
	}

	fn push_double(&mut self, double: f64) {
		self.inner.push(Operand::Double(double));
	}

	fn push_long(&mut self, long: i64) {
		self.inner.push(Operand::Long(long));
	}

	fn pop(&mut self) -> Operand {
		match self.inner.pop() {
			Some(op) => op,
			_ => panic!("Stack underflow error!")
		}
	}

	fn pop2(&mut self) {
		self.inner.pop();
		self.inner.pop();
	}

	fn pop_int(&mut self) -> i32 {
		let op = self.pop();
		match op {
			Operand::Constm1 => -1,
			Operand::Const0 => 0,
			Operand::Const1 => 1,
			Operand::Const2 => 2,
			Operand::Const3 => 3,
			Operand::Const4 => 4,
			Operand::Const5 => 5,
			Operand::Int(int) => int,
			_ => panic!("Unexpected operand type, wanted `int` got {:?}", op)
		}
	}

	fn pop_float(&mut self) -> f32 {
		let op = self.pop();
		match op {
			Operand::Constm1 => -1.0,
			Operand::Const0 => 0.0,
			Operand::Const1 => 1.0,
			Operand::Const2 => 2.0,
			Operand::Const3 => 3.0,
			Operand::Const4 => 4.0,
			Operand::Const5 => 5.0,
			Operand::Float(float) => float,
			_ => panic!("Unexpected operand type, wanted `float` got {:?}", op)
		}
	}

	fn pop_double(&mut self) -> f64 {
		let op = self.pop();
		match op {
			Operand::Constm1 => -1.0,
			Operand::Const0 => 0.0,
			Operand::Const1 => 1.0,
			Operand::Const2 => 2.0,
			Operand::Const3 => 3.0,
			Operand::Const4 => 4.0,
			Operand::Const5 => 5.0,
			Operand::Double(double) => double,
			_ => panic!("Unexpected operand type, wanted `double` got {:?}", op)
		}
	}

	fn pop_long(&mut self) -> i64 {
		let op = self.pop();
		match op {
			Operand::Constm1 => -1,
			Operand::Const0 => 0,
			Operand::Const1 => 1,
			Operand::Const2 => 2,
			Operand::Const3 => 3,
			Operand::Const4 => 4,
			Operand::Const5 => 5,
			Operand::Long(long) => long,
			_ => panic!("Unexpected operand type, wanted `long` got {:?}", op)
		}
	}

	fn dup(&mut self) {
		let top_of_stack = self.pop();
		self.inner.push(top_of_stack);
		self.inner.push(top_of_stack);
	}

	fn dup_x1(&mut self) {
		todo!()
	}

	fn dup_x2(&mut self) {
		todo!()
	}

	fn dup2(&mut self) {
		todo!()
	}

	fn dup2_x1(&mut self) {
		todo!()
	}

	fn dup2_x2(&mut self) {
		todo!()
	}

	fn swap(&mut self) {
		let val = self.pop();
		let val2 = self.pop();
		self.inner.push(val);
		self.inner.push(val2);
	}
}
