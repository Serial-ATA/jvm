use crate::error::{JniError, Result};
use crate::java_vm::JavaVm;

impl super::JniEnv {
	/// Returns the Java VM interface (used in the Invocation API) associated with the current thread.
	pub fn get_java_vm(&self) -> Result<JavaVm> {
		let mut vm = std::ptr::null_mut();
		let ret;
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).GetJavaVM)(self.0.cast::<jni_sys::JNIEnv>(), &raw mut vm);
		}

		if ret.is_negative() || vm.is_null() {
			return Err(JniError::Unknown);
		}

		Ok(unsafe { JavaVm::from_raw(vm) })
	}
}
