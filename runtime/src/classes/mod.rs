//! Utilities for interacting with and creating certain classes that are important to the VM

#![allow(non_snake_case)]

pub mod java;
pub mod jdk;

#[allow(dead_code)] // This is used in the `crate::classes::field_constructor!` macro
const MAX_FIELD_COUNT: usize = 11;

pub trait AsClassInstanceRef {
	fn as_class_instance_ref(&self) -> ClassInstanceRef;
}

impl AsClassInstanceRef for ClassInstanceRef {
	#[inline]
	fn as_class_instance_ref(&self) -> ClassInstanceRef {
		*self
	}
}

impl AsClassInstanceRef for Reference {
	#[inline]
	fn as_class_instance_ref(&self) -> ClassInstanceRef {
		self.extract_class()
	}
}

pub trait AsMirrorInstanceRef {
	fn as_mirror_instance_ref(&self) -> MirrorInstanceRef;
}

impl AsMirrorInstanceRef for MirrorInstanceRef {
	#[inline]
	fn as_mirror_instance_ref(&self) -> MirrorInstanceRef {
		*self
	}
}

impl AsMirrorInstanceRef for Reference {
	#[inline]
	fn as_mirror_instance_ref(&self) -> MirrorInstanceRef {
		self.extract_mirror()
	}
}

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
		@INJECTED $field_name:ident: $_descriptor:expr => $field_ty:ty, $($rest:tt)*
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

			crate::classes::field_constructor!(
				[enum FieldNames {}, [], 0]
				@FIELDS_ENUM $($field_tt)*
			);

			let class = crate::globals::classes::$class_name();

			if INJECTED_FIELD_COUNT > 0 {
				unsafe {
					class.inject_fields(
						crate::classes::injected_field_definition!(class, $($field_tt)*),
						INJECTED_FIELD_COUNT
					);
				}
			}

			let mut field_set = 0;

            #[allow(unused_variables)]
			for field in class.fields() {
				crate::classes::field_constructor!(@CHECKS field, field_set, 0, $($field_tt)*);
			}

			if field_set != EXPECTED_FIELD_SET {
				let missing = FieldNames::find_missing(field_set).into_iter().flatten().collect::<Vec<_>>();
				panic!("Not all fields found in {}, missing {missing:?}", stringify!($class_name))
			}

			$(
				unsafe {
					$sub_class_name::init_offsets();
				}
			)?
		}
	};

	(
		[
			enum FieldNames { $($existing_variants:tt)* },
			[$($variant_acc:tt)*],
			$current_shift:expr
		]

		@FIELDS_ENUM
		$(#[$meta:meta])*
		$([sym: $specified_sym_name:ident])?
		@INJECTED $field_name:ident: $_descriptor:expr => $field_ty:ty, $($rest:tt)*
	) => {
		// Ignore injected fields
		crate::classes::field_constructor!(
			[
				enum FieldNames { $($existing_variants)* },
				[$($variant_acc)*],
				$current_shift
			]

			@FIELDS_ENUM
			$($rest)*
		);
	};
	(
		[
			enum FieldNames { $($existing_variants:tt)* },
			[$($variant_acc:tt)*],
			$current_shift:expr
		]

        @FIELDS_ENUM
		$(#[$meta:meta])*
		$([sym: $specified_sym_name:ident])?
		@FIELD $field_name:ident: $matcher:pat $(if $guard:expr)?, $($rest:tt)*
	) => {
		crate::classes::field_constructor!(
			[
				enum FieldNames { $($existing_variants)* $field_name = 1usize << $current_shift, },
				[$($variant_acc)* FieldNames::$field_name,],
				($current_shift + 1)
			]

			@FIELDS_ENUM $($rest)*
		);
	};
    (
		[
			enum FieldNames { },
			[],
			$current_shift:expr
		]

		@FIELDS_ENUM) => {
		#[derive(Debug, Copy, Clone)]
		enum FieldNames {}

		impl FieldNames {
			const VARIANTS: [FieldNames; $current_shift] = [];

			fn find_missing(_found: usize) -> [Option<FieldNames>; $current_shift] {
				[]
			}
		}
	};
	(
		[
			enum FieldNames { $($existing_variants:tt)* },
			[$($variant_acc:tt)*],
			$current_shift:expr
		]

		@FIELDS_ENUM) => {
		#[allow(non_camel_case_types)]
		#[repr(usize)]
		#[derive(Debug, Copy, Clone)]
		enum FieldNames {
			$($existing_variants)*
		}

		impl FieldNames {
			const VARIANTS: [FieldNames; $current_shift] = [$($variant_acc)*];

			fn find_missing(found: usize) -> [Option<FieldNames>; $current_shift] {
				let mut missing = [None; $current_shift];

				for (index, variant) in Self::VARIANTS.into_iter().enumerate() {
					if found & variant as usize == 0 {
						missing[index] = Some(variant);
					}
				}

				missing
			}
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
			static mut [<__ $field_name:snake:upper _FIELD_INDEX>]: usize = 0;

			$(#[$meta])*
			/// This will not change for the lifetime of the program.
			pub fn [<$field_name _field_offset>]() -> usize {
				unsafe { [<__ $field_name:snake:upper _FIELD_OFFSET>] }
			}

			unsafe fn [<set_ $field_name _field_offset>](value: usize) {
				unsafe { [<__ $field_name:snake:upper _FIELD_OFFSET>] = value; }
			}

			$(#[$meta])*
			/// This will not change for the lifetime of the program.
			pub fn [<$field_name _field_index>]() -> usize {
				unsafe { [<__ $field_name:snake:upper _FIELD_INDEX>] }
			}

			unsafe fn [<set_ $field_name _field_index>](value: usize) {
				unsafe { [<__ $field_name:snake:upper _FIELD_INDEX>] = value; }
			}
		}

		crate::classes::field_constructor!(@METHODS $($rest)*);
	};
	(@METHODS) => {};

	(
		@CHECKS
		$field_ident:ident,
		$field_set_ident:ident,
		$current_shift:expr,

		$(#[$meta:meta])*
		$([sym: $specified_sym_name:ident])?
		@FIELD $field_name:ident: $matcher:pat $(if $guard:expr)?, $($rest:tt)*
	) => {
		paste::paste! {
			if $field_ident.name == crate::classes::get_sym!($($specified_sym_name)? $field_name) && matches!(&$field_ident.descriptor, $matcher $(if $guard)?) {
				$field_set_ident |= 1 << $current_shift;
				unsafe { [<set_ $field_name _field_offset>]($field_ident.offset()); }
				unsafe { [<set_ $field_name _field_index>]($field_ident.index()); }
				continue;
			}
		}

		crate::classes::field_constructor!(@CHECKS $field_ident, $field_set_ident, ($current_shift + 1), $($rest)*);
	};
	(
		@CHECKS
		$field_ident:ident,
		$field_set_ident:ident,
		$current_shift:expr,

		$(#[$meta:meta])*
		$([sym: $specified_sym_name:ident])?
		@INJECTED $field_name:ident: $_descriptor:expr => $field_ty:ty, $($rest:tt)*
	) => {
		// Injected fields are not checked, in the field set, we only need to set their ids
		paste::paste! {
			if $field_ident.name == crate::classes::get_sym!($($specified_sym_name)? $field_name) {
				unsafe { [<set_ $field_name _field_offset>]($field_ident.offset()); }
				unsafe { [<set_ $field_name _field_index>]($field_ident.index()); }
				continue;
			}
		}

		crate::classes::field_constructor!(@CHECKS $field_ident, $field_set_ident, $current_shift, $($rest)*);
	};
	(
		@CHECKS
		$field_ident:ident,
		$field_set_ident:ident,
		$current_shift:expr,
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

use crate::objects::instance::class::ClassInstanceRef;
use crate::objects::instance::mirror::MirrorInstanceRef;
use crate::objects::reference::Reference;
pub(self) use {
	field_constructor, field_module, get_sym, injected_field_count, injected_field_definition,
	instance_field_count,
};
