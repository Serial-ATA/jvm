use std::sync::atomic::{
	AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16,
	AtomicU32, AtomicU64, AtomicUsize, Ordering,
};

pub struct AtomicF32 {
	inner: AtomicU32,
}

impl AtomicF32 {
	pub fn new(value: f32) -> Self {
		Self {
			inner: AtomicU32::new(value.to_bits()),
		}
	}
	pub fn store(&self, value: f32, ordering: Ordering) {
		self.inner.store(value.to_bits(), ordering)
	}
	pub fn load(&self, ordering: Ordering) -> f32 {
		let as_u32 = self.inner.load(ordering);
		f32::from_bits(as_u32)
	}
	pub fn compare_exchange(
		&self,
		current: f32,
		new: f32,
		success: Ordering,
		failure: Ordering,
	) -> Result<f32, f32> {
		match self
			.inner
			.compare_exchange(current as u32, new as u32, success, failure)
		{
			Ok(v) => Ok(f32::from_bits(v)),
			Err(v) => Err(f32::from_bits(v)),
		}
	}
}

pub struct AtomicF64 {
	inner: AtomicU64,
}

impl AtomicF64 {
	pub fn new(value: f64) -> Self {
		Self {
			inner: AtomicU64::new(value.to_bits()),
		}
	}
	pub fn store(&self, value: f64, ordering: Ordering) {
		self.inner.store(value.to_bits(), ordering)
	}
	pub fn load(&self, ordering: Ordering) -> f64 {
		let as_u64 = self.inner.load(ordering);
		f64::from_bits(as_u64)
	}
	pub fn compare_exchange(
		&self,
		current: f64,
		new: f64,
		success: Ordering,
		failure: Ordering,
	) -> Result<f64, f64> {
		match self
			.inner
			.compare_exchange(current as u64, new as u64, success, failure)
		{
			Ok(v) => Ok(f64::from_bits(v)),
			Err(v) => Err(f64::from_bits(v)),
		}
	}
}

pub trait Atomic {
	type Output;

	fn new(v: Self::Output) -> Self;
	fn load(&self, order: Ordering) -> Self::Output;
	fn store(&self, val: Self::Output, order: Ordering);
	fn compare_exchange(
		&self,
		current: Self::Output,
		new: Self::Output,
		success: Ordering,
		failure: Ordering,
	) -> Self::Output;
}

pub trait AtomicCounterpart {
	type Counterpart: Atomic<Output = Self>;
}

macro_rules! impl_atomic {
	($(($ty:ty, $output:ty)),+ $(,)?) => {
		$(
		impl Atomic for $ty {
			type Output = $output;

			fn new(v: Self::Output) -> Self {
				Self::new(v)
			}

			fn load(&self, order: Ordering) -> Self::Output {
				self.load(order)
			}

			fn store(&self, val: Self::Output, order: Ordering) {
				self.store(val, order);
			}

			fn compare_exchange(&self, current: Self::Output, new: Self::Output, success: Ordering, failure: Ordering) -> Self::Output {
				self.compare_exchange(current, new, success, failure).unwrap_or_else(|v| {
					v
				})
			}
		}

		impl AtomicCounterpart for $output {
			type Counterpart = $ty;
		}
		)+
	}
}

impl_atomic!(
	(AtomicBool, bool),
	(AtomicU8, u8),
	(AtomicU16, u16),
	(AtomicU32, u32),
	(AtomicU64, u64),
	(AtomicI8, i8),
	(AtomicI16, i16),
	(AtomicI32, i32),
	(AtomicI64, i64),
	(AtomicUsize, usize),
	(AtomicIsize, isize),
	(AtomicF32, f32),
	(AtomicF64, f64),
);
