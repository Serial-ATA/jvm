use std::borrow::Cow;

/// A generic signal handler implementation
///
/// All methods simply defer to their platform-specific implementations
#[derive(Copy, Clone, PartialEq)]
pub struct SignalHandler(super::SignalHandlerT);

impl SignalHandler {
	pub fn user_handler() -> Self {
		Self(super::SignalHandlerT::user_handler())
	}

	pub fn as_usize(self) -> usize {
		self.0.as_usize()
	}

	pub unsafe fn from_raw(handler: usize) -> Self {
		let imp = unsafe { super::SignalHandlerT::from_raw(handler) };
		Self(imp)
	}
}

impl From<SignalHandler> for super::SignalHandlerT {
	fn from(handler: SignalHandler) -> Self {
		handler.0
	}
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Signal(pub(crate) i32);

impl Signal {
	pub const fn value(self) -> i32 {
		self.0
	}

	pub fn from_name<'a, S: Into<Cow<'a, str>>>(name: S) -> Option<Self> {
		<Self as SignalOsExt>::from_name_impl(name.into())
	}

	pub fn registration_allowed(self) -> bool {
		<Self as SignalOsExt>::registration_allowed_impl(self)
	}

	pub unsafe fn install<T: Into<SignalHandler>>(self, handler: T) -> Option<SignalHandler> {
		unsafe { <Self as SignalOsExt>::install_impl(self, handler.into()) }
	}
}

impl From<i32> for Signal {
	fn from(value: i32) -> Self {
		Signal(value)
	}
}

pub trait SignalOsExt: Sized {
	fn from_name_impl(name: Cow<'_, str>) -> Option<Self>;
	fn registration_allowed_impl(self) -> bool;
	unsafe fn install_impl(self, handler: SignalHandler) -> Option<SignalHandler>;
}
