use crate::error::{JniError, Result};
use crate::objects::JClass;
use crate::string::JniString;
use crate::sys::JNI_TRUE;

impl super::JniEnv {
	// TODO: DefineClass

	/// Find a class from its fully-qualified name
	///
	/// # Parameters
	///
	/// * `name`: The fully-qualified class name (a package name, delimited by “/”, followed by the class name)
	///
	/// # Errors
	///
	/// This will error if an exception is thrown.
	///
	/// Possible exceptions:
	///
	/// * `ClassFormatError`: The class data does not specify a valid class.
	/// * `ClassCircularityError`: The class or interface would be its own superclass or superinterface.
	/// * `NoClassDefFoundError`: No definition for a requested class or interface can be found.
	/// * `OutOfMemoryError`: The system runs out of memory.
	pub fn find_class(&self, name: impl Into<JniString>) -> Result<JClass> {
		assert!(!self.raw().is_null());

		let name = name.into();

		let ret;
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).FindClass)(
				self.0.cast::<jni_sys::JNIEnv>(),
				name.as_cstr().as_ptr(),
			);
		}

		if self.exception_check() {
			return Err(JniError::ExceptionThrown);
		}

		// Native call should've thrown `NoClassDefFoundError`
		assert!(!ret.is_null());
		Ok(unsafe { JClass::from_raw(ret) })
	}

	/// Returns the superclass of the class specified by `class`.
	///
	/// If `class` specifies the class `Object`, or class represents an interface, this function returns `None`.
	///
	/// ## PARAMETERS
	///
	/// `class`: a Java class object.
	///
	/// ## RETURNS
	///
	/// Returns the superclass of the class represented by `class`, or `None`.
	pub fn get_super_class(&self, sub: JClass) -> Option<JClass> {
		assert!(!self.raw().is_null());

		let ret;
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).GetSuperclass)(self.0.cast::<jni_sys::JNIEnv>(), sub.raw());
		}

		if ret.is_null() {
			return None;
		}

		Some(unsafe { JClass::from_raw(ret) })
	}

	/// Determines whether an object of `sub` can be safely cast to `sup`.
	///
	/// ## PARAMETERS
	///
	/// `sub`: the first class argument.
	///
	/// `sup`: the second class argument.
	///
	/// ## RETURNS
	///
	/// Returns `true` if any of the following are true:
	///
	/// * `sub` and `sup` refer to the same Java class.
	/// * `sub` is a subclass of `sup`.
	/// * `sub` has `sup` as one of its interfaces.
	pub fn is_assignable_from(&self, sub: JClass, sup: JClass) -> bool {
		assert!(!self.raw().is_null());

		let ret;
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).IsAssignableFrom)(
				self.0.cast::<jni_sys::JNIEnv>(),
				sub.raw(),
				sup.raw(),
			);
		}

		ret == JNI_TRUE
	}
}
