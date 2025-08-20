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

pub struct ForceSendSync<T>(pub T);

unsafe impl<T> Send for ForceSendSync<T> {}
unsafe impl<T> Sync for ForceSendSync<T> {}

impl<T> ForceSendSync<T> {
	pub const fn new(value: T) -> Self {
		ForceSendSync(value)
	}
}

impl<T> Deref for ForceSendSync<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}
