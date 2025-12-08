use crate::env::JniEnv;
use crate::error::{JniError, Result};
use crate::objects::{
	JArray, JBooleanArray, JByteArray, JCharArray, JClass, JDoubleArray, JFloatArray, JIntArray,
	JLongArray, JObject, JObjectArray, JShortArray, JString, JThrowable, JWeak,
};
use crate::string::JniString;

use std::ffi::c_void;

use jni_sys::{jboolean, jbyte, jchar, jdouble, jfloat, jint, jlong, jshort};

pub struct JniNativeMethod {
	name: JniString,
	signature: JniString,
	fn_ptr: *mut c_void,
}

impl JniNativeMethod {
	pub fn from<F, N, S>(name: N, signature: S, fn_ptr: F) -> Self
	where
		F: JniMethod,
		N: Into<JniString>,
		S: Into<JniString>,
	{
		Self {
			name: name.into(),
			signature: signature.into(),
			fn_ptr: fn_ptr.raw(),
		}
	}

	fn raw(&self) -> jni_sys::JNINativeMethod {
		jni_sys::JNINativeMethod {
			name: self.name.as_cstr().as_ptr().cast_mut(),
			signature: self.signature.as_cstr().as_ptr().cast_mut(),
			fnPtr: self.fn_ptr.cast(),
		}
	}
}

pub trait JniMethod {
	fn raw(self) -> *mut c_void;
}

macro_rules! impl_jni_method {
    (
        $(($($params:ident),*)),* $(,)?
    ) => {
        $(
        impl<R, $($params),*> JniMethod for fn(JniEnv, JObject, $($params),*) -> R
        where
            R: JniMethodReturnType,
            $($params: JniMethodParameter),*
        {
            fn raw(self) -> *mut c_void {
                self as _
            }
        }

        impl<R, $($params),*> JniMethod for fn(JniEnv, JClass, $($params),*) -> R
        where
            R: JniMethodReturnType,
            $($params: JniMethodParameter),*
        {
            fn raw(self) -> *mut c_void {
                self as _
            }
        }
        )*
    }
}

impl_jni_method!(
	(),
	(T),
	(T, U),
	(T, U, V),
	(T, U, V, W),
	(T, U, V, W, X),
	(T, U, V, W, X, Y),
	(T, U, V, W, X, Y, Z),
	(T, U, V, W, X, Y, Z, A),
	(T, U, V, W, X, Y, Z, A, B),
	(T, U, V, W, X, Y, Z, A, B, C),
	(T, U, V, W, X, Y, Z, A, B, C, D),
	(T, U, V, W, X, Y, Z, A, B, C, D, E),
	(T, U, V, W, X, Y, Z, A, B, C, D, E, F),
	(T, U, V, W, X, Y, Z, A, B, C, D, E, F, G),
);

pub trait JniMethodParameter: sealed::Sealed {}
macro_rules! params {
    ($($ty:ident),* $(,)*) => {
        $(
            impl JniMethodParameter for $ty {}
            impl sealed::Sealed for $ty {}
        )*
    }
}

params!(
	jbyte,
	jchar,
	jdouble,
	jfloat,
	jint,
	jlong,
	jshort,
	jboolean,
	JObject,
	JClass,
	JThrowable,
	JString,
	JWeak,
	JArray,
	JBooleanArray,
	JByteArray,
	JCharArray,
	JShortArray,
	JIntArray,
	JLongArray,
	JFloatArray,
	JDoubleArray,
	JObjectArray,
);

pub trait JniMethodReturnType: sealed::Sealed {}

impl<T> JniMethodReturnType for T where T: JniMethodParameter {}

impl JniMethodReturnType for () {}

mod sealed {
	pub(super) trait Sealed {}
	impl Sealed for () {}
}

impl super::JniEnv {
	/// Registers native methods with the class specified by the `clazz` argument.
	///
	/// ## PARAMETERS
	///
	/// `clazz`: a Java class object.
	/// `methods`: the native methods in the class.
	///
	/// ## THROWS
	///
	/// `NoSuchMethodError`: if a specified method cannot be found or if the method is not native.
	pub fn register_natives(&self, clazz: JClass, methods: &[JniNativeMethod]) -> Result<()> {
		assert!(!self.raw().is_null());

		let methods = methods.iter().map(JniNativeMethod::raw).collect::<Vec<_>>();

		let ret;
		unsafe {
			let invoke_interface = self.as_native_interface();
			ret = ((*invoke_interface).RegisterNatives)(
				self.0.cast::<jni_sys::JNIEnv>(),
				clazz.raw(),
				methods.as_ptr(),
				methods.len() as jint,
			);
		}

		if self.exception_check() {
			return Err(JniError::ExceptionThrown);
		}

		if ret != 0 {
			return Err(JniError::Unknown);
		}

		Ok(())
	}
	// TODO: UnregisterNatives
}
