use crate::error::{JniError, Result};
use crate::objects::{JClass, JMethodId, JObject, JObjectArray, JString, JValue};
use crate::string::JCesu8String;
use crate::version::JniVersion;
use jni_sys::{jsize, JNI_TRUE};

/// Safer wrapper around `jni_sys::JNIEnv`
#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct JniEnv(jni_sys::JNIEnv);

impl JniEnv {
	/// Returns the version of the native method interface.
	///
	/// # Errors
	///
	/// If an invalid version is returned, for some reason.
	pub fn get_version(&self) -> Result<JniVersion> {
		let ret;
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).GetVersion)(self.0 as _);
		}

		JniVersion::from_raw(ret).ok_or(JniError::Unknown)
	}

	pub fn define_class(
		&self,
		_name: impl Into<JCesu8String>,
		_loader: JObject,
		_buf: &[u8],
	) -> JClass {
		unimplemented!("JNIEnv::define_class");
	}

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
	pub fn find_class(&self, name: impl Into<JCesu8String>) -> Result<JClass> {
		let name = name.into();

		let ret;
		let exception;
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).FindClass)(self.0 as _, name.as_cstr().as_ptr());

			exception = ((*invoke_interface).ExceptionCheck)(self.0 as _);
		}

		if exception {
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
		let ret;
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).GetSuperclass)(self.0 as _, sub.raw());
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
		let ret;
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).IsAssignableFrom)(self.0 as _, sub.raw(), sup.raw());
		}

		ret == JNI_TRUE
	}

	/// Returns the method ID for a static method of a class.
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
		name: impl Into<JCesu8String>,
		sig: impl Into<JCesu8String>,
	) -> Result<JMethodId> {
		let name = name.into();
		let sig = sig.into();

		let ret;
		let exception;
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).GetStaticMethodID)(
				self.0 as _,
				class.raw(),
				name.as_cstr().as_ptr(),
				sig.as_cstr().as_ptr(),
			);

			exception = ((*invoke_interface).ExceptionCheck)(self.0 as _);
		}

		if exception {
			return Err(JniError::ExceptionThrown);
		}

		// Native call should've thrown `NoSuchMethodError`
		assert!(!ret.is_null());
		Ok(unsafe { JMethodId::from_raw(ret) })
	}

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

		let exception;
		unsafe {
			let invoke_interface = self.as_native_interface();
			((*invoke_interface).CallStaticVoidMethod)(
				self.0 as _,
				class.raw(),
				method_id.raw(),
				new_args.as_ptr(),
			);

			exception = ((*invoke_interface).ExceptionCheck)(self.0 as _);
		}

		if exception {
			return Err(JniError::ExceptionThrown);
		}

		Ok(())
	}

	/// Construct a new `java.lang.String` from a modified UTF-8 string.
	///
	/// # Parameters
	///
	/// * `utf`: The modified UTF-8 string.
	///
	/// # Errors
	///
	/// This will error if an exception is thrown.
	///
	/// Possible exceptions:
	///
	/// * `OutOfMemoryError`: The system runs out of memory.
	pub fn new_string_utf(&self, utf: impl Into<JCesu8String>) -> Result<JString> {
		let utf = utf.into();

		let ret;
		let exception;
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).NewStringUTF)(self.0 as _, utf.as_cstr().as_ptr());

			exception = ((*invoke_interface).ExceptionCheck)(self.0 as _);
		}

		if exception {
			return Err(JniError::ExceptionThrown);
		}

		if ret.is_null() {
			return Err(JniError::Unknown);
		}

		Ok(unsafe { JString::from_raw(ret) })
	}

	/// Constructs a new array of `class` objects.
	///
	/// # Parameters
	///
	/// * `length`: The array size.
	/// * `class`: The array element class.
	/// * `init`: The initialization value, or `None` for null.
	///
	/// # Errors
	///
	/// This will error if an exception is thrown.
	///
	/// Possible exceptions:
	///
	/// * `OutOfMemoryError`: The system runs out of memory.
	pub fn new_object_array(
		&self,
		len: jsize,
		class: JClass,
		init: Option<JObject>,
	) -> Result<JObjectArray> {
		let ret;
		let exception;
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).NewObjectArray)(
				self.0 as _,
				len,
				class.raw(),
				init.map_or(core::ptr::null_mut(), |init| init.raw()),
			);

			exception = ((*invoke_interface).ExceptionCheck)(self.0 as _);
		}

		if exception {
			return Err(JniError::ExceptionThrown);
		}

		// Native call should've thrown `OutOfMemoryError`
		assert!(!ret.is_null());
		Ok(unsafe { JObjectArray::from_raw(ret) })
	}

	/// Set an element at a position in an object array
	///
	/// ## PARAMETERS
	///
	/// `array`: The Java object array.
	/// `index`: The index to change.
	/// `value`: The new value to set at `index`, or `None` for null.
	///
	/// # Errors
	///
	/// This will error if an exception is thrown.
	///
	/// Possible exceptions:
	///
	/// * `ArrayIndexOutOfBoundsException`: The `index` does not specify a valid index in the array.
	/// * `ArrayStoreException`: The class of value is not a subclass of the element class of the array.
	pub fn set_object_array_element(
		&self,
		array: JObjectArray,
		index: jsize,
		val: Option<impl Into<JObject>>,
	) -> Result<()> {
		let exception;
		unsafe {
			let invoke_interface = self.as_native_interface();
			((*invoke_interface).SetObjectArrayElement)(
				self.0 as _,
				array.raw(),
				index,
				val.map_or(core::ptr::null_mut(), |init| init.into().raw()),
			);

			exception = ((*invoke_interface).ExceptionCheck)(self.0 as _);
		}

		if exception {
			return Err(JniError::ExceptionThrown);
		}

		Ok(())
	}
}

impl JniEnv {
	pub fn raw(&self) -> jni_sys::JNIEnv {
		self.0
	}

	pub unsafe fn from_raw(env: jni_sys::JNIEnv) -> Self {
		Self(env)
	}

	unsafe fn as_native_interface(&self) -> *const jni_sys::JNINativeInterface_ {
		self.0 as _
	}
}
