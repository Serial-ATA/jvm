use crate::error::JniError;
use crate::objects::{JClass, JFieldId};
use crate::string::JniString;

impl super::JniEnv {
	// --------------
	//   NON-STATIC
	// --------------

	/// Returns the field ID for an instance (nonstatic) field of a class.
	///
	/// ## Notes
	///
	/// * This causes an uninitialized class to be initialized.
	/// * This **cannot** be used to obtain the length field of an array, see [`Self::get_array_length()`] instead.
	///
	/// # Parameters
	///
	/// * `class`: The Java class object
	/// * `name`: The field name
	/// * `sig`: The field signature
	///
	/// # Errors
	///
	/// This will error if an exception is thrown.
	///
	/// Possible exceptions:
	///
	/// * `NoSuchFieldError`: The specified field cannot be found.
	/// * `ExceptionInInitializerError`: The class initializer fails due to an exception.
	/// * `OutOfMemoryError`: The system runs out of memory.
	pub fn get_field_id(
		&self,
		class: JClass,
		name: impl Into<JniString>,
		sig: impl Into<JniString>,
	) -> crate::error::Result<JFieldId> {
		let name = name.into();
		let sig = sig.into();

		let ret;
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).GetFieldID)(
				self.0.cast::<jni_sys::JNIEnv>(),
				class.raw(),
				name.as_cstr().as_ptr(),
				sig.as_cstr().as_ptr(),
			);
		}

		if self.exception_check() {
			return Err(JniError::ExceptionThrown);
		}

		// Native call should've thrown `NoSuchFieldError`
		assert!(!ret.is_null());
		Ok(unsafe { JFieldId::from_raw(ret) })
	}
	// TODO: GetObjectField
	// TODO: GetBooleanField
	// TODO: GetByteField
	// TODO: GetCharField
	// TODO: GetShortField
	// TODO: GetIntField
	// TODO: GetLongField
	// TODO: GetFloatField
	// TODO: SetObjectField
	// TODO: SetBooleanField
	// TODO: SetByteField
	// TODO: SetCharField
	// TODO: SetShortField
	// TODO: SetIntField
	// TODO: SetLongField
	// TODO: SetFloatField
	// TODO: SetDoubleField

	// --------------
	//     STATIC
	// --------------

	// TODO: GetStaticFieldID
	// TODO: GetStaticObjectField
	// TODO: GetStaticBooleanField
	// TODO: GetStaticByteField
	// TODO: GetStaticCharField
	// TODO: GetStaticShortField
	// TODO: GetStaticIntField
	// TODO: GetStaticLongField
	// TODO: GetStaticFloatField
	// TODO: GetStaticDoubleField
	// TODO: SetStaticObjectField
	// TODO: SetStaticBooleanField
	// TODO: SetStaticByteField
	// TODO: SetStaticCharField
	// TODO: SetStaticShortField
	// TODO: SetStaticIntField
	// TODO: SetStaticLongField
	// TODO: SetStaticFloatField
	// TODO: SetStaticDoubleField
}
