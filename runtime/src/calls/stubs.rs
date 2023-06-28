use crate::reference::MethodRef;

use ::jni::sys::*;

/// Used to generate the function pointer type in the `transmute` call
macro_rules! create_function_type {
	(@STATIC $($arg:ident),* $(-> $ret:ident)?) => {
		fn(env: ::jni::sys::JNIEnv, class: ::jni::sys::jclass, $($arg: $arg_ty),*) $(-> $ret)?
	};
	($($arg:ident),* $(-> $ret:ident)?) => {
		fn(env: ::jni::sys::JNIEnv, $($arg: $arg_ty),*) $(-> $ret)?
	};
}

#[rustfmt::skip]
macro_rules! type_mapping {
	(B) => { jbyte    };
	(Z) => { jboolean };
	(I) => { jint     };
	(J) => { jlong    };
	(F) => { jfloat   };
	(D) => { jdouble  };
	(L) => { jobject  };
}

/// Creates the call stubs for native methods
///
/// These functions cannot be used directly, they must be called through the `jcall!()` macro.
///
/// Example input:
///
/// ```rs
/// define_call_stubs! {
///     "()V": static fn();
///     "()J": fn() -> jlong;
/// }
/// ```
///
/// That will generate:
///
/// ```rs
/// fn stub_static_(method: MethodRef, args: ()) {
///     let code = core::mem::transmute::<*const core::ffi::c_void, fn(env: JNIEnv, class: jclass))>(method.native_method());
///     return code(env, args.0 /* for the class */);
/// }
///
/// fn stub__jlong(method: MethodRef, args: ()) {
///     let code = core::mem::transmute::<*const core::ffi::c_void, fn(env: JNIEnv, class: jclass) -> jlong)>(method.native_method());
///     return code(env);
/// }
/// ```
macro_rules! define_call_stubs {
	($descriptor:literal: static fn($($arg:ident),*) $(-> $ret:ident)? ; $($rest:tt)*) => {
		paste::paste! {
			fn [<stub_static_ $($arg)_* $(_ $ret)?>](method: MethodRef, env: JNIEnv, args: (jclass, $(type_mapping!($arg)),*)) $(-> $ret)? {
				let code = core::mem::transmute::<*const core::ffi::c_void, create_function_type!(@STATIC $($arg),* $(-> $ret)?)>(method.native_method());
				return code(env, args.0, $($arg),*);
			}
		}
		define_call_stubs!($($rest)*);
    };
	($descriptor:literal: fn($($arg:ident),*) $(-> $ret:ident)? ; $($rest:tt)*) => {
		paste::paste! {
			fn [<stub_ $($arg)_* $(_ $ret)?>](method: MethodRef, env: JNIEnv, args: ($(type_mapping!($arg)),*)) $(-> $ret)? {
				let code = core::mem::transmute::<*const core::ffi::c_void, create_function_type!($($arg),* $(-> $ret)?)>(method.native_method());
				return code(env, $($arg),*);
			}
		}
		define_call_stubs!($($rest)*);
	};
	() => {}
}

define_call_stubs! {
	"()V": static fn();
	"()B": static fn() -> jbyte;
	"()Z": static fn() -> jboolean;
	"()I": static fn() -> jint;
	"()J": static fn() -> jlong;
	"()F": static fn() -> jfloat;
	"()D": static fn() -> jdouble;
	"()L": static fn() -> jobject;

	"()V": fn();
	"()B": fn() -> jbyte;
	"()Z": fn() -> jboolean;
	"()I": fn() -> jint;
	"()J": fn() -> jlong;
	"()F": fn() -> jfloat;
	"()D": fn() -> jdouble;
	"()L": fn() -> jobject;
}
