#![allow(non_snake_case)]

//! Various offsets for fields of frequently accessed classes

macro_rules! get_sym {
	($specified_sym_name:ident $_fallback:ident) => {
		symbols::sym!($specified_sym_name)
	};
	($fallback:ident) => {
		symbols::sym!($fallback)
	};
}

// TODO: Document
macro_rules! field_module {
	(
	@CLASS $class_name:ident;
	$(
		$(#[$meta:meta])*
		$([sym: $specified_sym_name:ident])?
		@FIELD $field_name:ident: $matcher:pat $(if $guard:expr)?,
	)+
	$(
		mod $inner_mod:ident {
			$($inner_mod_tt:tt)*
		}
	)?
	) => {
		paste::paste! {
			$(
				static mut [<__ $field_name:snake:upper _FIELD_OFFSET>]: usize = 0;

				$(#[$meta])*
				/// This will not change for the lifetime of the program.
				pub fn [<$field_name _field_offset>]() -> usize {
					unsafe { [<__ $field_name:snake:upper _FIELD_OFFSET>] }
				}

				unsafe fn [<set_ $field_name _field_offset>](value: usize) {
					[<__ $field_name:snake:upper _FIELD_OFFSET>] = value;
				}
			)+

			/// Initialize the field offsets
			///
			/// # Safety
			///
			/// This **requires** that the target class is loaded
			pub unsafe fn init_offsets() {
				const EXPECTED_FIELD_SET: usize = (1 << ${count($field_name)}) - 1;
				let class = crate::globals::classes::$class_name();

				let mut field_set = 0;
				for field in class.fields() {
					$(
						if field.name == get_sym!($($specified_sym_name)? $field_name) && matches!(&field.descriptor, $matcher $(if $guard)?) {
							field_set |= 1 << ${index()};
							unsafe { [<set_ $field_name _field_offset>](field.idx); }
							continue;
						}
					)+
				}

				assert_eq!(field_set, EXPECTED_FIELD_SET, "Not all fields found in {}", stringify!($class_name));

				$(
					unsafe {
						$inner_mod::init_offsets();
					}
				)?
			}
		}

		$(
			pub mod $inner_mod {
				use super::*;

				field_module!(
					$($inner_mod_tt)*
				);
			}
		)?
	}
}

pub mod java_lang_ref_Reference {
	use classfile::FieldType;

	field_module! {
		@CLASS java_lang_ref_Reference;

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
	}
}

pub mod java_lang_String {
	use classfile::FieldType;

	field_module! {
		@CLASS java_lang_String;

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
	use classfile::FieldType;

	field_module! {
		@CLASS java_lang_Module;

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

		/// `java.lang.Thread#eetop` field offset
		///
		/// Expected type: `jlong`
		@FIELD eetop: FieldType::Long,
		/// `java.lang.Thread#holder` field offset
		///
		/// Expected type: `Reference` to `java.lang.Thread$FieldHolder`
		@FIELD holder: ty @ FieldType::Object(_) if ty.is_class(b"java/lang/Thread$FieldHolder"),

		mod holder {
			@CLASS java_lang_Thread_FieldHolder;

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
	use classfile::FieldType;

	field_module! {
		@CLASS java_lang_Throwable;

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
