pub trait StackLike {
	fn push_int(&mut self, int: i32);
	fn push_float(&mut self, float: f32);
	fn push_double(&mut self, double: f64);
	fn push_long(&mut self, long: i64);
	fn push_null(&mut self);

	fn pop_int();
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
