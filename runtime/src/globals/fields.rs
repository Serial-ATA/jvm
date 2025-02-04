#![allow(non_snake_case)]

//! Various offsets for fields of frequently accessed classes

#[allow(dead_code)] // This is used in the `field_constructor!` macro
const MAX_FIELD_COUNT: usize = 8;

macro_rules! get_sym {
	($specified_sym_name:ident $_fallback:ident) => {
		symbols::sym!($specified_sym_name)
	};
	($fallback:ident) => {
		symbols::sym!($fallback)
	};
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
		0 + instance_field_count!($($rest)*)
	};
	(
		$(#[$meta:meta])*
		$([sym: $specified_sym_name:ident])?
		@FIELD $field_name:ident: $matcher:pat $(if $guard:expr)?, $($rest:tt)*
	) => {
		1 + instance_field_count!($($rest)*)
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
		1 + injected_field_count!($($rest)*)
	};
	(
		$(#[$meta:meta])*
		$([sym: $specified_sym_name:ident])?
		@FIELD $field_name:ident: $matcher:pat $(if $guard:expr)?, $($rest:tt)*
	) => {
		0 + injected_field_count!($($rest)*)
	};
}

macro_rules! injected_field_definition {
	($class:ident, $($field_tt:tt)*) => {
		injected_field_definition!(@ACC [] $class, $($field_tt)*)
	};
	(
		@ACC [$($acc:tt)*] $class:ident,

		$(#[$meta:meta])*
		$([sym: $specified_sym_name:ident])?
		@INJECTED $field_name:ident: $descriptor:expr => $_field_ty:ty, $($rest:tt)*
	) => {
		injected_field_definition!(@ACC [$($acc)* crate::objects::field::Field::new_injected($class, get_sym!($($specified_sym_name)? $field_name), $descriptor), ] $class, $($rest)*)
	};
	(
		@ACC [$($acc:tt)*] $class:ident,

		$(#[$meta:meta])*
		$([sym: $specified_sym_name:ident])?
		@FIELD $field_name:ident: $matcher:pat $(if $guard:expr)?, $($rest:tt)*
	) => {
		injected_field_definition!(@ACC [$($acc)*] $class, $($rest)*)
	};
	(@ACC [$($acc:tt)*] $class:ident,) => { [$($acc)*] };
}

macro_rules! field_constructor {
	($class_name:ident $($sub_class_name:ident)? @FIELDSTART $($field_tt:tt)*) => {
		field_constructor!(@METHODS $($field_tt)*);

		/// Initialize the field offsets
		///
		/// # Safety
		///
		/// This **requires** that the target class is loaded
		pub unsafe fn init_offsets() {
			const INSTANCE_FIELD_COUNT: usize = instance_field_count!($($field_tt)*);
			const INJECTED_FIELD_COUNT: usize = injected_field_count!($($field_tt)*);
			const EXPECTED_FIELD_SET: usize = (1 << INSTANCE_FIELD_COUNT) - 1;
			const _: () = {
				assert!(INSTANCE_FIELD_COUNT + INJECTED_FIELD_COUNT <= crate::globals::fields::MAX_FIELD_COUNT);
			};

			let class = crate::globals::classes::$class_name();

			if INJECTED_FIELD_COUNT > 0 {
				class.inject_fields(
					injected_field_definition!(class, $($field_tt)*),
					INJECTED_FIELD_COUNT
				);
			}

			let mut field_set = 0;
			for field in class.fields() {
				field_constructor!(@CHECKS field, field_set, $($field_tt)*);
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
		field_constructor!(@METHODS
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

		field_constructor!(@METHODS $($rest)*);
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
			if $field_ident.name == get_sym!($($specified_sym_name)? $field_name) && matches!(&$field_ident.descriptor, $matcher $(if $guard)?) {
				$field_set_ident |= 1 << instance_field_count!($($rest)*);
				unsafe { [<set_ $field_name _field_offset>]($field_ident.index()); }
				continue;
			}
		}

		field_constructor!(@CHECKS $field_ident, $field_set_ident, $($rest)*);
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
			if $field_ident.name == get_sym!($($specified_sym_name)? $field_name) {
				unsafe { [<set_ $field_name _field_offset>]($field_ident.index()); }
				continue;
			}
		}

		field_constructor!(@CHECKS $field_ident, $field_set_ident, $($rest)*);
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
		field_constructor!($class_name $($sub_class_name)? @FIELDSTART $($field_tt)*);
	};
}

pub mod java_lang_ref_Reference {
	use classfile::FieldType;

	field_module! {
		@CLASS java_lang_ref_Reference;

		@FIELDSTART
		/// `java.lang.ref.Reference#referent` field offset
		///
		/// Expected type: `Reference`
		@FIELD referent: FieldType::Object(_),
	}
}

pub mod java_lang_Class {
	use classfile::FieldType;

	field_module! {
		@CLASS java_lang_Class;

		@FIELDSTART
		/// `java.lang.Class#name` field offset
		///
		/// Expected type: `Reference` to `java.lang.String`
		@FIELD name: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
		/// `java.lang.Class#module` field offset
		///
		/// Expected type: `Reference` to `java.lang.Module`
		@FIELD module: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Module"),
		/// `java.lang.Class#classLoader` field offset
		///
		/// Expected type: `Reference` to `java.lang.ClassLoader`
		@FIELD classLoader: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/ClassLoader"),
		/// `java.lang.Class#componentType` field offset
		///
		/// Expected type: `Reference` to `java.lang.Class`
		@FIELD componentType: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Class"),
	}
}

pub mod java_lang_ClassLoader {
	use crate::classpath::loader::ClassLoader;
	use crate::objects::instance::Instance;
	use crate::objects::reference::Reference;
	use classfile::FieldType;

	pub fn injected_loader_ptr_for(obj: Reference) -> Option<*const ClassLoader> {
		let ptr = obj
			.get_field_value0(loader_ptr_field_offset())
			.expect_long();
		if ptr == 0 {
			// Field not initialized yet.
			return None;
		}

		Some(ptr as *const ClassLoader)
	}

	field_module! {
		@CLASS java_lang_ClassLoader;

		@FIELDSTART
		/// [`ClassLoader`] pointer field
		///
		/// Expected type: `jlong`
		/// [`ClassLoader`]: crate::classpath::loader::ClassLoader
		@INJECTED loader_ptr: FieldType::Long => jni::sys::jlong,

		/// `java.lang.ClassLoader#name` field offset
		///
		/// Expected type: `Reference` to `java.lang.String`
		@FIELD name: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
		/// `java.lang.ClassLoader#unnamedModule` field offset
		///
		/// Expected type: `Reference` to `java.lang.Module`
		@FIELD unnamedModule: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Module"),
		/// `java.lang.ClassLoader#nameAndId` field offset
		///
		/// Expected type: `Reference` to `java.lang.String`
		@FIELD nameAndId: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
	}
}

pub mod java_lang_String {
	use classfile::FieldType;

	field_module! {
		@CLASS java_lang_String;

		@FIELDSTART
		/// `java.lang.String#value` field offset
		///
		/// Expected type: `jByteArray`
		@FIELD value: FieldType::Array(ref val) if **val == FieldType::Byte,
		/// `java.lang.String#coder` field offset
		///
		/// Expected type: `jint`
		@FIELD coder: FieldType::Byte,
	}
}

pub mod java_lang_Module {
	use crate::modules::Module;
	use crate::objects::instance::Instance;
	use crate::objects::reference::Reference;
	use classfile::FieldType;

	pub fn injected_module_ptr_for(obj: Reference) -> Option<*const Module> {
		let ptr = obj
			.get_field_value0(module_ptr_field_offset())
			.expect_long();
		if ptr == 0 {
			// Field not initialized yet.
			return None;
		}

		Some(ptr as *const Module)
	}

	field_module! {
		@CLASS java_lang_Module;

		@FIELDSTART
		/// [`Module`] pointer field
		///
		/// Expected type: `jlong`
		/// [`Module`]: crate::modules::Module
		@INJECTED module_ptr: FieldType::Long => jni::sys::jlong,

		/// `java.lang.Module#name` field offset
		///
		/// Expected type: `Reference` to `java.lang.String`
		@FIELD name: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
		/// `java.lang.Module#loader` field offset
		///
		/// Expected type: `Reference` to `java.lang.ClassLoader`
		@FIELD loader: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/ClassLoader"),
	}
}

pub mod java_lang_Thread {
	use classfile::FieldType;

	field_module! {
		@CLASS java_lang_Thread;
		@SUBCLASS holder;

		@FIELDSTART
		/// `java.lang.Thread#eetop` field offset
		///
		/// Expected type: `jlong`
		@FIELD eetop: FieldType::Long,
		/// `java.lang.Thread#holder` field offset
		///
		/// Expected type: `Reference` to `java.lang.Thread$FieldHolder`
		@FIELD holder: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Thread$FieldHolder"),
	}

	pub mod holder {
		use super::*;

		field_module! {
			@CLASS java_lang_Thread_FieldHolder;

			@FIELDSTART
			/// `java.lang.Thread$FieldHolder#stackSize` field offset
			///
			/// Expected field type: `jlong`
			@FIELD stackSize: FieldType::Long,
			/// `java.lang.Thread$FieldHolder#priority` field offset
			///
			/// Expected field type: `jint`
			@FIELD priority: FieldType::Int,
			/// `java.lang.Thread$FieldHolder#daemon` field offset
			///
			/// Expected field type: `jboolean`
			@FIELD daemon: FieldType::Boolean,
			/// `java.lang.Thread$FieldHolder#threadStatus` field offset
			///
			/// Expected field type: `jint`
			@FIELD threadStatus: FieldType::Int,
		}
	}
}

pub mod java_lang_StackTraceElement {
	use classfile::FieldType;

	field_module! {
		@CLASS java_lang_StackTraceElement;

		@FIELDSTART
		/// `java.lang.StackTraceElement#declaringClassObject` field offset
		///
		/// Expected field type: `Reference` to `java.lang.Class`
		@FIELD declaringClassObject: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Class"),
		/// `java.lang.StackTraceElement#classLoaderName` field offset
		///
		/// Expected field type: `Reference` to `java.lang.String`
		@FIELD classLoaderName: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
		/// `java.lang.StackTraceElement#moduleName` field offset
		///
		/// Expected field type: `Reference` to `java.lang.String`
		@FIELD moduleName: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
		/// `java.lang.StackTraceElement#moduleVersion` field offset
		///
		/// Expected field type: `Reference` to `java.lang.String`
		@FIELD moduleVersion: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
		/// `java.lang.StackTraceElement#declaringClass` field offset
		///
		/// Expected field type: `Reference` to `java.lang.String`
		@FIELD declaringClass: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
		/// `java.lang.StackTraceElement#methodName` field offset
		///
		/// Expected field type: `Reference` to `java.lang.String`
		@FIELD methodName: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
		/// `java.lang.StackTraceElement#fileName` field offset
		///
		/// Expected field type: `Reference` to `java.lang.String`
		@FIELD fileName: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
		/// `java.lang.StackTraceElement#lineNumber` field offset
		///
		/// Expected field type: `jint`
		@FIELD lineNumber: FieldType::Int,
	}
}

pub mod java_lang_Throwable {
	use crate::objects::class_instance::ClassInstance;
	use crate::objects::instance::Instance;
	use crate::objects::reference::{ClassInstanceRef, Reference};
	use classfile::FieldType;
	use instructions::Operand;

	/// `java.lang.Throwable#backtrace` field
	pub fn backtrace(instance: &ClassInstance) -> Reference {
		instance
			.get_field_value0(backtrace_field_offset())
			.expect_reference()
	}

	pub fn set_backtrace(instance: &mut ClassInstance, value: Reference) {
		instance.put_field_value0(backtrace_field_offset(), Operand::Reference(value))
	}

	field_module! {
		@CLASS java_lang_Throwable;

		@FIELDSTART
		/// `java.lang.Throwable#stackTrace` field offset
		///
		/// Expected field type: `Reference` to `StackTraceElement[]`
		@FIELD stackTrace: FieldType::Array(ref val) if val.is_class(b"java/lang/StackTraceElement"),
		/// `java.lang.Throwable#backtrace` field offset
		///
		/// Expected field type: `Reference` to `java.lang.Object`
		@FIELD backtrace: FieldType::Object(_),
		/// `java.lang.Throwable#depth` field offset
		///
		/// Expected field type: `jint`
		@FIELD depth: FieldType::Int,
	}
}

pub mod java_io_File {
	use classfile::FieldType;

	field_module! {
		@CLASS java_io_File;

		@FIELDSTART
		/// `java.io.File#path` field offset
		///
		/// Expected field type: `Reference` to `java.lang.String`
		@FIELD path: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
	}
}

pub mod java_io_FileDescriptor {
	use classfile::FieldType;

	#[cfg(unix)]
	field_module! {
		@CLASS java_io_FileDescriptor;

		@FIELDSTART
		/// `java.io.FileDescriptor#fd` field offset
		///
		/// Expected field type: `jint`
		@FIELD fd: FieldType::Int,
		/// `java.io.FileDescriptor#append` field offset
		///
		/// Expected field type: `jboolean`
		[sym: append_name] @FIELD append: FieldType::Boolean,
	}

	#[cfg(windows)]
	field_module! {
		@CLASS java_io_FileDescriptor;

		@FIELDSTART
		/// `java.io.FileDescriptor#fd` field offset
		///
		/// Expected field type: `jint`
		@FIELD fd: FieldType::Int,
		/// `java.io.FileDescriptor#handle` field offset
		///
		/// Expected field type: `jlong`
		@FIELD handle: FieldType::Long,
		/// `java.io.FileDescriptor#append` field offset
		///
		/// Expected field type: `jboolean`
		[sym: append_name] @FIELD append: FieldType::Boolean,
	}
}

pub mod java_io_FileInputStream {
	use classfile::FieldType;

	field_module! {
		@CLASS java_io_FileInputStream;

		@FIELDSTART
		/// `java.io.FileInputStream#fd` field offset
		///
		/// Expected field type: `Reference` to `java.io.FileDescriptor`
		@FIELD fd: ty @ FieldType::Object(_) if ty.is_class(b"java/io/FileDescriptor"),
	}
}

pub mod java_io_FileOutputStream {
	use classfile::FieldType;

	field_module! {
		@CLASS java_io_FileOutputStream;

		@FIELDSTART
		/// `java.io.FileOutputStream#fd` field offset
		///
		/// Expected field type: `Reference` to `java.io.FileDescriptor`
		@FIELD fd: ty @ FieldType::Object(_) if ty.is_class(b"java/io/FileDescriptor"),
	}
}

pub mod jdk_internal_misc_UnsafeConstants {
	use classfile::FieldType;
	use instructions::Operand;
	use jni::sys::jint;

	field_module! {
		@CLASS jdk_internal_misc_UnsafeConstants;

		@FIELDSTART
		/// `jdk.internal.misc.UnsafeConstants#ADDRESS_SIZE0` field offset
		///
		/// Expected field type: `jint`
		@FIELD ADDRESS_SIZE0: FieldType::Int,
		/// `jdk.internal.misc.UnsafeConstants#PAGE_SIZE` field offset
		///
		/// Expected field type: `jint`
		@FIELD PAGE_SIZE: FieldType::Int,
		/// `jdk.internal.misc.UnsafeConstants#BIG_ENDIAN` field offset
		///
		/// Expected field type: `jboolean`
		@FIELD BIG_ENDIAN: FieldType::Boolean,
		/// `jdk.internal.misc.UnsafeConstants#UNALIGNED_ACCESS` field offset
		///
		/// Expected field type: `jboolean`
		@FIELD UNALIGNED_ACCESS: FieldType::Boolean,
		/// `jdk.internal.misc.UnsafeConstants#DATA_CACHE_LINE_FLUSH_SIZE` field offset
		///
		/// Expected field type: `jint`
		@FIELD DATA_CACHE_LINE_FLUSH_SIZE: FieldType::Int,
	}

	/// Initialize the static fields for `jdk.internal.misc.UnsafeConstants`
	///
	/// # Safety
	///
	/// This **requires** that:
	/// * `jdk.internal.misc.UnsafeConstants` is loaded *and* initialized
	/// * all field offsets have been initialized
	pub unsafe fn init() {
		let class = crate::globals::classes::jdk_internal_misc_UnsafeConstants();

		// NOTE: The fields are already default initialized to 0
		class.set_static_field(
			ADDRESS_SIZE0_field_offset(),
			Operand::from(size_of::<usize>() as jint),
		);
		class.set_static_field(
			PAGE_SIZE_field_offset(),
			Operand::from(platform::mem::get_page_size() as jint),
		);
		class.set_static_field(
			BIG_ENDIAN_field_offset(),
			Operand::from(cfg!(target_endian = "big") as jint),
		);
		// TODO: class.set_static_field(unaligned_access_field_offset(), /* ... */);
		// TODO: class.set_static_field(data_cache_line_flush_size_field_offset(), /* ... */);
	}
}

pub mod java_lang_invoke_MemberName {
	use crate::objects::class_instance::ClassInstance;
	use crate::objects::instance::Instance;
	use crate::objects::reference::{MirrorInstanceRef, Reference};
	use classfile::FieldType;
	use instructions::Operand;
	use jni::sys::jint;

	/// `java.lang.invoke.MemberName#clazz` field
	pub fn clazz(instance: &ClassInstance) -> MirrorInstanceRef {
		instance
			.get_field_value0(clazz_field_offset())
			.expect_reference()
			.extract_mirror()
	}

	pub fn set_clazz(instance: &mut ClassInstance, value: Reference) {
		instance.put_field_value0(clazz_field_offset(), Operand::Reference(value))
	}

	/// `java.lang.invoke.MemberName#name` field
	pub fn name(instance: &ClassInstance) -> Reference {
		instance
			.get_field_value0(name_field_offset())
			.expect_reference()
	}

	pub fn set_name(instance: &mut ClassInstance, value: Reference) {
		instance.put_field_value0(name_field_offset(), Operand::Reference(value))
	}

	/// `java.lang.invoke.MemberName#type` field
	pub fn type_(instance: &ClassInstance) -> Reference {
		instance
			.get_field_value0(type_field_offset())
			.expect_reference()
	}

	pub fn set_type(instance: &mut ClassInstance, value: Reference) {
		instance.put_field_value0(type_field_offset(), Operand::Reference(value));
	}

	/// `java.lang.invoke.MemberName#flags` field
	pub fn flags(instance: &ClassInstance) -> jint {
		instance.get_field_value0(flags_field_offset()).expect_int()
	}

	pub fn set_flags(instance: &mut ClassInstance, value: jint) {
		instance.put_field_value0(flags_field_offset(), Operand::Int(value));
	}

	/// `java.lang.invoke.MemberName#method` field
	pub fn method(instance: &ClassInstance) -> Reference {
		instance
			.get_field_value0(flags_field_offset())
			.expect_reference()
	}

	pub fn set_method(instance: &mut ClassInstance, value: Reference) {
		instance.put_field_value0(method_field_offset(), Operand::Reference(value));
	}

	field_module! {
		@CLASS java_lang_invoke_MemberName;

		@FIELDSTART
		/// `java.lang.invoke.MemberName#clazz` field offset
		///
		/// Expected field type: `Reference` to `java.lang.Class`
		@FIELD clazz: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Class"),
		/// `java.lang.invoke.MemberName#name` field offset
		///
		/// Expected field type: `Reference` to `java.lang.String`
		@FIELD name: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/String"),
		/// `java.lang.invoke.MemberName#type` field offset
		///
		/// Expected field type: `Reference` to `java.lang.Object`
		@FIELD r#type: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Object"),
		/// `java.lang.invoke.MemberName#flags` field offset
		///
		/// Expected field type: jint
		@FIELD flags: FieldType::Int,
		/// `java.lang.invoke.MemberName#method` field offset
		///
		/// Expected field type: `Reference` to `java.lang.invoke.ResolvedMethodName`
		@FIELD method: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/invoke/ResolvedMethodName"),
	}
}

pub mod java_lang_invoke_MethodType {
	use classfile::FieldType;

	field_module! {
		@CLASS java_lang_invoke_MethodType;

		@FIELDSTART
		/// `java.lang.invoke.MethodType#ptypes` field offset
		///
		/// Expected field type: `Reference` to `java.lang.Class[]`
		@FIELD ptypes: FieldType::Array(ref val) if val.is_class(b"java/lang/Class"),
		/// `java.lang.invoke.MethodType#rtype` field offset
		///
		/// Expected field type: `Reference` to `java.lang.Class`
		@FIELD rtype: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Class"),
	}
}

pub mod java_lang_invoke_ResolvedMethodName {
	use classfile::FieldType;

	field_module! {
		@CLASS java_lang_invoke_ResolvedMethodName;

		@FIELDSTART
		/// `java.lang.invoke.ResolvedMethodName#vmholder` field offset
		///
		/// Expected field type: `Reference` to `java.lang.Class`
		@FIELD vmholder: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Class"),
	}
}
