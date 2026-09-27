use std::ops::Deref;

#[repr(transparent)]
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

#[repr(transparent)]
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

impl<T: Copy> Copy for ForceSendSync<T> {}

impl<T: Clone> Clone for ForceSendSync<T> {
	fn clone(&self) -> Self {
		Self(self.0.clone())
	}
}

impl<T: core::fmt::Debug> core::fmt::Debug for ForceSendSync<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		core::fmt::Debug::fmt(&self.0, f)
	}
}
