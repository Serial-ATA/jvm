use crate::objects::method::Method;

/// A thin marker frame for native methods
///
/// A stack frame is not usually necessary for native methods, but we still need to keep track of
/// the calls.
///
/// These are useful for certain reflection methods (ex. `Reflection#getCallerClass`) and stacktraces.
#[derive(Debug)]
pub struct NativeFrame {
	pub method: &'static Method,
}

impl NativeFrame {
	/// Get the method associated with this frame
	pub fn method(&self) -> &'static Method {
		self.method
	}
}
