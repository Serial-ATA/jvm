use crate::error::{JniError, Result};
use crate::objects::{JClass, JObject, JObjectArray};
use crate::sys::jsize;

impl super::JniEnv {
	// TODO: GetArrayLength

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
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).NewObjectArray)(
				self.raw(),
				len,
				class.raw(),
				init.map_or(core::ptr::null_mut(), |init| init.raw()),
			);
		}

		if self.exception_check() {
			return Err(JniError::ExceptionThrown);
		}

		// Native call should've thrown `OutOfMemoryError`
		assert!(!ret.is_null());
		Ok(unsafe { JObjectArray::from_raw(ret) })
	}

	// TODO: GetObjectArrayElement

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
		unsafe {
			let invoke_interface = self.as_native_interface();
			((*invoke_interface).SetObjectArrayElement)(
				self.0 as _,
				array.raw(),
				index,
				val.map_or(core::ptr::null_mut(), |init| init.into().raw()),
			);
		}

		if self.exception_check() {
			return Err(JniError::ExceptionThrown);
		}

		Ok(())
	}

	// TODO: NewBooleanArray
	// TODO: NewByteArray
	// TODO: NewCharArray
	// TODO: NewShortArray
	// TODO: NewIntArray
	// TODO: NewLongArray
	// TODO: NewFloatArray
	// TODO: NewDoubleArray
	// TODO: GetBooleanArrayElements
	// TODO: GetByteArrayElements
	// TODO: GetCharArrayElements
	// TODO: GetShortArrayElements
	// TODO: GetIntArrayElements
	// TODO: GetLongArrayElements
	// TODO: GetFloatArrayElements
	// TODO: GetDoubleArrayElements
	// TODO: ReleaseBooleanArrayElements
	// TODO: ReleaseByteArrayElements
	// TODO: ReleaseCharArrayElements
	// TODO: ReleaseShortArrayElements
	// TODO: ReleaseIntArrayElements
	// TODO: ReleaseLongArrayElements
	// TODO: ReleaseFloatArrayElements
	// TODO: ReleaseDoubleArrayElements
	// TODO: GetBooleanArrayRegion
	// TODO: GetByteArrayRegion
	// TODO: GetCharArrayRegion
	// TODO: GetShortArrayRegion
	// TODO: GetIntArrayRegion
	// TODO: GetLongArrayRegion
	// TODO: GetFloatArrayRegion
	// TODO: GetDoubleArrayRegion
	// TODO: SetBooleanArrayRegion
	// TODO: SetByteArrayRegion
	// TODO: SetCharArrayRegion
	// TODO: SetShortArrayRegion
	// TODO: SetIntArrayRegion
	// TODO: SetLongArrayRegion
	// TODO: SetFloatArrayRegion
	// TODO: SetDoubleArrayRegion
	// TODO: GetPrimitiveArrayCritical
	// TODO: ReleasePrimitiveArrayCritical
}
