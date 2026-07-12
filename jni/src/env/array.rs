use crate::error::{JniError, Result};
use crate::objects::{
	JBooleanArray, JByteArray, JCharArray, JClass, JDoubleArray, JFloatArray, JIntArray,
	JLongArray, JObject, JObjectArray, JShortArray,
};
use crate::sys::jsize;

use jni_sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort};

/// Generate all of the typed primitive array methods (`New<Type>Array`, `{Get,Set}<Type>ArrayRegion`).
macro_rules! define_primitive_array_methods {
    ($([$java_type:ident, $jni_wrapper_type:ty, $rust_component_type:ty]),* $(,)?) => {
        $(
        paste::paste! {
            pub fn [<new_ $java_type:lower _array>](&self, len: jsize) -> Result<$jni_wrapper_type> {
                let ret;
                unsafe {
                    let invoke_interface = self.as_native_interface();
                    ret = ((*invoke_interface).[<New $java_type:camel Array>])(self.0.cast::<jni_sys::JNIEnv>(), len);
                }

                if self.exception_check() {
                    return Err(JniError::ExceptionThrown);
                }

                if ret.is_null() {
                    return Err(JniError::Unknown);
                }

                Ok(unsafe { $jni_wrapper_type::from_raw(ret) })
            }

            /// Copy a region of a
            #[doc = " [`" $jni_wrapper_type "`] "]
            /// into `buf`.
            ///
            /// The size of the region is determined by the length of `buf`.
            ///
            /// # Parameters
            ///
            /// `array`: The array to slice.
            /// `start`: The starting index, must be greater than or equal to zero, and less than the array length (see [`Self::get_array_length()`]).
            /// `buf`: The destination buffer.
            ///
            /// # Errors
            ///
            /// This will error if an exception is thrown.
            ///
            /// Possible exceptions:
            ///
            /// * `ArrayIndexOutOfBoundsException`: The start index and/or buffer length is out of bounds.
            pub fn [<get_ $java_type:lower _array_region>](
                &self,
                array: $jni_wrapper_type,
                start: jsize,
                buf: &mut [$rust_component_type],
            ) -> Result<()> {
                unsafe {
                    let invoke_interface = self.as_native_interface();
                    ((*invoke_interface).[<Get $java_type:camel ArrayRegion>])(
                        self.0.cast::<jni_sys::JNIEnv>(),
                        array.raw(),
                        start,
                        buf.len() as jsize,
                        buf.as_mut_ptr(),
                    );
                }

                if self.exception_check() {
                    return Err(JniError::ExceptionThrown);
                }

                Ok(())
            }

            pub fn [<set_ $java_type:lower _array_region>](
                &self,
                array: $jni_wrapper_type,
                start: jsize,
                buf: &mut [$rust_component_type],
            ) -> Result<()> {
                unsafe {
                    let invoke_interface = self.as_native_interface();
                    ((*invoke_interface).[<Set $java_type:camel ArrayRegion>])(
                        self.0.cast::<jni_sys::JNIEnv>(),
                        array.raw(),
                        start,
                        buf.len() as jsize,
                        buf.as_mut_ptr(),
                    );
                }

                if self.exception_check() {
                    return Err(JniError::ExceptionThrown);
                }

                Ok(())
            }
        }
        )*
    }
}

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
				self.0.cast::<jni_sys::JNIEnv>(),
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
	// TODO: GetPrimitiveArrayCritical
	// TODO: ReleasePrimitiveArrayCritical

	define_primitive_array_methods! {
		[boolean, JBooleanArray, jboolean],
		[byte, JByteArray, jbyte],
		[char, JCharArray, jchar],
		[short, JShortArray, jshort],
		[int, JIntArray, jint],
		[long, JLongArray, jlong],
		[float, JFloatArray, jfloat],
		[double, JDoubleArray, jdouble]
	}
}
