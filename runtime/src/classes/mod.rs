//! Utilities for interacting with and creating certain classes that are important to the VM

#![allow(non_snake_case)]

mod boxes;
pub mod java_io_File;
pub mod java_io_FileDescriptor;
pub mod java_io_FileInputStream;
pub mod java_io_FileOutputStream;
pub mod java_lang_Class;
pub mod java_lang_ClassLoader;
pub mod java_lang_Module;
pub mod java_lang_StackTraceElement;
pub mod java_lang_String;
pub mod java_lang_Thread;
pub mod java_lang_Throwable;
pub mod java_lang_invoke_LambdaForm;
pub mod java_lang_invoke_MemberName;
pub mod java_lang_invoke_MethodHandle;
pub mod java_lang_invoke_MethodType;
pub mod java_lang_invoke_ResolvedMethodName;
pub mod java_lang_ref_Reference;
pub mod java_lang_reflect_Constructor;
pub mod java_lang_reflect_Method;
pub mod jdk_internal_misc_UnsafeConstants;
pub mod jdk_internal_reflect_ConstantPool;

pub use boxes::*;

#[allow(dead_code)] // This is used in the `crate::classes::field_constructor!` macro
const MAX_FIELD_COUNT: usize = 11;

macro_rules! get_sym {
	($specified_sym_name:ident $_fallback:ident) => {{
		use crate::symbols::sym;
		sym!($specified_sym_name)
	}};
	($fallback:ident) => {{
		use crate::symbols::sym;
		sym!($fallback)
	}};
}

