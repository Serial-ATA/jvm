use crate::operand::OperandLike;

use common::int_types::{s4, s8};

pub trait StackLike<T: OperandLike, Reference> {
	fn push_op(&mut self, op: T);
	fn push_int(&mut self, int: s4);
	fn push_float(&mut self, float: f32);
	fn push_double(&mut self, double: f64);
	fn push_long(&mut self, long: s8);
	fn push_reference(&mut self, reference: Reference);

	fn pop(&mut self) -> T;
	fn pop2(&mut self);
	fn popn(&mut self, count: usize) -> Vec<T>;
	fn pop_int(&mut self) -> s4;
	fn pop_float(&mut self) -> f32;
	fn pop_double(&mut self) -> f64;
	fn pop_long(&mut self) -> s8;
	fn pop_reference(&mut self) -> Reference;

	fn dup(&mut self);
	fn dup_x1(&mut self);
	fn dup_x2(&mut self);

	fn dup2(&mut self);
	fn dup2_x1(&mut self);
	fn dup2_x2(&mut self);

	fn swap(&mut self);
}
