impl super::JniEnv {
	// TODO: Throw
	// TODO: ThrowNew
	// TODO: ExceptionOccurred

	/// Prints an exception and a backtrace of the stack to a system error-reporting channel, such as stderr.
	///
	/// This is a convenience routine provided for debugging.
	pub fn exception_describe(&self) {
		unsafe {
			let invoke_interface = self.as_native_interface();
			((*invoke_interface).ExceptionDescribe)(self.0 as _);
		}
	}
	// TODO: ExceptionClear
	// TODO: FatalError

	/// Returns `true` when there is a pending exception
	pub fn exception_check(&self) -> bool {
		let ret;
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).ExceptionCheck)(self.0 as _);
		}

		ret
	}
}
