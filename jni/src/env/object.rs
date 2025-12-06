use crate::error::{JniError, Result};
use crate::objects::{JClass, JMethodId, JObject, JValue};

impl super::JniEnv {
	// TODO: AllocObject
	// TODO: IsSameObject
	/// Constructs a new Java object.
	///
	/// The method ID indicates which constructor method to invoke. This ID must be obtained by calling [`Self::get_method_id()`] with `<init>` as the method name.
	///
	/// The class argument must not refer to an array class.
	///
	/// ## PARAMETERS
	///
	/// * `class`: a Java class object.
	/// * `methodID`: the method ID of the constructor.
	///
	/// ## THROWS
	///
	/// * `InstantiationException`: if the class is an interface or an abstract class.
	/// * `OutOfMemoryError`: if the system runs out of memory.
	/// * Any exceptions thrown by the constructor.
	pub fn new_object(
		&self,
		class: JClass,
		method: JMethodId,
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
			ret = ((*invoke_interface).NewObjectA)(
				self.0.cast::<jni_sys::JNIEnv>(),
				class.raw(),
				method.raw(),
				new_args.as_ptr(),
			);
		}

		if self.exception_check() {
			return Err(JniError::ExceptionThrown);
		}

		Ok(unsafe { JObject::from_raw(ret) })
	}

	// TODO: GetObjectClass
	/// Tests whether an object is an instance of a class.
	///
	/// ## PARAMETERS
	///
	/// `obj`: a Java object.
	/// `class`: a Java class object.
	pub fn is_instance_of(&self, obj: JObject, class: JClass) -> bool {
		let ret;
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).IsInstanceOf)(
				self.0.cast::<jni_sys::JNIEnv>(),
				obj.raw(),
				class.raw(),
			);
		}

		ret
	}
	// TODO: GetObjectRefType
}
