use std::ops::Deref;

pub struct ForceSync<T>(pub T);

unsafe impl<T> Sync for ForceSync<T> {}

impl<T> ForceSync<T> {
	pub const fn new(value: T) -> Self {
		ForceSync(value)
	}
}

impl<T> Deref for ForceSync<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
