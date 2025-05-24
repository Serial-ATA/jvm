use crate::error::{JniError, Result};
use crate::objects::{JClass, JMethodId, JObject, JValue};
use crate::string::JniString;

impl super::JniEnv {
	// --------------
	//   NON-STATIC
	// --------------

	/// Returns the method ID for an instance (nonstatic) method of a class or interface.
	///
	/// This causes an uninitialized class to be initialized.
	///
	/// The method may be defined in one of the class’s superclasses and inherited by class.
	/// The method is determined by its name and signature.
	///
	/// To obtain the method ID of a constructor, supply `<init>` as the method name and void (V) as the return type.
	///
	/// # Parameters
	///
	/// * `class`: The Java class object
	/// * `name`: The method name
	/// * `sig`: The method signature
	///
	/// # Errors
	///
	/// This will error if an exception is thrown.
	///
	/// Possible exceptions:
	///
	/// * `NoSuchMethodError`: The specified method cannot be found.
	/// * `ExceptionInInitializerError`: The class initializer fails due to an exception.
	/// * `OutOfMemoryError`: The system runs out of memory.
	pub fn get_method_id(
		&self,
		class: JClass,
		name: impl Into<JniString>,
		sig: impl Into<JniString>,
	) -> Result<JMethodId> {
		let name = name.into();
		let sig = sig.into();

		let ret;
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).GetMethodID)(
				self.0 as _,
				class.raw(),
				name.as_cstr().as_ptr(),
				sig.as_cstr().as_ptr(),
			);
		}

		if self.exception_check() {
			return Err(JniError::ExceptionThrown);
		}

		// Native call should've thrown `NoSuchMethodError`
		assert!(!ret.is_null());
		Ok(unsafe { JMethodId::from_raw(ret) })
	}
	// TODO: CallObjectMethod
	// TODO: CallObjectMethodV
	// TODO: CallObjectMethodA
	// TODO: CallBooleanMethod
	// TODO: CallBooleanMethodV
	// TODO: CallBooleanMethodA
	// TODO: CallByteMethod
	// TODO: CallByteMethodV
	// TODO: CallByteMethodA
	// TODO: CallCharMethod
	// TODO: CallCharMethodV
	// TODO: CallCharMethodA
	// TODO: CallShortMethod
	// TODO: CallShortMethodV
	// TODO: CallShortMethodA
	// TODO: CallIntMethod
	// TODO: CallIntMethodV
	// TODO: CallIntMethodA
	// TODO: CallLongMethod
	// TODO: CallLongMethodV
	// TODO: CallLongMethodA
	// TODO: CallFloatMethod
	// TODO: CallFloatMethodV
	// TODO: CallFloatMethodA
	// TODO: CallDoubleMethod
	// TODO: CallDoubleMethodV
	// TODO: CallDoubleMethodA
	// TODO: CallVoidMethod
	// TODO: CallVoidMethodV
	// TODO: CallVoidMethodA

	// --------------
	//   NON-VIRTUAL
	// --------------

	// TODO: CallNonvirtualObjectMethod
	// TODO: CallNonvirtualObjectMethodV
	// TODO: CallNonvirtualObjectMethodA
	// TODO: CallNonvirtualBooleanMethod
	// TODO: CallNonvirtualBooleanMethodV
	// TODO: CallNonvirtualBooleanMethodA
	// TODO: CallNonvirtualByteMethod
	// TODO: CallNonvirtualByteMethodV
	// TODO: CallNonvirtualByteMethodA
	// TODO: CallNonvirtualCharMethod
	// TODO: CallNonvirtualCharMethodV
	// TODO: CallNonvirtualCharMethodA
	// TODO: CallNonvirtualShortMethod
	// TODO: CallNonvirtualShortMethodV
	// TODO: CallNonvirtualShortMethodA
	// TODO: CallNonvirtualIntMethod
	// TODO: CallNonvirtualIntMethodV
	// TODO: CallNonvirtualIntMethodA
	// TODO: CallNonvirtualLongMethod
	// TODO: CallNonvirtualLongMethodV
	// TODO: CallNonvirtualLongMethodA
	// TODO: CallNonvirtualFloatMethod
	// TODO: CallNonvirtualFloatMethodV
	// TODO: CallNonvirtualFloatMethodA
	// TODO: CallNonvirtualDoubleMethod
	// TODO: CallNonvirtualDoubleMethodV
	// TODO: CallNonvirtualDoubleMethodA
	// TODO: CallNonvirtualVoidMethod
	// TODO: CallNonvirtualVoidMethodV
	// TODO: CallNonvirtualVoidMethodA

	// --------------
	//     STATIC
	// --------------

	/// Returns the method ID for a static method of a class.
	///
	/// This causes an uninitialized class to be initialized.
	///
	/// # Parameters
	///
	/// * `class`: The Java class object
	/// * `name`: The static method name
	/// * `sig`: The method signature
	///
	/// # Errors
	///
	/// This will error if an exception is thrown.
	///
	/// Possible exceptions:
	///
	/// * `NoSuchMethodError`: The specified static method cannot be found.
	/// * `ExceptionInInitializerError`: The class initializer fails due to an exception.
	/// * `OutOfMemoryError`: The system runs out of memory.
	pub fn get_static_method_id(
		&self,
		class: JClass,
		name: impl Into<JniString>,
		sig: impl Into<JniString>,
	) -> Result<JMethodId> {
		let name = name.into();
		let sig = sig.into();

		let ret;
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).GetStaticMethodID)(
				self.0 as _,
				class.raw(),
				name.as_cstr().as_ptr(),
				sig.as_cstr().as_ptr(),
			);
		}

		if self.exception_check() {
			return Err(JniError::ExceptionThrown);
		}

		// Native call should've thrown `NoSuchMethodError`
		assert!(!ret.is_null());
		Ok(unsafe { JMethodId::from_raw(ret) })
	}

	pub fn call_static_object_method(
		&self,
		class: JClass,
		method_id: JMethodId,
		args: impl IntoIterator<Item = impl Into<JValue>>,
	) -> Result<JObject> {
		let new_args = args
			.into_iter()
			.map(Into::into)
			.map(JValue::raw)
			.collect::<Vec<_>>();

		let ret;
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).CallStaticObjectMethodA)(
				self.0 as _,
				class.raw(),
				method_id.raw(),
				new_args.as_ptr(),
			);
		}

		if self.exception_check() {
			return Err(JniError::ExceptionThrown);
		}

		Ok(unsafe { JObject::from_raw(ret) })
	}

	// TODO: CallStaticObjectMethodV
	// TODO: CallStaticObjectMethodA
	// TODO: CallStaticBooleanMethod
	// TODO: CallStaticBooleanMethodV
	// TODO: CallStaticBooleanMethodA
	// TODO: CallStaticByteMethod
	// TODO: CallStaticByteMethodV
	// TODO: CallStaticByteMethodA
	// TODO: CallStaticCharMethod
	// TODO: CallStaticCharMethodV
	// TODO: CallStaticCharMethodA
	// TODO: CallStaticShortMethod
	// TODO: CallStaticShortMethodV
	// TODO: CallStaticShortMethodA
	// TODO: CallStaticIntMethod
	// TODO: CallStaticIntMethodV
	// TODO: CallStaticIntMethodA
	// TODO: CallStaticLongMethod
	// TODO: CallStaticLongMethodV
	// TODO: CallStaticLongMethodA
	// TODO: CallStaticFloatMethod
	// TODO: CallStaticFloatMethodV
	// TODO: CallStaticFloatMethodA
	// TODO: CallStaticDoubleMethod
	// TODO: CallStaticDoubleMethodV
	// TODO: CallStaticDoubleMethodA

	pub fn call_static_void_method(
		&self,
		class: JClass,
		method_id: JMethodId,
		args: impl IntoIterator<Item = impl Into<JValue>>,
	) -> Result<()> {
		let new_args = args
			.into_iter()
			.map(Into::into)
			.map(JValue::raw)
			.collect::<Vec<_>>();

		unsafe {
			let invoke_interface = self.as_native_interface();
			((*invoke_interface).CallStaticVoidMethodA)(
				self.0 as _,
				class.raw(),
				method_id.raw(),
				new_args.as_ptr(),
			);
		}

		if self.exception_check() {
			return Err(JniError::ExceptionThrown);
		}

		Ok(())
	}

	// TODO: CallStaticVoidMethodV
	// TODO: CallStaticVoidMethodA
}
