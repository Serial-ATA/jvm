use std::borrow::Cow;

extern "C" fn default_handler(sig: libc::c_int) {
	todo!()
}

#[derive(Copy, Clone, PartialEq)]
pub struct SignalHandlerT(usize);

impl SignalHandlerT {
	#[inline]
	#[allow(trivial_casts)]
	pub fn user_handler() -> Self {
		Self(default_handler as *const extern "C" fn(libc::c_int) as usize);
	}

	#[inline]
	pub fn as_usize(self) -> usize {
		self.0
	}

	pub unsafe fn from_raw(handler: usize) -> Self {
		Self(handler)
	}
}

impl crate::SignalOsExt for crate::Signal {
	fn from_name_impl(name: Cow<'_, str>) -> Option<Self> {
		match &*name {
			"ABRT" => Some(Self(libc::SIGABRT)),
			"FPE" => Some(Self(libc::SIGFPE)),
			"SEGV" => Some(Self(libc::SIGSEGV)),
			"INT" => Some(Self(libc::SIGINT)),
			"TERM" => Some(Self(libc::SIGTERM)),
			"BREAK" => Some(Self(libc::SIGBREAK)),
			"ILL" => Some(Self(libc::SIGILL)),
			_ => None,
		}
	}

	fn registration_allowed_impl(self) -> bool {
		self.0 != libc::SIGFPE
	}

	unsafe fn install_impl(self, handler: crate::SignalHandler) -> Option<crate::SignalHandler> {
		todo!("windows::Signal::install")
	}
}