macro_rules! instance_field_count {
	() => {
		0
	};
	(
		$(#[$meta:meta])*
		$([sym: $specified_sym_name:ident])?
		@INJECTED $field_name:ident: $_descriptor:pat => $field_ty:ty, $($rest:tt)*
	) => {
		0 + crate::classes::instance_field_count!($($rest)*)
	};
	(
		$(#[$meta:meta])*
		$([sym: $specified_sym_name:ident])?
		@FIELD $field_name:ident: $matcher:pat $(if $guard:expr)?, $($rest:tt)*
	) => {
		1 + crate::classes::instance_field_count!($($rest)*)
	};
}

macro_rules! injected_field_count {
	() => {
		0
	};
	(
		$(#[$meta:meta])*
		$([sym: $specified_sym_name:ident])?
		@INJECTED $field_name:ident: $_descriptor:expr => $field_ty:ty, $($rest:tt)*
	) => {
		1 + crate::classes::injected_field_count!($($rest)*)
	};
	(
		$(#[$meta:meta])*
		$([sym: $specified_sym_name:ident])?
		@FIELD $field_name:ident: $matcher:pat $(if $guard:expr)?, $($rest:tt)*
	) => {
		0 + crate::classes::injected_field_count!($($rest)*)
	};
}

macro_rules! injected_field_definition {
	($class:ident, $($field_tt:tt)*) => {
		crate::classes::injected_field_definition!(@ACC [] $class, $($field_tt)*)
	};
	(
		@ACC [$($acc:tt)*] $class:ident,

		$(#[$meta:meta])*
		$([sym: $specified_sym_name:ident])?
		@INJECTED $field_name:ident: $descriptor:expr => $_field_ty:ty, $($rest:tt)*
	) => {
		crate::classes::injected_field_definition!(@ACC [$($acc)* crate::objects::field::Field::new_injected($class, crate::classes::get_sym!($($specified_sym_name)? $field_name), $descriptor), ] $class, $($rest)*)
	};
	(
		@ACC [$($acc:tt)*] $class:ident,

		$(#[$meta:meta])*
		$([sym: $specified_sym_name:ident])?
		@FIELD $field_name:ident: $matcher:pat $(if $guard:expr)?, $($rest:tt)*
	) => {
		crate::classes::injected_field_definition!(@ACC [$($acc)*] $class, $($rest)*)
	};
	(@ACC [$($acc:tt)*] $class:ident,) => { [$($acc)*] };
}

macro_rules! field_constructor {
	($class_name:ident $($sub_class_name:ident)? @FIELDSTART $($field_tt:tt)*) => {
		crate::classes::field_constructor!(@METHODS $($field_tt)*);

		/// Initialize the field offsets
        ///
        /// # Safety
        ///
        /// This **requires** that the target class is loaded
		pub unsafe fn init_offsets() {
			const INSTANCE_FIELD_COUNT: usize = crate::classes::instance_field_count!($($field_tt)*);
			const INJECTED_FIELD_COUNT: usize = crate::classes::injected_field_count!($($field_tt)*);
			const EXPECTED_FIELD_SET: usize = (1 << INSTANCE_FIELD_COUNT) - 1;
			const _: () = {
				assert!(INSTANCE_FIELD_COUNT + INJECTED_FIELD_COUNT <= crate::classes::MAX_FIELD_COUNT);
			};

			let class = crate::globals::classes::$class_name();

			if INJECTED_FIELD_COUNT > 0 {
				class.inject_fields(
					crate::classes::injected_field_definition!(class, $($field_tt)*),
					INJECTED_FIELD_COUNT
				);
			}

			let mut field_set = 0;
			for field in class.fields() {
				crate::classes::field_constructor!(@CHECKS field, field_set, $($field_tt)*);
			}

			assert_eq!(field_set, EXPECTED_FIELD_SET, "Not all fields found in {}", stringify!($class_name));

			$(
				unsafe {
					$sub_class_name::init_offsets();
				}
			)?
		}
	};
	(@METHODS
		$(#[$meta:meta])*
		$([sym: $specified_sym_name:ident])?
		@INJECTED $field_name:ident: $_descriptor:expr => $field_ty:ty, $($rest:tt)*
	) => {
		// Treat this field as a normal field
		crate::classes::field_constructor!(@METHODS
			$(#[$meta])*
			$([sym: $specified_sym_name])?
			@FIELD $field_name: _, $($rest)*
		);
	};
	(@METHODS
		$(#[$meta:meta])*
		$([sym: $specified_sym_name:ident])?
		@FIELD $field_name:ident: $matcher:pat $(if $guard:expr)?, $($rest:tt)*
	) => {
		paste::paste! {
			static mut [<__ $field_name:snake:upper _FIELD_OFFSET>]: usize = 0;

			$(#[$meta])*
			/// This will not change for the lifetime of the program.
			pub fn [<$field_name _field_offset>]() -> usize {
				unsafe { [<__ $field_name:snake:upper _FIELD_OFFSET>] }
			}

			unsafe fn [<set_ $field_name _field_offset>](value: usize) {
				[<__ $field_name:snake:upper _FIELD_OFFSET>] = value;
			}
		}

		crate::classes::field_constructor!(@METHODS $($rest)*);
	};
	(@METHODS) => {};

	(
		@CHECKS
		$field_ident:ident,
		$field_set_ident:ident,

		$(#[$meta:meta])*
		$([sym: $specified_sym_name:ident])?
		@FIELD $field_name:ident: $matcher:pat $(if $guard:expr)?, $($rest:tt)*
	) => {
		paste::paste! {
			if $field_ident.name == crate::classes::get_sym!($($specified_sym_name)? $field_name) && matches!(&$field_ident.descriptor, $matcher $(if $guard)?) {
				$field_set_ident |= 1 << crate::classes::instance_field_count!($($rest)*);
				unsafe { [<set_ $field_name _field_offset>]($field_ident.index()); }
				continue;
			}
		}

		crate::classes::field_constructor!(@CHECKS $field_ident, $field_set_ident, $($rest)*);
	};
	(
		@CHECKS
		$field_ident:ident,
		$field_set_ident:ident,

		$(#[$meta:meta])*
		$([sym: $specified_sym_name:ident])?
		@INJECTED $field_name:ident: $_descriptor:expr => $field_ty:ty, $($rest:tt)*
	) => {
		// Injected fields are not checked, in the field set, we only need to set their ids
		paste::paste! {
			if $field_ident.name == crate::classes::get_sym!($($specified_sym_name)? $field_name) {
				unsafe { [<set_ $field_name _field_offset>]($field_ident.index()); }
				continue;
			}
		}

		crate::classes::field_constructor!(@CHECKS $field_ident, $field_set_ident, $($rest)*);
	};
	(
		@CHECKS
		$field_ident:ident,
		$field_set_ident:ident,
	) => {};
}

// TODO: Document
macro_rules! field_module {
	(
	@CLASS $class_name:ident;
	$(@SUBCLASS $sub_class_name:ident;)?

	@FIELDSTART
	$($field_tt:tt)*
	) => {
		crate::classes::field_constructor!($class_name $($sub_class_name)? @FIELDSTART $($field_tt)*);
	};
}

pub(self) use {
	field_constructor, field_module, get_sym, injected_field_count, injected_field_definition,
	instance_field_count,
};
