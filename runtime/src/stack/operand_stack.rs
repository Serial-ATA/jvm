use crate::reference::Reference;

use common::int_types::{s4, s8};

use instructions::{ConstOperandType, Operand, StackLike};

macro_rules! trace_stack {
	($operation:ident, $value:ident) => {{
		#[cfg(debug_assertions)]
		{
			log::trace!(
				"[STACK      ] {} - received value: {:?}",
				stringify!($operation),
				$value
			);
		}
	}};
	($operation:ident) => {{
		#[cfg(debug_assertions)]
		{
			log::trace!("[STACK      ] {}", stringify!($operation));
		}
	}};
}

// https://docs.oracle.com/javase/specs/jvms/se19/html/jvms-2.html#jvms-2.6.2
#[derive(Debug, Clone, PartialEq)]
pub struct OperandStack {
	inner: Vec<Operand<Reference>>,
}

impl OperandStack {
	pub fn new(capacity: usize) -> Self {
		Self {
			inner: Vec::with_capacity(capacity),
		}
	}
}

impl StackLike<Reference> for OperandStack {
	fn push_op(&mut self, op: Operand<Reference>) {
		trace_stack!(push_op, op);
		let needs_empty = matches!(op, Operand::Long(_) | Operand::Double(_));
		self.inner.push(op);
		if needs_empty {
			self.inner.push(Operand::Empty)
		}
	}

	fn push_int(&mut self, int: s4) {
		trace_stack!(push_int, int);
		self.inner.push(Operand::Int(int));
	}

	fn push_float(&mut self, float: f32) {
		trace_stack!(push_float, float);
		self.inner.push(Operand::Float(float));
	}

	fn push_double(&mut self, double: f64) {
		trace_stack!(push_double, double);
		self.inner.push(Operand::Double(double));
	}

	fn push_long(&mut self, long: s8) {
		trace_stack!(push_long, long);
		self.inner.push(Operand::Long(long));
	}

	fn push_reference(&mut self, reference: Reference) {
		trace_stack!(push_reference, reference);
		self.inner.push(Operand::Reference(reference))
	}

	fn pop(&mut self) -> Operand<Reference> {
		match self.inner.pop() {
			Some(op) => {
				trace_stack!(pop, op);
				op
			},
			_ => panic!("Stack underflow error!"),
		}
	}

	fn pop2(&mut self) {
		trace_stack!(pop2);
		self.inner.pop();
		self.inner.pop();
	}

	fn popn(&mut self, count: usize) -> Vec<Operand<Reference>> {
		trace_stack!(popn, count);
		assert!(self.inner.len() >= count);

		let split_pos = self.inner.len() - count;
		self.inner.split_off(split_pos)
	}

	fn pop_int(&mut self) -> s4 {
		trace_stack!(pop_int);
		let op = self.pop();
		match op {
			Operand::Constm1 => -1,
			Operand::Const0(ConstOperandType::Int) => 0,
			Operand::Const1(ConstOperandType::Int) => 1,
			Operand::Const2(ConstOperandType::Int) => 2,
			Operand::Const3 => 3,
			Operand::Const4 => 4,
			Operand::Const5 => 5,
			Operand::Int(int) => int,
			_ => panic!("Unexpected operand type, wanted `int` got {:?}", op),
		}
	}

	fn pop_float(&mut self) -> f32 {
		trace_stack!(pop_float);
		let op = self.pop();
		match op {
			Operand::Constm1 => -1.0,
			Operand::Const0(ConstOperandType::Float) => 0.0,
			Operand::Const1(ConstOperandType::Float) => 1.0,
			Operand::Const2(ConstOperandType::Float) => 2.0,
			Operand::Const3 => 3.0,
			Operand::Const4 => 4.0,
			Operand::Const5 => 5.0,
			Operand::Float(float) => float,
			_ => panic!("Unexpected operand type, wanted `float` got {:?}", op),
		}
	}

	fn pop_double(&mut self) -> f64 {
		trace_stack!(pop_double);
		let op = self.pop();
		assert_eq!(
			self.pop(),
			Operand::Empty,
			"Double only occupied single slot on stack!"
		);

		match op {
			Operand::Constm1 => -1.0,
			Operand::Const0(ConstOperandType::Double) => 0.0,
			Operand::Const1(ConstOperandType::Double) => 1.0,
			Operand::Const2(ConstOperandType::Double) => 2.0,
			Operand::Const3 => 3.0,
			Operand::Const4 => 4.0,
			Operand::Const5 => 5.0,
			Operand::Double(double) => double,
			_ => panic!("Unexpected operand type, wanted `double` got {:?}", op),
		}
	}

	fn pop_long(&mut self) -> s8 {
		trace_stack!(pop_long);
		let op = self.pop();
		assert_eq!(
			self.pop(),
			Operand::Empty,
			"Long only occupied single slot on stack!"
		);

		match op {
			Operand::Constm1 => -1,
			Operand::Const0(ConstOperandType::Long) => 0,
			Operand::Const1(ConstOperandType::Long) => 1,
			Operand::Const2(ConstOperandType::Long) => 2,
			Operand::Const3 => 3,
			Operand::Const4 => 4,
			Operand::Const5 => 5,
			Operand::Long(long) => long,
			_ => panic!("Unexpected operand type, wanted `long` got {:?}", op),
		}
	}

	fn pop_reference(&mut self) -> Reference {
		trace_stack!(pop_reference);
		let op = self.pop();

		match op {
			Operand::Reference(ref_) => ref_,
			_ => panic!("Unexpected operand type, wanted `reference` got {:?}", op),
		}
	}

	fn dup(&mut self) {
		trace_stack!(dup);
		let top_of_stack = self.pop();
		self.inner.push(top_of_stack.clone());
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
		trace_stack!(swap);
		let val = self.pop();
		let val2 = self.pop();
		self.inner.push(val);
		self.inner.push(val2);
	}
}
