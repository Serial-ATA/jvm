#![allow(non_snake_case)]

pub mod intrinsics;
pub mod jni;
pub mod lookup;

use crate::reference::Reference;
use crate::stack::local_stack::LocalStack;

use std::collections::HashMap;
use std::ptr::NonNull;
use std::sync::{LazyLock, RwLock};

use ::jni::env::JniEnv;
use common::traits::PtrType;
use instructions::Operand;
use symbols::Symbol;
use crate::method::Method;

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
pub struct NativeMethodDef {
	pub class: Symbol,
	pub name: Symbol,
	pub descriptor: Symbol,
}

#[macro_export]
macro_rules! include_generated {
	($path:literal) => {
		include!(std::concat!(
			env!("CARGO_MANIFEST_DIR"),
			"/../generated/",
			$path
		));
	};
}

pub type NativeReturn = Option<Operand<Reference>>;
pub type NativeMethodPtr = fn(NonNull<JniEnv>, LocalStack) -> NativeReturn;

include!("../../../generated/native/native_init.rs"); // Provides `init_native_method_table()`, generated by `runtime/build.rs`
pub(self) static NATIVE_METHOD_TABLE: LazyLock<RwLock<HashMap<NativeMethodDef, NativeMethodPtr>>> =
	LazyLock::new(|| RwLock::new(init_native_method_table()));

/// Lookup the native method definition for `method`
///
/// # Panics
///
/// This will panic if a definition is not found, see [`lookup_method_opt`].
pub fn lookup_method(method: &Method) -> NativeMethodPtr {
	lookup_method_opt(method).expect("native method should be present")
}

/// Lookup the native method defintion for `method`, or return `None`
pub fn lookup_method_opt(method: &Method) -> Option<NativeMethodPtr> {
	let native_method = NativeMethodDef {
		class: method.class.get().name,
		name: method.name,
		descriptor: method.descriptor,
	};

	NATIVE_METHOD_TABLE
		.read()
		.unwrap()
		.get(&native_method)
		.copied()
}

/// Insert a new method definition into the native method table
pub(self) fn insert_method((def, ptr): (NativeMethodDef, NativeMethodPtr)) {
	NATIVE_METHOD_TABLE.write().unwrap().insert(def, ptr);
}

// Module marker, do not remove
pub(crate) mod jdk {
	pub(crate) mod internal {
		pub(crate) mod misc {
			pub(crate) mod CDS;
			pub(crate) mod VM;
			pub(crate) mod Unsafe;
		}
		pub(crate) mod util {
		pub(crate) mod SystemProps;
		}
		pub(crate) mod loader {
		pub(crate) mod NativeLibraries;
		}
		pub(crate) mod reflect {
		pub(crate) mod Reflection;
		}
	}
}

pub(crate) mod java {
	pub(crate) mod lang {
		pub(crate) mod StringBuilder;
		pub(crate) mod Runtime;
		pub(crate) mod StringUTF16;
		pub(crate) mod System;
		pub(crate) mod Float;
		pub(crate) mod ClassLoader;
		pub(crate) mod Double;
		pub(crate) mod Throwable;
		pub(crate) mod Thread;
		pub(crate) mod Object;
		pub(crate) mod Class;
	}
	pub(crate) mod security {
	pub(crate) mod AccessController;
	}
}

