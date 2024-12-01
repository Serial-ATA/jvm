use std::borrow::Cow;
use std::mem::MaybeUninit;

extern "C" fn default_handler(
	sig: libc::c_int,
	info: *mut libc::siginfo_t,
	context: *mut libc::c_void,
) {
	todo!()
}

#[derive(Copy, Clone, PartialEq)]
pub enum SignalHandlerT {
	Handler(extern "C" fn(libc::c_int)),
	SigAction(extern "C" fn(libc::c_int, *mut libc::siginfo_t, *mut libc::c_void)),
	Indiscriminate(usize),
}

impl SignalHandlerT {
	#[inline]
	#[allow(trivial_casts)]
	pub fn as_usize(&self) -> usize {
		match self {
			Self::Handler(handler) => handler as *const extern "C" fn(libc::c_int) as usize,
			Self::SigAction(action) => {
				action as *const extern "C" fn(libc::c_int, *mut libc::siginfo_t, *mut libc::c_void)
					as usize
			},
			Self::Indiscriminate(indiscriminate) => *indiscriminate,
		}
	}

	pub fn user_handler() -> Self {
		Self::SigAction(default_handler)
	}

	pub unsafe fn from_raw(handler: usize) -> Self {
		Self::Indiscriminate(handler)
	}
}

impl crate::SignalOsExt for crate::Signal {
	fn from_name_impl(mut name: Cow<'_, str>) -> Option<Self> {
		if !name.starts_with("SIG") {
			name = Cow::Owned(format!("SIG{}", name));
		}

		match &*name {
			"SIGHUP" => Some(Self(libc::SIGHUP)),
			"SIGINT" => Some(Self(libc::SIGINT)),
			"SIGQUIT" => Some(Self(libc::SIGQUIT)),
			"SIGILL" => Some(Self(libc::SIGILL)),
			"SIGABRT" => Some(Self(libc::SIGABRT)),
			"SIGFPE" => Some(Self(libc::SIGFPE)),
			"SIGKILL" => Some(Self(libc::SIGKILL)),
			"SIGSEGV" => Some(Self(libc::SIGSEGV)),
			"SIGPIPE" => Some(Self(libc::SIGPIPE)),
			"SIGALRM" => Some(Self(libc::SIGALRM)),
			"SIGTERM" => Some(Self(libc::SIGTERM)),
			"SIGUSR1" => Some(Self(libc::SIGUSR1)),
			"SIGUSR2" => Some(Self(libc::SIGUSR2)),
			"SIGCHLD" => Some(Self(libc::SIGCHLD)),
			"SIGCONT" => Some(Self(libc::SIGCONT)),
			"SIGSTOP" => Some(Self(libc::SIGSTOP)),
			"SIGTSTP" => Some(Self(libc::SIGTSTP)),
			"SIGTTIN" => Some(Self(libc::SIGTTIN)),
			"SIGTTOU" => Some(Self(libc::SIGTTOU)),
			_ => unimplemented!("Signal not supported: {}", name),
		}
	}

	fn registration_allowed_impl(self) -> bool {
		const DISALLOWED: &[libc::c_int] = &[
			libc::SIGFPE,
			libc::SIGILL,
			libc::SIGSEGV,
			libc::SIGQUIT,
			#[cfg(target_os = "macos")]
			libc::SIGBUS,
		];

		!DISALLOWED.contains(&self.0)
	}

	#[allow(non_camel_case_types)]
	unsafe fn install_impl(self, handler: crate::SignalHandler) -> Option<crate::SignalHandler> {
		const ERROR_SIGNALS_TO_REMOVE: [libc::c_int; 5] = [
			libc::SIGILL,
			libc::SIGBUS,
			libc::SIGFPE,
			libc::SIGSEGV,
			libc::SIGTRAP,
		];

		type sa_sigaction_t = usize;

		union sigaction_t {
			sa_handler: libc::sighandler_t,
			sa_sigaction: sa_sigaction_t,
		}

		let mut flags = libc::SA_RESTART;

		let mut sig_action = unsafe { MaybeUninit::<libc::sigaction>::zeroed().assume_init() };
		let mut old_action = unsafe { MaybeUninit::<libc::sigaction>::zeroed().assume_init() };

		libc::sigfillset(&raw mut sig_action.sa_mask);
		for sig in ERROR_SIGNALS_TO_REMOVE {
			libc::sigdelset(&raw mut sig_action.sa_mask, sig);
		}

		let handler = handler.as_usize();
		if handler == libc::SIG_IGN || handler == libc::SIG_DFL {
			sig_action.sa_sigaction = sigaction_t {
				sa_handler: handler,
			}
			.sa_handler;
		} else {
			sig_action.sa_flags |= libc::SA_SIGINFO;
			sig_action.sa_sigaction = sigaction_t {
				sa_sigaction: handler,
			}
			.sa_sigaction;
		}

		let ok = libc::sigaction(self.0, &raw const sig_action, &raw mut old_action);
		if ok < 0 {
			return None;
		}

		let siginfo = old_action.sa_flags & libc::SA_SIGINFO != 0;
		let old_signal_handler = old_action.sa_sigaction;

		Some(unsafe { crate::SignalHandler::from_raw(old_signal_handler) })
	}
}
