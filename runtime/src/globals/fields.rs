#![allow(non_snake_case)]

//! Various offsets for fields of frequently accessed classes

pub mod java_lang_Class {
	use classfile::FieldType;
	use symbols::sym;

	static mut CLASS_NAME_FIELD_OFFSET: usize = 0;

	pub fn name_field_offset() -> usize {
		unsafe { CLASS_NAME_FIELD_OFFSET }
	}

	unsafe fn set_name_field_offset(value: usize) {
		CLASS_NAME_FIELD_OFFSET = value;
	}

	/// Initialize the field offsets for `java.lang.Class`
	///
	/// # Safety
	///
	/// This **requires** that:
	/// * `java.lang.Class` is loaded
	pub unsafe fn init_offsets() {
		let class_class = crate::globals::classes::java_lang_Class();
		let class_name_field = class_class
			.fields()
			.find(|field| {
				!field.is_static()
					&& field.name == sym!(name)
					&& matches!(field.descriptor, FieldType::Object(ref val) if **val == *b"java/lang/String")
			})
			.expect("java.lang.Class should have a `name` field");

		unsafe {
			set_name_field_offset(class_name_field.idx);
		}
	}
}

pub mod java_lang_String {
	use classfile::FieldType;
	use symbols::sym;

	static mut STRING_VALUE_FIELD_OFFSET: usize = 0;
	static mut STRING_CODER_FIELD_OFFSET: usize = 0;

	/// `java.lang.String#value` field offset
	///
	/// This will not change for the lifetime of the program.
	///
	/// Expected type: `jByteArray`
	pub fn value_field_offset() -> usize {
		unsafe { STRING_VALUE_FIELD_OFFSET }
	}

	unsafe fn set_value_field_offset(value: usize) {
		unsafe { STRING_VALUE_FIELD_OFFSET = value }
	}

	/// `java.lang.String#coder` field offset
	///
	/// This will not change for the lifetime of the program.
	///
	/// Expected type: `jint`
	pub fn coder_field_offset() -> usize {
		unsafe { STRING_CODER_FIELD_OFFSET }
	}

	unsafe fn set_coder_field_offset(value: usize) {
		unsafe { STRING_CODER_FIELD_OFFSET = value }
	}

	/// Initialize the field offsets for `java.lang.String`
	///
	/// # Safety
	///
	/// This **requires** that:
	/// * `java.lang.String` is loaded
	pub unsafe fn init_offsets() {
		let string_class = crate::globals::classes::java_lang_String();

		let mut field_set = 0;
		for field in string_class.instance_fields() {
			if field.name == sym!(value)
				&& matches!(field.descriptor, FieldType::Array(ref val) if **val == FieldType::Byte)
			{
				field_set |= 1;
				unsafe {
					set_value_field_offset(field.idx);
				}
				continue;
			}

			if field.is_final()
				&& field.name == sym!(coder)
				&& matches!(field.descriptor, FieldType::Byte)
			{
				field_set |= 1 << 1;
				unsafe {
					set_coder_field_offset(field.idx);
				}
				continue;
			}
		}

		assert_eq!(field_set, 0b11, "Not all fields found in java/lang/String");
	}
}

pub mod java_lang_Module {
	use classfile::FieldType;
	use symbols::sym;

	static mut MODULE_NAME_FIELD_OFFSET: usize = 0;
	static mut MODULE_LOADER_FIELD_OFFSET: usize = 0;

	/// `java.lang.Module#name` field offset
	///
	/// This will not change for the lifetime of the program.
	///
	/// Expected type: `Reference` to `java.lang.String`
	pub fn name_field_offset() -> usize {
		unsafe { MODULE_NAME_FIELD_OFFSET }
	}

	unsafe fn set_name_field_offset(value: usize) {
		MODULE_NAME_FIELD_OFFSET = value;
	}

	/// `java.lang.Module#loader` field offset
	///
	/// This will not change for the lifetime of the program.
	///
	/// Expected type: `Reference` to `java.lang.ClassLoader`
	pub fn loader_field_offset() -> usize {
		unsafe { MODULE_LOADER_FIELD_OFFSET }
	}

	unsafe fn set_loader_field_offset(value: usize) {
		MODULE_LOADER_FIELD_OFFSET = value;
	}

