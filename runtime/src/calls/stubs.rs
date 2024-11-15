use crate::reference::Reference;

use core::ffi::c_void;

use crate::native::NativeReturn;
use crate::stack::local_stack::LocalStack;
use ::jni::sys::*;
use instructions::Operand;

trait StubArguments<Args> {
	type Output;

	fn call_stub_with(&self, env: JNIEnv, args: Args) -> Self::Output;
}

trait StaticStubArguments<Args> {
	type Output;

	fn call_static_stub_with(&self, env: JNIEnv, class: jclass, args: Args) -> Self::Output;
}

impl<F, T, R> StubArguments<[T; 0]> for F
where
	F: Fn(::jni::sys::JNIEnv) -> R,
{
	type Output = R;

	fn call_stub_with(&self, env: ::jni::sys::JNIEnv, _args: [T; 0]) -> R {
		self(env)
	}
}

impl<F, T, R> StaticStubArguments<[T; 0]> for F
where
	F: Fn(JNIEnv, jclass) -> R,
{
	type Output = R;

	fn call_static_stub_with(&self, env: JNIEnv, class: jclass, _args: [T; 0]) -> R {
		self(env, class)
	}
}

macro_rules! implement_stub_arguments {
	($(($($ty:ident),+: $len:literal)),+ $(,)?) => {
		$(
			impl<F, T, R> StubArguments<[T; $len]> for F
			where
				F: Fn(::jni::sys::JNIEnv, $($ty),+) -> R,
			{
				type Output = R;

				fn call_stub_with(&self, env: ::jni::sys::JNIEnv, args: [T; $len]) -> R {
                    ::seq_macro::seq!(i in 0..$len {{
						// Destructure the args
                        let [ #(arg~i ,)* ] = args;

                        // Then provide the arguments to the function
                        self( env, #(arg~i ,)* )
                    }})
				}
			}

			impl<F, T, R> StaticStubArguments<[T; $len]> for F
			where
				F: Fn(JNIEnv, jclass, $($ty),+) -> R,
			{
				type Output = R;

				fn call_static_stub_with(&self, env: JNIEnv, class: jclass, args: [T; $len]) -> R {
					::seq_macro::seq!(i in 0..$len {{
						// Destructure the args
						let [ #(arg~i ,)* ] = args;

						// Then provide the arguments to the function
						self( env, class, #(arg~i ,)* )
					}})
				}
			}
		)+
	};
}

implement_stub_arguments! {
	(T: 1),
	(T, T: 2),
}

pub struct CallStub {
	signature: &'static str,
	func: *const c_void,
}

// impl CallStub {
// 	pub fn for_descriptor(descriptor: &str, is_static: bool) -> *const c_void {
// 		let list;
// 		if is_static {
// 			list = STATIC_CALL_STUBS;
// 		} else {
// 			list = CALL_STUBS;
// 		}
//
// 		for stub in list {
// 			if stub.signature == descriptor {
// 				return stub.func;
// 			}
// 		}
//
// 		unimplemented!(
// 			"No stub found for signature: {} (is_static: {})",
// 			descriptor,
// 			is_static
// 		);
// 	}
// }

/// Used to generate the function pointer type in the `transmute` call
macro_rules! create_function_type {
	(@STATIC $($arg:ident),* $(-> $ret:ident)?) => {
		fn(env: ::jni::sys::JNIEnv, class: ::jni::sys::jclass, $($arg: $arg_ty),*) $(-> $ret)?
	};
	($($arg:ident),* $(-> $ret:ident)?) => {
		fn(env: ::jni::sys::JNIEnv, $($arg: $arg_ty),*) $(-> $ret)?
	};
}

/// Used to generate argument array lengths (with are then passed to `StubArguments::call_stub_with()`)
macro_rules! count_arguments {
    () => {0usize};
    ($_head:ident $($tail:tt)*) => {1usize + count_arguments!($($tail)*)};
}

/// A function pointer to a native invoker
pub type NonStaticNativeInvokerPtr = fn(JNIEnv, LocalStack) -> NativeReturn;

/// Static variant of `NativeInvokerPtr`, needs a class pointer
pub type StaticNativeInvokerPtr = fn(JNIEnv, LocalStack) -> NativeReturn;

/// Holds both static and non-static native invoker pointers
///
/// It is very important that:
///
/// 1. A method be checked with `is_native()` + `is_static()`
/// 2. The method **has an invoker** in the first place
pub union NativeInvokerPtr {
	static_: StaticNativeInvokerPtr,
	non_static: NonStaticNativeInvokerPtr,
}

impl NativeInvokerPtr {
	/// Invoke the associated native static method
	///
	/// # Safety
	///
	/// This relies on the following **caller verified** conditions:
	///
	/// * There is a static invoker for the method
	/// * The static invoker is correctly defined according to [`StaticNativeInvokerPtr`]
	pub unsafe fn invoke_static(
		&self,
		env: JNIEnv,
		class: jclass,
		local_stack: LocalStack,
		ret: &mut Option<Operand<Reference>>,
	) {
		*ret = (self.static_)(env, /* class, */ local_stack);
	}

	/// Invoke the associated native method
	///
	/// # Safety
	///
	/// This relies on the following **caller verified** conditions:
	///
	/// * There is a non-static invoker for the method
	/// * The invoker is correctly defined according to [`NonStaticNativeInvokerPtr`]
	pub unsafe fn invoke(
		&self,
		env: JNIEnv,
		local_stack: LocalStack,
		ret: &mut Option<Operand<Reference>>,
	) {
		*ret = (self.non_static)(env, local_stack);
	}
}

/// Creates the call stubs for native methods
///
/// These functions cannot be used directly, they must be called through the `jcall!()` macro.
///
/// Example input:
///
/// ```rs
/// define_call_stubs! {
///     static {
///         "()V": static fn();
///     }
///
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
///     let code = core::mem::transmute::<*const core::ffi::c_void, fn(env: JNIEnv) -> jlong)>(method.native_method());
///     return code(env);
/// }
/// ```
macro_rules! define_call_stubs {
	(
		static {
			$( $descriptor:literal: static fn($($arg:ident),*) $(-> $ret:ident)? );+ $(;)?
		}
		$($rest:tt)*
	) => {
		paste::paste! {
			pub const STATIC_CALL_STUBS: &[CallStub] = &[
				$(CallStub {
					signature: $descriptor,
					func: [<stub_static_ $($arg)_* $(_ $ret)?>] as *const c_void
				}),+
			];
		}

		paste::paste! {
			$(
				unsafe fn [<stub_static_ $($arg)_* $(_ $ret)?>](
					method: $crate::reference::MethodRef,
					env: JNIEnv,
					class: jclass,
					args: Box<[::instructions::Operand<$crate::reference::Reference>]>,
					ret: &mut Option<::instructions::Operand<$crate::reference::Reference>>
				) $(-> $ret)? {
					let args = TryInto::<Box<[::instructions::Operand<$crate::reference::Reference>; count_arguments!($($arg)*)]>>::try_into(args).unwrap();
					let code = core::mem::transmute::<*const core::ffi::c_void, create_function_type!(@STATIC $($arg),* $(-> $ret)?)>(method.native_method());
					define_call_stubs!(@CALL [code.call_static_stub_with(env, class, Box::into_inner(args))]);
				}
			)+
		}

		define_call_stubs!($($rest)*);
    };
	($($descriptor:literal: fn($($arg:ident),*) $(-> $ret:ident)?);+ $(;)?) => {
		paste::paste! {
			pub const CALL_STUBS: &[CallStub] = &[
				$(CallStub {
					signature: $descriptor,
					func: [<stub_ $($arg)_* $(_ $ret)?>] as *const c_void
				}),+
			];
		}

		paste::paste! {
			$(
				unsafe fn [<stub_ $($arg)_* $(_ $ret)?>](
					method: $crate::reference::MethodRef,
					env: JNIEnv,
					args: Box<[::instructions::Operand<$crate::reference::Reference>]>,
					ret: &mut Option<::instructions::Operand<$crate::reference::Reference>>
				) $(-> $ret)? {
					let args = TryInto::<Box<[::instructions::Operand<$crate::reference::Reference>; count_arguments!($($arg)*)]>>::try_into(args).unwrap();
					let code = core::mem::transmute::<*const core::ffi::c_void, create_function_type!($($arg),* $(-> $ret)?)>(method.native_method());
					define_call_stubs!(@CALL [code.call_stub_with(env, Box::into_inner(args))]);
				}
			)+
		}
	};
	(@CALL [$($fn:tt)*] $ret:ident) => {
		*ret = Some(::instructions::Operand::from($($fn)*));
	};
	(@CALL [$($fn:tt)*]) => {
		return $($fn)*;
	};
	() => {}
}

// define_call_stubs! {
// 	static {
// 		"()V": static fn();
// 		"()B": static fn() -> jbyte;
// 		"()Z": static fn() -> jboolean;
// 		"()I": static fn() -> jint;
// 		"()J": static fn() -> jlong;
// 		"()F": static fn() -> jfloat;
// 		"()D": static fn() -> jdouble;
// 		"()L": static fn() -> jobject;
// 	}
//
// 	"()V": fn();
// 	"()B": fn() -> jbyte;
// 	"()Z": fn() -> jboolean;
// 	"()I": fn() -> jint;
// 	"()J": fn() -> jlong;
// 	"()F": fn() -> jfloat;
// 	"()D": fn() -> jdouble;
// 	"()L": fn() -> jobject;
// }
