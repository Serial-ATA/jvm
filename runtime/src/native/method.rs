use crate::objects::class::Class;
use crate::objects::method::Method;
use crate::objects::reference::Reference;
use crate::stack::local_stack::LocalStack;

use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{LazyLock, RwLock};

use ::jni::env::JniEnv;
use instructions::Operand;
use symbols::Symbol;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct NativeMethodDef {
	pub class: Symbol,
	pub name: Symbol,
	pub descriptor: Symbol,
	pub is_static: bool,
}

impl Debug for NativeMethodDef {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!(
			"{}#{} ({})",
			self.class.as_str(),
			self.name.as_str(),
			self.descriptor.as_str()
		))
	}
}

pub type NativeReturn = Option<Operand<Reference>>;
pub type NativeStaticMethodPtr = fn(JniEnv, &'static Class, LocalStack) -> NativeReturn;
pub type NativeNonStaticMethodPtr = fn(JniEnv, LocalStack) -> NativeReturn;

#[derive(Copy, Clone)]
union NativeMethodPtrInner {
	_static: NativeStaticMethodPtr,
	non_static: NativeNonStaticMethodPtr,
}

#[derive(Copy, Clone)]
pub struct NativeMethodPtr {
	inner: NativeMethodPtrInner,
}

impl NativeMethodPtr {
	/// Used by the native method generator, should never be used directly.
	#[doc(hidden)]
	pub fn new_static(f: NativeStaticMethodPtr) -> NativeMethodPtr {
		Self {
			inner: NativeMethodPtrInner { _static: f },
		}
	}
	/// Used by the native method generator, should never be used directly.
	#[doc(hidden)]
	pub fn new_non_static(f: NativeNonStaticMethodPtr) -> NativeMethodPtr {
		Self {
			inner: NativeMethodPtrInner { non_static: f },
		}
	}

	pub unsafe fn from_raw(ptr: *const ()) -> Self {
		Self {
			inner: unsafe { core::mem::transmute::<*const (), NativeMethodPtrInner>(ptr) },
		}
	}

	pub unsafe fn as_static(self) -> NativeStaticMethodPtr {
		unsafe { self.inner._static }
	}

	pub unsafe fn as_non_static(self) -> NativeNonStaticMethodPtr {
		unsafe { self.inner.non_static }
	}
}

include!("../../../generated/native/native_init.rs"); // Provides `init_native_method_table()`, generated by `runtime/build.rs`
pub(self) static NATIVE_METHOD_TABLE: LazyLock<RwLock<HashMap<NativeMethodDef, NativeMethodPtr>>> =
	LazyLock::new(|| RwLock::new(init_native_method_table()));

/// Lookup the native method definition for `method`
///
/// # Panics
///
/// This will panic if a definition is not found, see [`lookup_method_opt`].
pub fn lookup_method(method: &Method) -> NativeMethodPtr {
	let Some(method) = lookup_method_opt(method) else {
		panic!("Native method `{:?}` should be present", method)
	};

	method
}

/// Lookup the native method defintion for `method`, or return `None`
pub fn lookup_method_opt(method: &Method) -> Option<NativeMethodPtr> {
	let native_method = NativeMethodDef {
		class: method.class().name,
		name: method.name,
		descriptor: method.descriptor,
		is_static: method.is_static(),
	};

	NATIVE_METHOD_TABLE
		.read()
		.unwrap()
		.get(&native_method)
		.copied()
}

/// Insert a new method definition into the native method table
pub(super) fn insert_method((def, ptr): (NativeMethodDef, NativeMethodPtr)) {
	NATIVE_METHOD_TABLE.write().unwrap().insert(def, ptr);
}