	/// Initialize the field offsets for `java.lang.Module`
	///
	/// # Safety
	///
	/// This **requires** that:
	/// * `java.lang.Module` is loaded
	pub unsafe fn init_offsets() {
		let module_class = crate::globals::classes::java_lang_Module();

		let mut field_set = 0;
		for field in module_class.instance_fields() {
			if field.name == sym!(name)
				&& matches!(field.descriptor, FieldType::Object(ref val) if **val == *b"java/lang/String")
			{
				field_set |= 1;
				set_name_field_offset(field.idx);
				continue;
			}

			if field.name == sym!(loader)
				&& matches!(field.descriptor, FieldType::Object(ref val) if **val == *b"java/lang/ClassLoader")
			{
				field_set |= 1 << 1;
				set_loader_field_offset(field.idx);
				continue;
			}
		}

		assert_eq!(field_set, 0b11, "Not all fields found in java/lang/Module");
	}
}

pub mod java_lang_ref_Reference {
	use classfile::FieldType;
	use symbols::sym;

	static mut REFERENT_FIELD_OFFSET: usize = 0;

	/// `java.lang.ref.Reference#referent` field offset
	///
	/// This will not change for the lifetime of the program.
	///
	/// Expected type: `Reference`
	pub fn referent_field_offset() -> usize {
		unsafe { REFERENT_FIELD_OFFSET }
	}

	unsafe fn set_referent_field_offset(value: usize) {
		REFERENT_FIELD_OFFSET = value;
	}

	/// Initialize the field offsets for `java.lang.ref.Reference`
	///
	/// # Safety
	///
	/// This **requires** that:
	/// * `java.lang.ref.Reference` is loaded
	pub unsafe fn init_offsets() {
		let reference_class = crate::globals::classes::java_lang_ref_Reference();
		let reference_referent_field = reference_class
			.instance_fields()
			.find(|field| {
				field.name == sym!(referent) && matches!(field.descriptor, FieldType::Object(_))
			})
			.expect("java.lang.ref.Reference should have a `referent` field");

		unsafe {
			set_referent_field_offset(reference_referent_field.idx);
		}
	}
}

pub mod java_lang_Thread {
	use symbols::sym;

	static mut THREAD_EETOP_FIELD_OFFSET: usize = 0;
	static mut THREAD_HOLDER_FIELD_OFFSET: usize = 0;

	/// `java.lang.Thread#eetop` field offset
	///
	/// This will not change for the lifetime of the program.
	///
	/// Expected type: `jlong`
	pub fn eetop_field_offset() -> usize {
		unsafe { THREAD_EETOP_FIELD_OFFSET }
	}

	unsafe fn set_eetop_field_offset(value: usize) {
		THREAD_EETOP_FIELD_OFFSET = value;
	}

	/// `java.lang.Thread#holder` field offset
	///
	/// This will not change for the lifetime of the program.
	///
	/// Expected type: `java.lang.Thread$FieldHolder` reference
	pub fn holder_field_offset() -> usize {
		unsafe { THREAD_HOLDER_FIELD_OFFSET }
	}

	unsafe fn set_holder_field_offset(value: usize) {
		THREAD_HOLDER_FIELD_OFFSET = value;
	}

	/// Initialize the field offsets for `java.lang.Thread`
	///
	/// **NOTE: This also sets the offsets for java.lang.Thread$FieldHolder**
	///
	/// # Safety
	///
	/// This **requires** that:
	/// * `java.lang.Thread` is loaded
	/// * `java.lang.Thread$FieldHolder` is loaded
	pub unsafe fn init_offsets() {
		let class = crate::globals::classes::java_lang_Thread();

		let mut field_set = 0;
		for (index, field) in class.instance_fields().enumerate() {
			if field.name == sym!(holder) {
				unsafe {
					set_holder_field_offset(index);
				}

				field_set |= 1;
				continue;
			}

			if field.name == sym!(eetop) {
				unsafe {
					set_eetop_field_offset(index);
				}

				field_set |= 1 << 1;
				continue;
			}
		}

		assert_eq!(
			field_set, 0b11,
			"Not all fields were found in java/lang/Thread"
		);

		unsafe {
			holder::init_offsets();
		}
	}

	pub mod holder {
		static mut FIELDHOLDER_STACK_SIZE_FIELD_OFFSET: usize = 0;
		static mut FIELDHOLDER_PRIORITY_FIELD_OFFSET: usize = 0;
		static mut FIELDHOLDER_DAEMON_FIELD_OFFSET: usize = 0;
		static mut FIELDHOLDER_THREAD_STATUS_FIELD_OFFSET: usize = 0;

		/// `java.lang.Thread$FieldHolder#stackSize` field offset
		///
		/// This will not change for the lifetime of the program.
		///
		/// Expected field type: `jint`
		pub fn stack_size_field_offset() -> usize {
			unsafe { FIELDHOLDER_STACK_SIZE_FIELD_OFFSET }
		}

		unsafe fn set_stack_size_field_offset(value: usize) {
			FIELDHOLDER_STACK_SIZE_FIELD_OFFSET = value;
		}

		/// `java.lang.Thread$FieldHolder#priority` field offset
		///
		/// This will not change for the lifetime of the program.
		///
		/// Expected field type: `jint`
		pub fn priority_field_offset() -> usize {
			unsafe { FIELDHOLDER_PRIORITY_FIELD_OFFSET }
		}

		unsafe fn set_priority_field_offset(value: usize) {
			FIELDHOLDER_PRIORITY_FIELD_OFFSET = value;
		}

		/// `java.lang.Thread$FieldHolder#daemon` field offset
		///
		/// **THIS IS A STATIC FIELD**
		///
		/// This will not change for the lifetime of the program.
		///
		/// Expected field type: `jboolean`
		pub fn daemon_field_offset() -> usize {
			unsafe { FIELDHOLDER_DAEMON_FIELD_OFFSET }
		}

		unsafe fn set_daemon_field_offset(value: usize) {
			FIELDHOLDER_DAEMON_FIELD_OFFSET = value;
		}

		/// `java.lang.Thread$FieldHolder#threadStatus` field offset
		///
		/// **THIS IS A STATIC FIELD**
		///
		/// This will not change for the lifetime of the program.
		///
		/// Expected field type: `jint`
		pub fn thread_status_field_offset() -> usize {
			unsafe { FIELDHOLDER_THREAD_STATUS_FIELD_OFFSET }
		}

		unsafe fn set_thread_status_field_offset(value: usize) {
			FIELDHOLDER_THREAD_STATUS_FIELD_OFFSET = value;
		}

		/// Initialize the field offsets for `java.lang.Thread$FieldHolder`
		///
		/// # Safety
		///
		/// This **requires** that:
		/// * `java.lang.Thread$FieldHolder` is loaded
		pub(super) unsafe fn init_offsets() {
			let class = crate::globals::classes::java_lang_Thread_FieldHolder();

			let mut field_set = 0;
			for field in class.fields() {
				match field.name.as_str() {
					"stackSize" => {
						unsafe {
							set_stack_size_field_offset(field.idx);
						}
						field_set |= 1;
					},
					"priority" => {
						unsafe {
							set_priority_field_offset(field.idx);
						}
						field_set |= 1 << 1;
					},
					"daemon" => {
						unsafe {
							set_daemon_field_offset(field.idx);
						}
						field_set |= 1 << 2;
					},
					"threadStatus" => {
						unsafe {
							set_thread_status_field_offset(field.idx);
						}
						field_set |= 1 << 3;
					},
					_ => {},
				}
			}

			assert_eq!(
				field_set, 0b1111,
				"Not all fields were found in java/lang/Thread$FieldHolder"
			);
		}
	}
}

pub mod jdk_internal_misc_UnsafeConstants {
	use instructions::Operand;
	use jni::sys::jint;
	use symbols::sym;

	static mut ADDRESS_SIZE0_FIELD_OFFSET: usize = 0;
	static mut PAGE_SIZE_FIELD_OFFSET: usize = 0;
	static mut BIG_ENDIAN_FIELD_OFFSET: usize = 0;
	static mut UNALIGNED_ACCESS_FIELD_OFFSET: usize = 0;
	static mut DATA_CACHE_LINE_FLUSH_SIZE_FIELD_OFFSET: usize = 0;

	/// `jdk.internal.misc.UnsafeConstants#ADDRESS_SIZE0` field offset
	///
	/// This will not change for the lifetime of the program.
	///
	/// Expected field type: `jint`
	pub fn address_size0_field_offset() -> usize {
		unsafe { ADDRESS_SIZE0_FIELD_OFFSET }
	}

	unsafe fn set_address_size0_field_offset(value: usize) {
		ADDRESS_SIZE0_FIELD_OFFSET = value;
	}

	/// `jdk.internal.misc.UnsafeConstants#PAGE_SIZE` field offset
	///
	/// This will not change for the lifetime of the program.
	///
	/// Expected field type: `jint`
	pub fn page_size_field_offset() -> usize {
		unsafe { PAGE_SIZE_FIELD_OFFSET }
	}

	unsafe fn set_page_size_field_offset(value: usize) {
		PAGE_SIZE_FIELD_OFFSET = value;
	}

	/// `jdk.internal.misc.UnsafeConstants#BIG_ENDIAN` field offset
	///
	/// This will not change for the lifetime of the program.
	///
	/// Expected field type: `jboolean`
	pub fn big_endian_field_offset() -> usize {
		unsafe { BIG_ENDIAN_FIELD_OFFSET }
	}

	unsafe fn set_big_endian_field_offset(value: usize) {
		BIG_ENDIAN_FIELD_OFFSET = value;
	}

	/// `jdk.internal.misc.UnsafeConstants#UNALIGNED_ACCESS` field offset
	///
	/// This will not change for the lifetime of the program.
	///
	/// Expected field type: `jboolean`
	pub fn unaligned_access_field_offset() -> usize {
		unsafe { UNALIGNED_ACCESS_FIELD_OFFSET }
	}

	unsafe fn set_unaligned_access_field_offset(value: usize) {
		UNALIGNED_ACCESS_FIELD_OFFSET = value;
	}

	/// `jdk.internal.misc.UnsafeConstants#DATA_CACHE_LINE_FLUSH_SIZE` field offset
	///
	/// This will not change for the lifetime of the program.
	///
	/// Expected field type: `jint`
	pub fn data_cache_line_flush_size_field_offset() -> usize {
		unsafe { DATA_CACHE_LINE_FLUSH_SIZE_FIELD_OFFSET }
	}

	unsafe fn set_data_cache_line_flush_size_field_offset(value: usize) {
		DATA_CACHE_LINE_FLUSH_SIZE_FIELD_OFFSET = value;
	}

	/// Initialize the field offsets for `jdk.internal.misc.UnsafeConstants`
	///
	/// # Safety
	///
	/// This **requires** that:
	/// * `jdk.internal.misc.UnsafeConstants` is loaded
	pub unsafe fn init_offsets() {
		let class = crate::globals::classes::jdk_internal_misc_UnsafeConstants();

		let mut field_set = 0;
		for field in class.static_fields() {
			if field.name == sym!(ADDRESS_SIZE0) {
				field_set |= 1;
				unsafe {
					set_address_size0_field_offset(field.idx);
				}
				continue;
			}

			if field.name == sym!(PAGE_SIZE) {
				field_set |= 1 << 1;
				unsafe {
					set_page_size_field_offset(field.idx);
				}
				continue;
			}

			if field.name == sym!(BIG_ENDIAN) {
				field_set |= 1 << 2;
				unsafe {
					set_big_endian_field_offset(field.idx);
				}
				continue;
			}

			if field.name == sym!(UNALIGNED_ACCESS) {
				field_set |= 1 << 3;
				unsafe {
					set_unaligned_access_field_offset(field.idx);
				}
				continue;
			}

			if field.name == sym!(DATA_CACHE_LINE_FLUSH_SIZE) {
				field_set |= 1 << 4;
				unsafe {
					set_data_cache_line_flush_size_field_offset(field.idx);
				}
				continue;
			}
		}

		assert_eq!(
			field_set, 0b11111,
			"Not all fields were found in jdk/internal/misc/UnsafeConstants"
		);
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
			address_size0_field_offset(),
			Operand::from(size_of::<usize>() as jint),
		);
		class.set_static_field(
			page_size_field_offset(),
			Operand::from(platform::mem::get_page_size() as jint),
		);
		class.set_static_field(
			big_endian_field_offset(),
			Operand::from(cfg!(target_endian = "big") as jint),
		);
		// TODO: class.set_static_field(unaligned_access_field_offset(), /* ... */);
		// TODO: class.set_static_field(data_cache_line_flush_size_field_offset(), /* ... */);
	}
}
